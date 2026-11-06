use crate::arch::rom_structs::{CHR_ROM_BANK_SIZE, PRG_RAM_BANK_SIZE, PRG_ROM_BANK_SIZE};

pub type RomBank<const N: usize> = [u8; N];
pub type PrgRomBank = RomBank<PRG_ROM_BANK_SIZE>;
pub type PrgRamBank = RomBank<PRG_RAM_BANK_SIZE>;
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

#[cfg(test)]
pub(crate) mod test_utils {
    use crate::arch::rom_structs::{
        CHR_ROM_BANK_SIZE, HeaderFlags, INesHeader, PRG_ROM_BANK_SIZE, TRAINER_SIZE,
    };

    pub const FLAG_VERTICAL: u16 = 1 << 0;
    pub const FLAG_TRAINER: u16 = 1 << 2;

    pub fn header(
        prg_rom_banks: u8,
        chr_rom_banks: u8,
        prg_ram_banks: u8,
        flags: u16,
    ) -> INesHeader {
        INesHeader {
            header: [0; 4],
            flags: HeaderFlags(flags),
            chr_rom_banks,
            flags_9: 0,
            flags_10: 0,
            prg_rom_banks,
            prg_ram_banks,
            _padding: [0; 5],
        }
    }

    /// Expected PRG byte: the high nibble identifies the 16KB bank, the low nibble the offset.
    pub fn prg_byte(bank: usize, offset: usize) -> u8 {
        ((bank << 4) | (offset & 0x0F)) as u8
    }

    /// Expected CHR byte for an 8KB bank and an offset in `0..0x2000`. Each 4KB half has a
    /// distinct value, so 4KB banking can be verified too.
    pub fn chr_byte(bank: usize, offset: usize) -> u8 {
        let bank_4kb = bank * 2 + offset / 0x1000;
        (0x80 ^ (bank_4kb << 3) ^ (offset & 0x07)) as u8
    }

    /// Builds a ROM (without the iNES header) with recognizable PRG and CHR patterns, optionally
    /// preceded by a trainer filled with garbage.
    pub fn build_rom(prg_banks: usize, chr_banks: usize, trainer: bool) -> Vec<u8> {
        let mut rom = Vec::new();
        if trainer {
            rom.extend(std::iter::repeat_n(0xEE, TRAINER_SIZE));
        }
        for bank in 0..prg_banks {
            rom.extend((0..PRG_ROM_BANK_SIZE).map(|i| prg_byte(bank, i)));
        }
        for bank in 0..chr_banks {
            rom.extend((0..CHR_ROM_BANK_SIZE).map(|i| chr_byte(bank, i)));
        }
        rom
    }
}
