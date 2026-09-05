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
    Bit0 = 0,
    Bit1,
    Bit2,
    Bit3,
    Bit4,
    Bit5,
    Bit6,
    Bit7,
}

impl TryFrom<u8> for BitIndex {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BitIndex::Bit0),
            1 => Ok(BitIndex::Bit1),
            2 => Ok(BitIndex::Bit2),
            3 => Ok(BitIndex::Bit3),
            4 => Ok(BitIndex::Bit4),
            5 => Ok(BitIndex::Bit5),
            6 => Ok(BitIndex::Bit6),
            7 => Ok(BitIndex::Bit7),
            _ => Err("bit index must be 0..=7"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BitCount {
    Bit0 = 0,
    Bit1,
    Bit2,
    Bit3,
    Bit4,
    Bit5,
    Bit6,
    Bit7,
    Bit8,
}

impl TryFrom<u8> for BitCount {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BitCount::Bit0),
            1 => Ok(BitCount::Bit1),
            2 => Ok(BitCount::Bit2),
            3 => Ok(BitCount::Bit3),
            4 => Ok(BitCount::Bit4),
            5 => Ok(BitCount::Bit5),
            6 => Ok(BitCount::Bit6),
            7 => Ok(BitCount::Bit7),
            8 => Ok(BitCount::Bit8),
            _ => Err("bit count must be 0..=7"),
        }
    }
}

impl From<BitCount> for u8 {
    fn from(value: BitCount) -> Self {
        match value {
            BitCount::Bit0 => 0,
            BitCount::Bit1 => 1,
            BitCount::Bit2 => 2,
            BitCount::Bit3 => 3,
            BitCount::Bit4 => 4,
            BitCount::Bit5 => 5,
            BitCount::Bit6 => 6,
            BitCount::Bit7 => 7,
            BitCount::Bit8 => 8,
        }
    }
}

/// Extracts the bit at offset `offset` of `value` and returns true if it was 1, false else.
///
/// `offset` is counted from the right, so 0 is the rightmost bit.
///
/// Panics if `offset >= 8`.
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
///
/// Panics if `shift >= 8`.
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
///
/// Panics if `count > 8`.
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
///
/// Panics if `count > 8`.
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
        assert!(extract_flag(v, BitIndex::Bit0));
        assert!(extract_flag(v, BitIndex::Bit1));
        assert!(!extract_flag(v, BitIndex::Bit5));
    }

    #[test]
    fn test_set_flag() {
        let v: u8 = 0b0000_1111;
        assert_eq!(set_flag(v, BitIndex::Bit0, false), 0b0000_1110);
        assert_eq!(set_flag(v, BitIndex::Bit1, false), 0b0000_1101);
        assert_eq!(set_flag(v, BitIndex::Bit5, true), 0b0010_1111);
    }

    #[test]
    fn test_extract_bits_shift() {
        let v: u8 = 0b1010_1011;
        assert_eq!(
            extract_bits_shift(v, BitIndex::Bit4, BitCount::Bit4),
            0b0000_1010
        );
        assert_eq!(
            extract_bits_shift(v, BitIndex::Bit2, BitCount::Bit6),
            0b0010_1010
        );
        assert_eq!(
            extract_bits_shift(v, BitIndex::Bit2, BitCount::Bit5),
            0b0000_1010
        );
        assert_eq!(
            extract_bits_shift(v, BitIndex::Bit0, BitCount::Bit8),
            0b1010_1011
        );
        assert_eq!(
            extract_bits_shift(v, BitIndex::Bit0, BitCount::Bit6),
            0b0010_1011
        );
        assert_eq!(
            extract_bits_shift(v, BitIndex::Bit3, BitCount::Bit5),
            0b0001_0101
        );
        assert_eq!(
            extract_bits_shift(v, BitIndex::Bit3, BitCount::Bit1),
            0b0000_0001
        );
    }

    #[test]
    fn test_extract_bits_mask_msb() {
        let v: u8 = 0b1010_1011;
        assert_eq!(extract_bits_mask_msb(v, BitCount::Bit4), 0b1010_0000);
        assert_eq!(extract_bits_mask_msb(v, BitCount::Bit2), 0b1000_0000);
        assert_eq!(extract_bits_mask_msb(v, BitCount::Bit0), 0b0000_0000);
        assert_eq!(extract_bits_mask_msb(v, BitCount::Bit5), 0b1010_1000);
        assert_eq!(extract_bits_mask_msb(v, BitCount::Bit8), 0b1010_1011);
    }

    #[test]
    fn test_extract_bits_mask_lsb() {
        let v: u8 = 0b1010_1011;
        assert_eq!(extract_bits_mask_lsb(v, BitCount::Bit4), 0b0000_1011);
        assert_eq!(extract_bits_mask_lsb(v, BitCount::Bit2), 0b0000_0011);
        assert_eq!(extract_bits_mask_lsb(v, BitCount::Bit0), 0b0000_0000);
        assert_eq!(extract_bits_mask_lsb(v, BitCount::Bit7), 0b0010_1011);
        assert_eq!(extract_bits_mask_lsb(v, BitCount::Bit8), 0b1010_1011);
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
