use rustines_macro::Named;

use crate::arch::mappers::common::{ChrRomBank, PrgRomBank, get_bank_coordinates, get_rom_banks};
use crate::arch::mappers::mapper::Mapper;
use crate::arch::rom_structs::{CHR_ROM_BANK_SIZE, INesHeader, MirroringType, TRAINER_SIZE};
use crate::utils::named::Named;
const CHR_RAM_SIZE: usize = 8192;

#[derive(Named)]
pub struct Mapper2 {
    prg_rom_banks: Vec<PrgRomBank>,
    chr_rom: Option<Vec<ChrRomBank>>,
    chr_ram: Option<[u8; CHR_RAM_SIZE]>,
    mirroring_type: MirroringType,
    selected_bank: u8,
    bank_mask: u8,
}

impl Mapper2 {
    pub fn new(header: &INesHeader, mut rom: Vec<u8>) -> anyhow::Result<Self> {
        if header.has_trainer() {
            let _ = rom.drain(0..TRAINER_SIZE).collect::<Vec<_>>();
        }

        let prg_rom_banks = get_rom_banks(&mut rom, header.prg_rom_size());

        let (chr_rom, chr_ram) = if header.uses_chr_ram() {
            (None, Some([0; CHR_RAM_SIZE]))
        } else {
            (Some(get_rom_banks(&mut rom, header.chr_rom_size())), None)
        };

        Ok(Mapper2 {
            prg_rom_banks,
            chr_rom,
            chr_ram,
            mirroring_type: header.mirroring_type(),
            selected_bank: 0,
            bank_mask: header.prg_rom_banks - 1,
        })
    }
}

impl Mapper for Mapper2 {
    fn reset(&mut self) {
        self.selected_bank = 0;
    }

    fn fetch_cpu(&self, addr: u16) -> u8 {
        match addr {
            0x6000..=0x7FFF => 0,
            0x8000..=0xBFFF => {
                let addr = (addr - 0x8000) as usize;
                self.prg_rom_banks[self.selected_bank as usize][addr]
            }
            0xC000..=0xFFFF => {
                let addr = (addr - 0xC000) as usize;
                self.prg_rom_banks[self.prg_rom_banks.len() - 1][addr]
            }
            _ => panic!("Invalid value provided to fetch_cpu: {addr}"),
        }
    }

    fn fetch_ppu(&self, addr: u16) -> u8 {
        if addr <= 0x1FFF {
            if let Some(chr_ram) = self.chr_ram {
                chr_ram[addr as usize]
            } else {
                let (index, offset) = get_bank_coordinates(addr, CHR_ROM_BANK_SIZE as u16);
                debug_assert_eq!(index, 0);
                self.chr_rom.as_ref().unwrap()[index as usize][offset as usize]
            }
        } else {
            0
        }
    }

    fn store_ppu(&mut self, addr: u16, val: u8) {
        if addr <= 0x1FFF
            && let Some(chr_ram) = self.chr_ram.as_mut()
        {
            chr_ram[addr as usize] = val;
        }
    }

    fn store_cpu(&mut self, _addr: u16, val: u8) {
        self.selected_bank = val & self.bank_mask;
    }

    fn mirroring_mode(&self) -> MirroringType {
        self.mirroring_type
    }
}
