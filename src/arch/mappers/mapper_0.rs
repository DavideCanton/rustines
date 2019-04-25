use crate::arch::mappers::mapper::Mapper;
use std::any::Any;

pub struct Mapper0;

impl Mapper0 {
    pub fn new() -> Self {
        Mapper0 {}
    }
}

impl Mapper for Mapper0 {
    fn as_any(&self) -> &Any { self }
}