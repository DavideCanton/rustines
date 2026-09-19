use bitfield::bitfield;
use bytemuck::{Pod, Zeroable};

use crate::arch::mappers::mapper::MapperBox;

bitfield! {
    #[derive(Clone, Copy, Pod, Zeroable)]
    #[repr(C)]
    struct HeaderFlags(u16);
    impl Debug;
    u8;

    /** 6 / 0 -> Nametable arrangement: 0: horizontal mirrored, 1: vertically mirrored */
    pub mirroring, _: 0;
    /** 6 / 1 -> Cartridge contains battery-backed PRG RAM ($6000-7FFF) or other persistent memory */
    pub battery_backed_prg_ram, _: 1;
    /** 6 / 2 -> 512-byte trainer at $7000-$71FF (stored before PRG data) */
    pub has_trainer, _: 2;
    /** 6 / 3 -> Alternative nametable layout */
    pub alt_nametable_layout, _: 3;
    /** 6 / 4-7 -> Lower nybble of mapper number */
    pub mapper_number_lower, _: 7, 4;
    /** 7 / 0 -> VS Unisystem */
    pub vs_unisystem, _: 8;
    /** 7 / 1 -> PlayChoice-10 (8 KB of Hint Screen data stored after CHR data) */
    pub play_choice_10, _: 9;
    /** 7 / 2-3 -> If equal to 2, flags 8-15 are in NES 2.0 format */
    pub nes_2, _: 11, 10;
    /** 7 / 4-7 -> Upper nybble of mapper number */
    pub mapper_number_upper, _: 15, 12;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct INesHeader {
    pub header: [u8; 4],
    pub prg_rom_banks: u8,
    pub chr_rom_banks: u8,
    flags: HeaderFlags,
    pub prg_ram_size: u8,
    pub flags_9: u8,
    pub flags_10: u8,
    pub _padding: [u8; 5],
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum MirroringType {
    Horizontal,
    Vertical,
    // TODO add other types of mirroring
}

pub const PRG_ROM_BANK_SIZE: usize = 1 << 14;
pub const CHR_ROM_BANK_SIZE: usize = 1 << 13;

pub const TRAINER_SIZE: usize = 1 << 9;

pub const HEADER: &[u8; 4] = b"NES\x1A";

impl INesHeader {
    pub fn prg_rom_size(&self) -> usize {
        (self.prg_rom_banks as usize) * PRG_ROM_BANK_SIZE
    }

    pub fn prg_rom_banks(&self) -> usize {
        self.prg_rom_banks as usize
    }

    pub fn chr_rom_size(&self) -> usize {
        (self.chr_rom_banks as usize) * CHR_ROM_BANK_SIZE
    }

    pub fn uses_chr_ram(&self) -> bool {
        self.chr_rom_banks == 0
    }

    pub fn mirroring_type(&self) -> MirroringType {
        if self.flags.mirroring() {
            MirroringType::Vertical
        } else {
            MirroringType::Horizontal
        }
    }

    pub fn battery_backed_prg_ram(&self) -> bool {
        self.flags.battery_backed_prg_ram()
    }

    pub fn has_trainer(&self) -> bool {
        self.flags.has_trainer()
    }

    pub fn alt_nametable_layout(&self) -> bool {
        self.flags.alt_nametable_layout()
    }

    pub fn mapping_number(&self) -> u8 {
        (self.flags.mapper_number_upper() << 4) | self.flags.mapper_number_lower()
    }
}

impl From<&[u8; 16]> for INesHeader {
    fn from(value: &[u8; 16]) -> Self {
        bytemuck::cast(*value)
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
        let header: INesHeader = (&DEFAULT).into();
        assert_eq!(header.header, [1, 2, 3, 4]);
        assert_eq!(header.prg_rom_banks, 5);
        assert_eq!(header.chr_rom_banks, 6);
        assert_eq!(header.flags.0, 0x0807);
        assert_eq!(header.prg_ram_size, 9);
        assert_eq!(header.flags_9, 10);
        assert_eq!(header.flags_10, 11);
        assert_eq!(header._padding, [12, 13, 14, 15, 16]);

        assert_eq!(header.prg_rom_size(), 5 * PRG_ROM_BANK_SIZE);
        assert_eq!(header.prg_rom_banks(), 5);
    }

    #[test_case(10, false)]
    #[test_case(0, true)]
    fn test_ext(chr_rom_size: u8, uses_chr_ram: bool) {
        let mut bytes = DEFAULT;
        bytes[5] = chr_rom_size;

        let header: INesHeader = (&bytes).into();
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

        let header: INesHeader = (&bytes).into();
        assert_eq!(header.mirroring_type(), mirroring);
    }

    #[test_case(3, true)]
    #[test_case(6, true)]
    #[test_case(5, false)]
    #[test_case(8, false)]
    fn test_battery_backed_prg_ram(val: u8, has_other: bool) {
        let mut bytes = DEFAULT;
        bytes[6] = val;

        let header: INesHeader = (&bytes).into();
        assert_eq!(header.battery_backed_prg_ram(), has_other);
    }

    #[test_case(15, true)]
    #[test_case(12, true)]
    #[test_case(6, false)]
    #[test_case(0, false)]
    fn test_alt_nametable_layout(val: u8, ignore: bool) {
        let mut bytes = DEFAULT;
        bytes[6] = val;

        let header: INesHeader = (&bytes).into();
        assert_eq!(header.alt_nametable_layout(), ignore);
    }

    #[test_case(15 << 4, 18 << 4, 18 << 4 | 15)]
    #[test_case(12 << 4 | 0b0000_1111, 18 << 4 | 0b0000_1110, 18 << 4 | 12)]
    fn test_mapping_number(flag_6: u8, flag_7: u8, map: u8) {
        let mut bytes = DEFAULT;
        bytes[6] = flag_6;
        bytes[7] = flag_7;

        let header: INesHeader = (&bytes).into();
        assert_eq!(header.mapping_number(), map);
    }
}
