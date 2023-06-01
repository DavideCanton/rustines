use rustines_macro::Named;

use crate::arch::mappers::mapper::Mapper;
use crate::utils::named::Named;

#[derive(Named)]
pub struct Mapper0;

impl Mapper for Mapper0 {}

#[cfg(test)]
mod tests {
    use super::Mapper0;
    use super::Named;

    // testing the procedural macro here since it's easier
    #[test]
    fn test_named() {
        let m = Mapper0;
        assert_eq!(m.name(), "Mapper0");
    }
}
