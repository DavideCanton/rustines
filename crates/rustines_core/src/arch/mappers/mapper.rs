use crate::utils::named::Named;

pub trait Mapper: Named {
    fn prg_rom(&self) -> &[u8];
    fn chr_rom(&self) -> &[u8];
    fn fetch_prg_rom(&self, addr: u16) -> u8;
    fn fetch_chr_rom(&self, addr: u16) -> u8;
    fn store_chr_ram(&mut self, addr: u16, val: u8) -> u8;
    fn fetch_prg_ram(&self, _addr: u16) -> u8 {
        0
    }
}

pub type MapperBox = Box<dyn Mapper>;
