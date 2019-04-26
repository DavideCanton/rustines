use crate::arch::mappers::{mapper::Mapper, mapper_0::Mapper0};

pub fn instantiate_mapper(mapping_number: u8) -> Option<Box<dyn Mapper>> {
    match mapping_number {
        0 => Some(Box::new(Mapper0::new())),
        _ => None,
    }
}
