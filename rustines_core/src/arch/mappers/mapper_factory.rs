use crate::arch::{
    mappers::{mapper::Mapper, mapper_0::Mapper0},
    rom_structs::INesHeader,
};

pub fn instantiate_mapper(header: &INesHeader, buf: Vec<u8>) -> Option<Box<dyn Mapper>> {
    match header.mapping_number() {
        0 => Some(Box::new(Mapper0::new(header, buf))),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use crate::arch::rom_structs::PRG_ROM_BANK_SIZE;

    use super::*;

    #[test]
    fn it_detects_0_correctly() {
        let mut header = INesHeader::from_bytes(&[0; 16]);
        header.prg_rom_size = 1;
        let mapper = instantiate_mapper(&header, vec![0; PRG_ROM_BANK_SIZE]);

        assert!(mapper.is_some());
        assert_eq!(mapper.unwrap().name(), "Mapper0");
    }

    #[test]
    fn it_detects_none_correctly() {
        let mut header = INesHeader::from_bytes(&[0xFF; 16]);
        header.prg_rom_size = 1;
        let mapper = instantiate_mapper(&header, vec![]);

        assert!(mapper.is_none());
    }
}
