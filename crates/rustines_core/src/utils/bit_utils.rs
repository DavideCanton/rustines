#![allow(clippy::just_underscores_and_digits)]

/// Converts the two values provided `low` and `high` to an `u16`
/// by making `(high << 8) | low`.
#[inline(always)]
pub fn to_u16(low: u8, high: u8) -> u16 {
    let low = low as u16;
    let high = high as u16;

    (high << 8) | low
}

/// Converts the provided value `v` into a pair of `u8`,
/// returning `(low, high)`.
#[inline(always)]
pub fn to_u8_lh(v: u16) -> (u8, u8) {
    let low = (v & 0b1111_1111) as u8;
    let high = ((v >> 8) & 0b1111_1111) as u8;

    (low, high)
}

/// Converts the two values provided `low` and `high` to an `u32`
/// by making `(high << 16) | low`.
#[inline(always)]
pub fn to_u32(low: u16, high: u16) -> u32 {
    let low = low as u32;
    let high = high as u32;

    (high << 16) | low
}

/// Converts the provided value `v` into a pair of `u16`,
/// returning `(low, high)`.
#[inline(always)]
pub fn to_u16_lh(v: u32) -> (u16, u16) {
    let low = (v & 0b1111_1111_1111_1111) as u16;
    let high = ((v >> 16) & 0b1111_1111_1111_1111) as u16;

    (low, high)
}

#[macro_export]
macro_rules! hex {
    ( $val:expr ) => {{ format!("{:02X}", $val) }};
}

#[macro_export]
macro_rules! hex16 {
    ( $val:expr ) => {{ format!("{:04X}", $val) }};
}

#[macro_export]
macro_rules! bin {
    ( $val:expr ) => {{ format!("{:08b}", $val) }};
}

#[derive(Clone, Copy, Debug)]
pub enum BitIndex {
    _0 = 0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
}

impl TryFrom<u8> for BitIndex {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use BitIndex::*;
        match value {
            0 => Ok(_0),
            1 => Ok(_1),
            2 => Ok(_2),
            3 => Ok(_3),
            4 => Ok(_4),
            5 => Ok(_5),
            6 => Ok(_6),
            7 => Ok(_7),
            _ => Err("bit index must be 0..=7"),
        }
    }
}

impl From<BitIndex> for u8 {
    fn from(value: BitIndex) -> Self {
        use BitIndex::*;
        match value {
            _0 => 0,
            _1 => 1,
            _2 => 2,
            _3 => 3,
            _4 => 4,
            _5 => 5,
            _6 => 6,
            _7 => 7,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BitCount {
    _1 = 1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
}

impl TryFrom<u8> for BitCount {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use BitCount::*;
        match value {
            1 => Ok(_1),
            2 => Ok(_2),
            3 => Ok(_3),
            4 => Ok(_4),
            5 => Ok(_5),
            6 => Ok(_6),
            7 => Ok(_7),
            8 => Ok(_8),
            _ => Err("bit count must be 1..=8"),
        }
    }
}

impl From<BitCount> for u8 {
    fn from(value: BitCount) -> Self {
        use BitCount::*;
        match value {
            _1 => 1,
            _2 => 2,
            _3 => 3,
            _4 => 4,
            _5 => 5,
            _6 => 6,
            _7 => 7,
            _8 => 8,
        }
    }
}

/// Extracts the bit at offset `offset` of `value` and returns true if it was 1, false else.
///
/// `offset` is counted from the right, so 0 is the rightmost bit.
#[inline(always)]
pub fn extract_flag(value: u8, offset: BitIndex) -> bool {
    let offset = offset as u8;
    (value & (1 << offset)) > 0
}

/// Sets the bit at offset `offset` of `value` to 1 if `flag` is true, else to 0.
#[inline(always)]
pub fn set_flag(value: u8, offset: BitIndex, flag: bool) -> u8 {
    let offset = offset as u8;
    if flag {
        value | (1 << offset)
    } else {
        value & !(1 << offset)
    }
}

/// Shifts to the right `value` of `shift` bits, then keeps only `count`.
///
/// For example, if value is `12345678`, shift is `3` and count is `2`, the result is `00000045`.
///
/// If `shift` is 0, the value is returned unchanged.
#[inline(always)]
pub fn extract_bits_shift(value: u8, shift: BitIndex, count: BitCount) -> u8 {
    let value = value >> (shift as u8);
    extract_bits_mask_lsb(value, count)
}

/// Extracts the MSB `count` bits of `value`.
///
/// For example, if value is `12345678` and count is `3`, the result is `12300000`.
///
/// If `count` is 0, 0 is returned.
#[inline(always)]
pub fn extract_bits_mask_msb(value: u8, count: BitCount) -> u8 {
    let count = count as u8;
    if count == 0 {
        0
    } else {
        let mask = !0 << (8 - count);
        value & mask
    }
}

/// Extracts the LSB `count` bits of `value`.
///
/// For example, if value is `12345678` and count is `3`, the result is `00000678`.
///
/// If `count` is 0, 0 is returned.
#[inline(always)]
pub fn extract_bits_mask_lsb(value: u8, count: BitCount) -> u8 {
    let count = count as u8;
    if count == 0 {
        0
    } else {
        let mask = !0 >> (8 - count);
        value & mask
    }
}

#[cfg(test)]
#[allow(clippy::assertions_on_constants)]
mod tests {
    use crate::utils::bit_utils::{BitCount, BitIndex, set_flag};

    use super::{extract_bits_mask_lsb, extract_bits_mask_msb, extract_bits_shift, extract_flag};

    #[test]
    fn test_extract_flag() {
        let v: u8 = 0b0000_1111;
        assert!(extract_flag(v, BitIndex::_0));
        assert!(extract_flag(v, BitIndex::_1));
        assert!(!extract_flag(v, BitIndex::_5));
    }

    #[test]
    fn test_set_flag() {
        let v: u8 = 0b0000_1111;
        assert_eq!(set_flag(v, BitIndex::_0, false), 0b0000_1110);
        assert_eq!(set_flag(v, BitIndex::_1, false), 0b0000_1101);
        assert_eq!(set_flag(v, BitIndex::_5, true), 0b0010_1111);
    }

    #[test]
    fn test_extract_bits_shift() {
        let v: u8 = 0b1010_1011;
        assert_eq!(
            extract_bits_shift(v, BitIndex::_4, BitCount::_4),
            0b0000_1010
        );
        assert_eq!(
            extract_bits_shift(v, BitIndex::_2, BitCount::_6),
            0b0010_1010
        );
        assert_eq!(
            extract_bits_shift(v, BitIndex::_2, BitCount::_5),
            0b0000_1010
        );
        assert_eq!(
            extract_bits_shift(v, BitIndex::_0, BitCount::_8),
            0b1010_1011
        );
        assert_eq!(
            extract_bits_shift(v, BitIndex::_0, BitCount::_6),
            0b0010_1011
        );
        assert_eq!(
            extract_bits_shift(v, BitIndex::_3, BitCount::_5),
            0b0001_0101
        );
        assert_eq!(
            extract_bits_shift(v, BitIndex::_3, BitCount::_1),
            0b0000_0001
        );
    }

    #[test]
    fn test_extract_bits_mask_msb() {
        let v: u8 = 0b1010_1011;
        assert_eq!(extract_bits_mask_msb(v, BitCount::_4), 0b1010_0000);
        assert_eq!(extract_bits_mask_msb(v, BitCount::_2), 0b1000_0000);
        assert_eq!(extract_bits_mask_msb(v, BitCount::_5), 0b1010_1000);
        assert_eq!(extract_bits_mask_msb(v, BitCount::_8), 0b1010_1011);
    }

    #[test]
    fn test_extract_bits_mask_lsb() {
        let v: u8 = 0b1010_1011;
        assert_eq!(extract_bits_mask_lsb(v, BitCount::_4), 0b0000_1011);
        assert_eq!(extract_bits_mask_lsb(v, BitCount::_2), 0b0000_0011);
        assert_eq!(extract_bits_mask_lsb(v, BitCount::_7), 0b0010_1011);
        assert_eq!(extract_bits_mask_lsb(v, BitCount::_8), 0b1010_1011);
    }

    #[test]
    fn test_bit_index_invalid() {
        BitIndex::try_from(8).expect_err("bit index must be 0..=7");
    }

    #[test]
    fn test_bit_count_invalid() {
        BitCount::try_from(9).expect_err("bit count must be 0..=8");
    }
}
