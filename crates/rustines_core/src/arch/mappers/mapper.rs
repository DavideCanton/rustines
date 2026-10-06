use crate::{arch::rom_structs::MirroringType, utils::named::Named};

pub trait Mapper: Named {
    fn prg_rom(&self) -> &[u8];
    fn chr_rom(&self) -> &[u8];

    fn fetch_cpu(&self, addr: u16) -> u8;
    fn store_cpu(&mut self, addr: u16, val: u8);

    fn fetch_ppu(&self, addr: u16) -> u8;
    fn store_ppu(&mut self, addr: u16, val: u8);

    fn mirroring_mode(&self) -> MirroringType;
    fn has_prg_ram(&self) -> bool {
        false
    }

    fn irq_active(&self) -> bool {
        false
    }
}

pub type MapperBox = Box<dyn Mapper>;
