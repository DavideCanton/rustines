use anyhow::bail;
use rustines_macro::Named;

use crate::arch::mappers::mapper::Mapper;
use crate::arch::rom_structs::{INesHeader, TRAINER_SIZE};
use crate::utils::named::Named;

#[derive(Named, Default)]
pub struct Mapper0 {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    banks: usize,
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

        Ok(Mapper0 {
            prg_rom,
            chr_rom,
            banks,
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
        // TODO move outside?
        let mut addr = addr - 0x8000;

        if self.banks == 1 {
            addr &= 0x3FFF;
        }

        self.prg_rom[addr as usize]
    }

    fn store_prg_rom(&mut self, _addr: u16, _val: u8) -> u8 {
        // Mapper 0 doesn't write
        0
    }

    fn fetch_chr_rom(&self, addr: u16) -> u8 {
        if addr <= 0x1FFF {
            self.chr_rom[addr as usize]
        } else {
            0
        }
    }

    fn store_chr_rom(&mut self, _addr: u16, _val: u8) -> u8 {
        // Mapper 0 doesn't write
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Mapper0;
    use super::Named;

    // testing the procedural macro here since it's easier
    #[test]
    fn test_named() {
        let m = Mapper0::default();
        assert_eq!(m.name(), "Mapper0");
    }
}
