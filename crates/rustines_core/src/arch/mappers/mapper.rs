use crate::{arch::rom_structs::MirroringType, utils::named::Named};

pub trait Mapper: Named {
    fn reset(&mut self) {}
    fn fetch_cpu(&self, addr: u16) -> u8;
    fn store_cpu(&mut self, addr: u16, val: u8);

    fn fetch_ppu(&self, addr: u16) -> u8;
    fn store_ppu(&mut self, addr: u16, val: u8);

    fn mirroring_mode(&self) -> MirroringType;

    fn irq_active(&self) -> bool {
        false
    }

    fn get_prg_ram(&self) -> Option<&[u8]> {
        None
    }

    fn set_prg_ram(&mut self, _content: &[u8]) {}
}

pub type MapperBox = Box<dyn Mapper>;
