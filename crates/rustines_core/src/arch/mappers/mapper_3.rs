use anyhow::bail;
use rustines_macro::Named;

use crate::arch::mappers::mapper::Mapper;
use crate::arch::rom_structs::{CHR_ROM_BANK_SIZE, INesHeader, MirroringType, TRAINER_SIZE};
use crate::utils::named::Named;

#[derive(Named)]
pub struct Mapper3 {
    prg_rom: Vec<u8>,
    prg_ram: Option<Vec<u8>>,
    chr_rom: Vec<u8>,
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
            bail!("Invalid number of banks, expected 2, found {banks}")
        }

        let prg_rom = rom.drain(0..header.prg_rom_size()).collect();

        if header.uses_chr_ram() {
            bail!("Unexpected CHR RAM for mapper 0");
        }
        let chr_rom = rom.drain(0..header.chr_rom_size()).collect();

        let prg_ram = if header.uses_prg_ram() {
            Some(vec![0; header.prg_ram_size()])
        } else {
            None
        };

        Ok(Mapper3 {
            prg_rom,
            chr_rom,
            prg_ram,
            mirroring_type: header.mirroring_type(),
            selected_bank: 0,
            bank_mask: header.chr_rom_banks - 1,
        })
    }
}

impl Mapper for Mapper3 {
    fn prg_rom(&self) -> &[u8] {
        &self.prg_rom
    }

    fn chr_rom(&self) -> &[u8] {
        &self.chr_rom
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
                let addr = (addr - 0x8000) as usize;
                self.prg_rom[addr]
            }
            _ => panic!("Invalid value provided to fetch_cpu: {addr}"),
        }
    }

    fn fetch_ppu(&self, addr: u16) -> u8 {
        if addr <= 0x1FFF {
            let base = (self.selected_bank as usize) * CHR_ROM_BANK_SIZE;
            self.chr_rom[base + addr as usize]
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
