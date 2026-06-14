use rustines_macro::Named;

use crate::arch::mappers::mapper::Mapper;
use crate::arch::memory::FetchStore;
use crate::utils::named::Named;

#[derive(Named, Default)]
pub struct Mapper0 {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    prg_ram: Vec<u8>,
}

impl Mapper0 {
    pub fn new(prg_rom: Vec<u8>, chr_rom: Vec<u8>) -> Self {
        Mapper0 {
            prg_rom,
            chr_rom,
            prg_ram: vec![0; 0x2000],
        }
    }
}

impl Mapper for Mapper0 {}

impl FetchStore for Mapper0 {
    fn fetch(&self, addr: u16) -> u8 {
        match addr {
            // CHR ROM (PPU Bus)
            0x0000..=0x1FFF => {
                if !self.chr_rom.is_empty() {
                    self.chr_rom[addr as usize]
                } else {
                    0
                }
            }
            // PRG RAM
            0x6000..=0x7FFF => {
                if !self.prg_ram.is_empty() {
                    self.prg_ram[(addr - 0x6000) as usize]
                } else {
                    0
                }
            }
            // PRG ROM
            0x8000..=0xFFFF => {
                let mut mapped_addr = (addr - 0x8000) as usize;
                if self.prg_rom.len() == 0x4000 {
                    // Mirror 16KB
                    mapped_addr &= 0x3FFF;
                }
                self.prg_rom[mapped_addr]
            }
            _ => 0,
        }
    }

    fn store(&mut self, addr: u16, val: u8) -> u8 {
        match addr {
            0x6000..=0x7FFF => {
                let old = self.prg_ram[(addr - 0x6000) as usize];
                self.prg_ram[(addr - 0x6000) as usize] = val;
                old
            }
            _ => 0, // PRG ROM is not writable, CHR ROM usually isn't either in NROM
        }
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
