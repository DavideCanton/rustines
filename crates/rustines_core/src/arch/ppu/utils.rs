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

pub fn decompose_rgba(rgba: u32) -> [u8; 4] {
    rgba.to_be_bytes()
}

pub fn compose_rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32)
}

#[cfg(test)]
mod tests {
    use crate::{arch::ppu::utils::get_bits_mask, compose_rgba, decompose_rgba};

    #[test]
    fn test_get_bits_mask() {
        assert_eq!(get_bits_mask(0u8, 0, 0xFF), 0x0);
        assert_eq!(get_bits_mask(0xFFu8, 0, 0xFF), 0x1);
        assert_eq!(get_bits_mask(0u8, 0xFF, 0xFF), 0x2);
        assert_eq!(get_bits_mask(0xFFu8, 0xFF, 0xFF), 0x3);
    }

    #[test]
    fn test_decompose_rgba() {
        assert_eq!(decompose_rgba(0x12345678), [0x12, 0x34, 0x56, 0x78])
    }

    #[test]
    fn test_compose_rgba() {
        assert_eq!(compose_rgba(0x12, 0x34, 0x56, 0x78), 0x12345678);
    }
}
