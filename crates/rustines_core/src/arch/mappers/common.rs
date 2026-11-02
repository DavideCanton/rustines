use crate::arch::rom_structs::{CHR_ROM_BANK_SIZE, PRG_ROM_BANK_SIZE};

pub type RomBank<const N: usize> = [u8; N];
pub type PrgRomBank = RomBank<PRG_ROM_BANK_SIZE>;
pub type ChrRomBank = RomBank<CHR_ROM_BANK_SIZE>;

pub fn get_rom_banks<const N: usize>(rom: &mut Vec<u8>, size: usize) -> Vec<RomBank<N>> {
    rom.drain(0..size)
        .collect::<Vec<u8>>()
        .as_chunks::<N>()
        .0
        .to_vec()
}

#[inline(always)]
pub fn get_bank_coordinates(address: u16, size: u16) -> (u16, u16) {
    let index = (address & !(size - 1)) / size;
    let offset = address & (size - 1);
    (index, offset)
}
