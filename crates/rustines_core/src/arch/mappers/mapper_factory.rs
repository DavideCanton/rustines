use anyhow::{Result as AResult, bail};

use crate::{
    Mapper, MapperBox,
    arch::{
        mappers::{Mapper0, Mapper3},
        rom_structs::INesHeader,
    },
};

fn boxed_mapper<M>(
    ctor: impl FnOnce(&INesHeader, Vec<u8>) -> AResult<M>,
    header: &INesHeader,
    buf: Vec<u8>,
) -> AResult<MapperBox>
where
    M: Mapper + 'static,
{
    ctor(header, buf).map(|m| Box::new(m) as MapperBox)
}

pub fn instantiate_mapper(header: &INesHeader, buf: Vec<u8>) -> AResult<MapperBox> {
    match header.mapping_number() {
        0 => boxed_mapper(Mapper0::new, header, buf),
        3 => boxed_mapper(Mapper3::new, header, buf),
        _ => bail!("Invalid mapper"),
    }
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
