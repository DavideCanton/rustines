use anyhow::bail;
use rustines_macro::Named;

use crate::arch::mappers::common::{ChrRomBank, PrgRomBank, get_bank_coordinates, get_rom_banks};
use crate::arch::mappers::mapper::Mapper;
use crate::arch::rom_structs::{INesHeader, MirroringType, PRG_ROM_BANK_SIZE, TRAINER_SIZE};
use crate::utils::named::Named;

#[derive(Named)]
pub struct Mapper3 {
    prg_rom: Vec<PrgRomBank>,
    prg_ram: Option<Vec<u8>>,
    chr_rom_banks: Vec<ChrRomBank>,
    mirroring_type: MirroringType,
    selected_bank: u8,
    bank_mask: u8,
}

impl Mapper3 {
    pub fn new(header: &INesHeader, mut rom: Vec<u8>) -> anyhow::Result<Self> {
        if header.has_trainer() {
            let _ = rom.drain(0..TRAINER_SIZE).collect::<Vec<_>>();
        }

        let banks = header.prg_rom_banks();
        if banks != 2 {
            bail!("Invalid number of PRG banks, expected 2, found {banks}")
        }

        let prg_rom = get_rom_banks(&mut rom, header.prg_rom_size());

        if header.uses_chr_ram() {
            bail!("Unexpected CHR RAM for mapper 3");
        }
        let chr_rom_banks = get_rom_banks(&mut rom, header.chr_rom_size());

        let prg_ram = if header.uses_prg_ram() {
            Some(vec![0; header.prg_ram_size()])
        } else {
            None
        };

        Ok(Mapper3 {
            prg_rom,
            chr_rom_banks,
            prg_ram,
            mirroring_type: header.mirroring_type(),
            selected_bank: 0,
            bank_mask: header.chr_rom_banks - 1,
        })
    }
}

impl Mapper for Mapper3 {
    fn reset(&mut self) {
        self.selected_bank = 0;
    }

    fn fetch_cpu(&self, addr: u16) -> u8 {
        match addr {
            0x6000..=0x7FFF => {
                if let Some(prg_ram) = self.prg_ram.as_ref() {
                    let addr = (addr & 0x7FF) as usize;
                    prg_ram[addr]
                } else {
                    0
                }
            }
            0x8000..=0xFFFF => {
                let (index, offset) = get_bank_coordinates(addr - 0x8000, PRG_ROM_BANK_SIZE as u16);
                self.prg_rom[index as usize][offset as usize]
            }
            _ => panic!("Invalid value provided to fetch_cpu: {addr}"),
        }
    }

    fn fetch_ppu(&self, addr: u16) -> u8 {
        if addr <= 0x1FFF {
            self.chr_rom_banks[self.selected_bank as usize][addr as usize]
        } else {
            0
        }
    }

    fn store_ppu(&mut self, _addr: u16, _val: u8) {}

    fn store_cpu(&mut self, _addr: u16, val: u8) {
        self.selected_bank = val & self.bank_mask;
    }

    fn mirroring_mode(&self) -> MirroringType {
        self.mirroring_type
    }
}
