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

#[derive(Clone, Copy)]
pub struct BitIndex(u8);

impl BitIndex {
    pub const BIT_0: Self = Self::new(0);
    pub const BIT_1: Self = Self::new(1);
    pub const BIT_2: Self = Self::new(2);
    pub const BIT_3: Self = Self::new(3);
    pub const BIT_4: Self = Self::new(4);
    pub const BIT_5: Self = Self::new(5);
    pub const BIT_6: Self = Self::new(6);
    pub const BIT_7: Self = Self::new(7);

    pub const fn new(value: u8) -> Self {
        assert!(BitIndex::validate(value), "bit index must be 0..=7");
        Self(value)
    }

    const fn validate(value: u8) -> bool {
        value < 8
    }
}

impl TryFrom<u8> for BitIndex {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        BitIndex::validate(value)
            .then_some(Self(value))
            .ok_or("bit index must be 0..=7")
    }
}

#[derive(Clone, Copy)]
pub struct BitCount(u8);

impl BitCount {
    pub const BIT_0: Self = Self::new(0);
    pub const BIT_1: Self = Self::new(1);
    pub const BIT_2: Self = Self::new(2);
    pub const BIT_3: Self = Self::new(3);
    pub const BIT_4: Self = Self::new(4);
    pub const BIT_5: Self = Self::new(5);
    pub const BIT_6: Self = Self::new(6);
    pub const BIT_7: Self = Self::new(7);
    pub const BIT_8: Self = Self::new(8);

    pub const fn new(value: u8) -> Self {
        assert!(BitCount::validate(value), "bit count must be 0..=8");
        Self(value)
    }

    const fn validate(value: u8) -> bool {
        value <= 8
    }
}

impl TryFrom<u8> for BitCount {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        BitCount::validate(value)
            .then_some(Self(value))
            .ok_or("bit count must be 0..=8")
    }
}

/// Extracts the bit at offset `offset` of `value` and returns true if it was 1, false else.
///
/// `offset` is counted from the right, so 0 is the rightmost bit.
///
/// Panics if `offset >= 8`.
#[inline(always)]
pub fn extract_flag(value: u8, offset: BitIndex) -> bool {
    (value & (1 << offset.0)) > 0
}

/// Sets the bit at offset `offset` of `value` to 1 if `flag` is true, else to 0.
#[inline(always)]
pub fn set_flag(value: u8, offset: BitIndex, flag: bool) -> u8 {
    let offset = offset.0;
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
///
/// Panics if `shift >= 8`.
#[inline(always)]
pub fn extract_bits_shift(value: u8, shift: BitIndex, count: BitCount) -> u8 {
    let value = value >> shift.0;
    extract_bits_mask_lsb(value, count)
}

/// Extracts the MSB `count` bits of `value`.
///
/// For example, if value is `12345678` and count is `3`, the result is `12300000`.
///
/// If `count` is 0, 0 is returned.
///
/// Panics if `count > 8`.
#[inline(always)]
pub fn extract_bits_mask_msb(value: u8, count: BitCount) -> u8 {
    let count = count.0;
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
///
/// Panics if `count > 8`.
#[inline(always)]
pub fn extract_bits_mask_lsb(value: u8, count: BitCount) -> u8 {
    let count = count.0;
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
        assert!(extract_flag(v, BitIndex::BIT_0));
        assert!(extract_flag(v, BitIndex::BIT_1));
        assert!(!extract_flag(v, BitIndex::BIT_5));
    }

    #[test]
    #[should_panic = "bit index must be 0..=7"]
    fn test_extract_flag_invalid() {
        extract_flag(0xF, BitIndex::new(8));
    }

    #[test]
    fn test_set_flag() {
        let v: u8 = 0b0000_1111;
        assert_eq!(set_flag(v, BitIndex::BIT_0, false), 0b0000_1110);
        assert_eq!(set_flag(v, BitIndex::BIT_1, false), 0b0000_1101);
        assert_eq!(set_flag(v, BitIndex::BIT_5, true), 0b0010_1111);
    }

    #[test]
    #[should_panic = "bit index must be 0..=7"]
    fn test_set_flag_invalid() {
        set_flag(0xF, BitIndex::new(8), true);
    }

    #[test]
    fn test_extract_bits_shift() {
        let v: u8 = 0b1010_1011;
        assert_eq!(
            extract_bits_shift(v, BitIndex::BIT_4, BitCount::BIT_4),
            0b0000_1010
        );
        assert_eq!(
            extract_bits_shift(v, BitIndex::BIT_2, BitCount::BIT_6),
            0b0010_1010
        );
        assert_eq!(
            extract_bits_shift(v, BitIndex::BIT_2, BitCount::BIT_5),
            0b0000_1010
        );
        assert_eq!(
            extract_bits_shift(v, BitIndex::BIT_0, BitCount::BIT_8),
            0b1010_1011
        );
        assert_eq!(
            extract_bits_shift(v, BitIndex::BIT_0, BitCount::BIT_6),
            0b0010_1011
        );
        assert_eq!(
            extract_bits_shift(v, BitIndex::BIT_3, BitCount::BIT_5),
            0b0001_0101
        );
        assert_eq!(
            extract_bits_shift(v, BitIndex::BIT_3, BitCount::BIT_1),
            0b0000_0001
        );
    }

    #[test]
    #[should_panic = "bit index must be 0..=7"]
    fn test_extract_bits_shift_invalid_shift() {
        extract_bits_shift(0, BitIndex::new(8), BitCount::BIT_1);
    }

    #[test]
    #[should_panic = "bit count must be 0..=8"]
    fn test_extract_bits_shift_invalid_count() {
        extract_bits_shift(0, BitIndex::BIT_1, BitCount::new(9));
    }

    #[test]
    fn test_extract_bits_mask_msb() {
        let v: u8 = 0b1010_1011;
        assert_eq!(extract_bits_mask_msb(v, BitCount::BIT_4), 0b1010_0000);
        assert_eq!(extract_bits_mask_msb(v, BitCount::BIT_2), 0b1000_0000);
        assert_eq!(extract_bits_mask_msb(v, BitCount::BIT_0), 0b0000_0000);
        assert_eq!(extract_bits_mask_msb(v, BitCount::BIT_5), 0b1010_1000);
        assert_eq!(extract_bits_mask_msb(v, BitCount::BIT_8), 0b1010_1011);
    }

    #[test]
    #[should_panic = "bit count must be 0..=8"]
    fn test_extract_bits_mask_msb_invalid() {
        extract_bits_mask_msb(0, BitCount::new(9));
    }

    #[test]
    fn test_extract_bits_mask_lsb() {
        let v: u8 = 0b1010_1011;
        assert_eq!(extract_bits_mask_lsb(v, BitCount::BIT_4), 0b0000_1011);
        assert_eq!(extract_bits_mask_lsb(v, BitCount::BIT_2), 0b0000_0011);
        assert_eq!(extract_bits_mask_lsb(v, BitCount::BIT_0), 0b0000_0000);
        assert_eq!(extract_bits_mask_lsb(v, BitCount::BIT_7), 0b0010_1011);
        assert_eq!(extract_bits_mask_lsb(v, BitCount::BIT_8), 0b1010_1011);
    }

    #[test]
    #[should_panic = "bit count must be 0..=8"]
    fn test_extract_bits_mask_lsb_invalid() {
        extract_bits_mask_lsb(0, BitCount::new(9));
    }
}
