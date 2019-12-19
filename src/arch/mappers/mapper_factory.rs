use crate::arch::mappers::{mapper::Mapper, mapper_0::Mapper0};

pub fn instantiate_mapper(mapping_number: u8) -> Option<Box<dyn Mapper>> {
    match mapping_number {
        0 => Some(Box::new(Mapper0)),
        _ => None,
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_detects_0_correctly() {
        let mapper = instantiate_mapper(0);

        assert!(mapper.is_some());
        assert!(mapper.unwrap().as_any().is::<Mapper0>());
    }

    #[test]
    fn it_detects_none_correctly() {
        let mapper = instantiate_mapper(255);

        assert!(mapper.is_none());
    }
}
