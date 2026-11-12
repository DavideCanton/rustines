use anyhow::Result;
use rustines_macro::dispatch_mapper;

use crate::{Mapper, MapperBox, arch::rom_structs::INesHeader};

fn boxed_mapper<M>(
    ctor: impl FnOnce(&INesHeader, Vec<u8>) -> Result<M>,
    header: &INesHeader,
    buf: Vec<u8>,
) -> Result<MapperBox>
where
    M: Mapper + 'static,
{
    ctor(header, buf).map(|m| Box::new(m) as MapperBox)
}

pub fn instantiate_mapper(header: &INesHeader, buf: Vec<u8>) -> Result<MapperBox> {
    dispatch_mapper!(0 - 3)
}

#[cfg(test)]
mod test {
    use bytemuck::Zeroable;

    use crate::arch::rom_structs::PRG_ROM_BANK_SIZE;

    use super::*;

    #[test]
    fn it_detects_0_correctly() {
        let mut header = INesHeader::zeroed();
        header.prg_rom_banks = 1;
        let mapper = instantiate_mapper(&header, vec![0; PRG_ROM_BANK_SIZE]);

        assert!(mapper.is_ok());
        assert_eq!(mapper.unwrap().name(), "Mapper0");
    }

    #[test]
    fn it_detects_none_correctly() {
        let mut header: INesHeader = (&[0xFF; 16]).into();
        header.prg_rom_banks = 1;
        let mapper = instantiate_mapper(&header, vec![]);

        assert!(mapper.is_err());
    }
}
