use crate::arch::mappers::mapper::Mapper;
use std::any::Any;

pub struct Mapper0;

impl Mapper for Mapper0 {
    impl_mapper!();
}