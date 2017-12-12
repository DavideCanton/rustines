use arch::mappers::mapper::Mapper;
use arch::mappers::mapper_0::Mapper0;

pub struct MapperFactory;

impl MapperFactory {
    pub fn instantiate_mapper(mapping_number: u8) -> Option<Box<Mapper>> {
        match mapping_number {
            0 => Some(Box::new(Mapper0::new())),
            _ => None
        }
    }
}