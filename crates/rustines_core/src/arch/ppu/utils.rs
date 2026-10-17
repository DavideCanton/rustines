use std::ops::BitAnd;

trait IntType: BitAnd<Output = Self> + Copy + PartialEq {
    const ZERO: Self;
}

impl IntType for u8 {
    const ZERO: u8 = 0;
}
impl IntType for u16 {
    const ZERO: u16 = 0;
}

#[allow(private_bounds)]
/// Returns a two bits u8 `AB` such that:
/// - A is 1 if `hi & mask != 0`
/// - B is 1 if `lo & mask != 0`
pub fn get_bits_mask<T: IntType>(lo: T, hi: T, mask: T) -> u8 {
    let mut pixel = 0;

    if (lo & mask) != T::ZERO {
        pixel |= 0x1;
    }

    if (hi & mask) != T::ZERO {
        pixel |= 0x2;
    }

    pixel
}

#[cfg(test)]
mod tests {
    use crate::arch::ppu::utils::get_bits_mask;

    #[test]
    fn test_get_bits_mask() {
        assert_eq!(get_bits_mask(0u8, 0, 0xFF), 0x0);
        assert_eq!(get_bits_mask(0xFFu8, 0, 0xFF), 0x1);
        assert_eq!(get_bits_mask(0u8, 0xFF, 0xFF), 0x2);
        assert_eq!(get_bits_mask(0xFFu8, 0xFF, 0xFF), 0x3);
    }
}
