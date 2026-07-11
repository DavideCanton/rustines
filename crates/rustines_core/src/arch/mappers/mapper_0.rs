use anyhow::bail;
use rustines_macro::Named;

use crate::arch::mappers::mapper::Mapper;
use crate::arch::rom_structs::{INesHeader, MirroringType, TRAINER_SIZE};
use crate::utils::named::Named;

const CHR_RAM_SIZE: usize = 8192;

#[derive(Named)]
pub struct Mapper0 {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    banks: usize,
    chr_ram: Option<[u8; CHR_RAM_SIZE]>,
    mirroring_type: MirroringType,
}

impl Mapper0 {
    pub fn new(header: &INesHeader, mut rom: Vec<u8>) -> anyhow::Result<Self> {
        if header.has_trainer() {
            let _ = rom.drain(0..TRAINER_SIZE).collect::<Vec<_>>();
        }

        let banks = header.prg_rom_banks();
        if banks != 1 && banks != 2 {
            bail!("Invalid number of banks, expected 1 or 2, found {banks}")
        }

        let prg_rom = rom.drain(0..header.prg_rom_size()).collect();
        let chr_rom = if header.uses_chr_ram() {
            vec![]
        } else {
            rom.drain(0..header.chr_rom_size()).collect()
        };

        let chr_ram = if header.uses_chr_ram() {
            Some([0; CHR_RAM_SIZE])
        } else {
            None
        };

        Ok(Mapper0 {
            prg_rom,
            chr_rom,
            banks,
            chr_ram,
            mirroring_type: header.mirroring_type(),
        })
    }
}

impl Mapper for Mapper0 {
    fn prg_rom(&self) -> &[u8] {
        &self.prg_rom
    }

    fn chr_rom(&self) -> &[u8] {
        &self.chr_rom
    }

    fn fetch_prg_rom(&self, addr: u16) -> u8 {
        let mut addr = addr - 0x8000;

        if self.banks == 1 {
            addr &= 0x3FFF;
        }

        self.prg_rom[addr as usize]
    }

    fn fetch_chr_rom(&self, addr: u16) -> u8 {
        if addr <= 0x1FFF {
            if let Some(chr_ram) = self.chr_ram {
                chr_ram[addr as usize]
            } else {
                self.chr_rom[addr as usize]
            }
        } else {
            0
        }
    }

    fn store_chr_rom(&mut self, addr: u16, val: u8) -> u8 {
        if addr <= 0x1FFF
            && let Some(chr_ram) = self.chr_ram.as_mut()
        {
            let addr = addr as usize;
            let old = chr_ram[addr];
            chr_ram[addr] = val;
            old
        } else {
            0
        }
    }

    fn fetch_prg_ram(&self, _addr: u16) -> u8 {
        0
    }

    fn store_prg_ram(&mut self, _addr: u16, _val: u8) -> u8 {
        0
    }

    fn store_prg_rom(&mut self, _addr: u16, _val: u8) -> u8 {
        0
    }

    fn mirroring_mode(&self) -> MirroringType {
        self.mirroring_type
    }
}
