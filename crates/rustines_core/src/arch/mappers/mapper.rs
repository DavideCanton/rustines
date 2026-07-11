use crate::{arch::rom_structs::MirroringType, utils::named::Named};

pub trait Mapper: Named {
    fn prg_rom(&self) -> &[u8];
    fn chr_rom(&self) -> &[u8];

    fn fetch_prg_rom(&self, addr: u16) -> u8;
    fn store_prg_rom(&mut self, addr: u16, val: u8) -> u8;

    fn fetch_chr_rom(&self, addr: u16) -> u8;
    fn store_chr_rom(&mut self, addr: u16, val: u8) -> u8;

    fn fetch_prg_ram(&self, addr: u16) -> u8;
    fn store_prg_ram(&mut self, addr: u16, val: u8) -> u8;

    fn mirroring_mode(&self) -> MirroringType;
}

pub type MapperBox = Box<dyn Mapper>;
