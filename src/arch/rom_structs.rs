use std::mem;

#[repr(packed)]
#[derive(Debug)]
pub struct Header {
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

pub enum MirroringType {
    Horizontal,
    Vertical,
}

macro_rules! extract_flag {
    ( $val:expr, $offset:expr ) => {{
        ($val & (1 << $offset)) > 0
    }};
}


impl Header {
    pub fn from_bytes(buf: &[u8; 16]) -> Self {
        unsafe { mem::transmute_copy(buf) }
    }

    pub fn prg_rom_size(&self) -> usize {
        (self.prg_rom_size as usize) << 14
    }

    pub fn prg_rom_banks(&self) -> usize {
        self.prg_rom_size as usize
    }

    pub fn chr_rom_size(&self) -> usize {
        (self.chr_rom_size as usize) << 13
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