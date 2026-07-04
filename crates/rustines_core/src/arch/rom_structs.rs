use std::mem;

use crate::arch::mappers::mapper::MapperBox;

#[repr(C, packed)]
#[derive(Debug)]
pub struct INesHeader {
    pub header: [u8; 4],
    pub prg_rom_size: u8,
    pub chr_rom_size: u8,
    pub flags_6: u8,
    pub flags_7: u8,
    pub prg_ram_size: u8,
    pub flags_9: u8,
    pub flags_10: u8,
    pub padding: [u8; 5],
}

#[derive(Eq, PartialEq, Debug)]
pub enum MirroringType {
    Horizontal,
    Vertical,
}

macro_rules! extract_flag {
    ( $val:expr, $offset:expr ) => {{ ($val & (1 << $offset)) > 0 }};
}

pub const PRG_ROM_BANK_SIZE: usize = 1 << 14;
pub const CHR_ROM_BANK_SIZE: usize = 1 << 13;

pub const TRAINER_SIZE: usize = 1 << 9;

pub const HEADER: &[u8; 4] = b"NES\x1A";

impl INesHeader {
    pub fn from_bytes(buf: &[u8; 16]) -> Self {
        unsafe { mem::transmute_copy(buf) }
    }

    pub fn prg_rom_size(&self) -> usize {
        (self.prg_rom_size as usize) * PRG_ROM_BANK_SIZE
    }

    pub fn prg_rom_banks(&self) -> usize {
        self.prg_rom_size as usize
    }

    pub fn chr_rom_size(&self) -> usize {
        (self.chr_rom_size as usize) * CHR_ROM_BANK_SIZE
    }

    pub fn uses_chr_ram(&self) -> bool {
        self.chr_rom_size == 0
    }

    pub fn mirroring_type(&self) -> MirroringType {
        if extract_flag!(self.flags_6, 0) {
            MirroringType::Vertical
        } else {
            MirroringType::Horizontal
        }
    }

    pub fn has_other_memory(&self) -> bool {
        extract_flag!(self.flags_6, 1)
    }

    pub fn has_trainer(&self) -> bool {
        extract_flag!(self.flags_6, 2)
    }

    pub fn ignore_mirroring(&self) -> bool {
        extract_flag!(self.flags_6, 3)
    }

    pub fn mapping_number(&self) -> u8 {
        let low = self.flags_6 >> 4;
        let high = self.flags_7 & 0xF0;
        high | low
    }
}

pub struct NesRom {
    pub header: INesHeader,
    pub mapper: MapperBox,
}

impl NesRom {
    pub fn new(header: INesHeader, mapper: MapperBox) -> Self {
        NesRom { header, mapper }
    }
}

#[cfg(test)]
mod tests {
    use crate::arch::rom_structs::MirroringType;

    use super::{CHR_ROM_BANK_SIZE, PRG_ROM_BANK_SIZE};
    use test_case::test_case;

    use super::INesHeader;

    const DEFAULT: [u8; 16] = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    #[test]
    fn test_header() {
        let header = INesHeader::from_bytes(&DEFAULT);
        assert_eq!(header.header, [1u8, 2, 3, 4]);
        assert_eq!(header.prg_rom_size, 5);
        assert_eq!(header.chr_rom_size, 6);
        assert_eq!(header.flags_6, 7);
        assert_eq!(header.flags_7, 8);
        assert_eq!(header.prg_ram_size, 9);
        assert_eq!(header.flags_9, 10);
        assert_eq!(header.flags_10, 11);
        assert_eq!(header.padding, [12u8, 13, 14, 15, 16]);

        assert_eq!(header.prg_rom_size(), 5 * PRG_ROM_BANK_SIZE);
        assert_eq!(header.prg_rom_banks(), 5);
    }

    #[test_case(10, false)]
    #[test_case(0, true)]
    fn test_ext(chr_rom_size: u8, uses_chr_ram: bool) {
        let mut bytes = DEFAULT;
        bytes[5] = chr_rom_size;

        let header = INesHeader::from_bytes(&bytes);
        assert_eq!(
            header.chr_rom_size(),
            (chr_rom_size as usize) * CHR_ROM_BANK_SIZE
        );
        assert_eq!(header.uses_chr_ram(), uses_chr_ram);
    }

    #[test_case(1, MirroringType::Vertical)]
    #[test_case(3, MirroringType::Vertical)]
    #[test_case(0, MirroringType::Horizontal)]
    #[test_case(2, MirroringType::Horizontal)]
    fn test_mirroring(val: u8, mirroring: MirroringType) {
        let mut bytes = DEFAULT;
        bytes[6] = val;

        let header = INesHeader::from_bytes(&bytes);
        assert_eq!(header.mirroring_type(), mirroring);
    }

    #[test_case(3, true)]
    #[test_case(6, true)]
    #[test_case(5, false)]
    #[test_case(8, false)]
    fn test_has_other_memory(val: u8, has_other: bool) {
        let mut bytes = DEFAULT;
        bytes[6] = val;

        let header = INesHeader::from_bytes(&bytes);
        assert_eq!(header.has_other_memory(), has_other);
    }

    #[test_case(15, true)]
    #[test_case(12, true)]
    #[test_case(6, false)]
    #[test_case(0, false)]
    fn test_ignore_mirroring(val: u8, ignore: bool) {
        let mut bytes = DEFAULT;
        bytes[6] = val;

        let header = INesHeader::from_bytes(&bytes);
        assert_eq!(header.ignore_mirroring(), ignore);
    }

    #[test_case(15 << 4, 18 << 4, 18 << 4 | 15)]
    #[test_case(12 << 4 | 0x0F, 18 << 4 | 0x0E, 18 << 4 | 12)]
    fn test_mapping_number(flag_6: u8, flag_7: u8, map: u8) {
        let mut bytes = DEFAULT;
        bytes[6] = flag_6;
        bytes[7] = flag_7;

        let header = INesHeader::from_bytes(&bytes);
        assert_eq!(header.mapping_number(), map);
    }
}
