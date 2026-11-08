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
    let low = (v & 0xFF) as u8;
    let high = ((v >> 8) & 0xFF) as u8;

    (low, high)
}

pub trait NumberFormat {
    fn hex(&self) -> String;
    fn bin(&self) -> String;
}

impl NumberFormat for u8 {
    #[inline(always)]
    fn hex(&self) -> String {
        format!("{:02X}", *self)
    }

    #[inline(always)]
    fn bin(&self) -> String {
        format!("{:08b}", *self)
    }
}

impl NumberFormat for u16 {
    #[inline(always)]
    fn hex(&self) -> String {
        format!("{:04X}", *self)
    }

    #[inline(always)]
    fn bin(&self) -> String {
        format!("{:016b}", *self)
    }
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

pub trait BitManip {
    type Index;
    type Count;

    fn get_bit(&self, offset: Self::Index) -> bool;
    fn set_bit(&self, offset: Self::Index, bit: bool) -> Self;
    fn extract_bits_shift(&self, shift: Self::Index, count: Self::Count) -> Self;
    fn extract_bits_mask_msb(&self, count: Self::Count) -> u8;
    fn extract_bits_mask_lsb(&self, count: Self::Count) -> u8;
}

impl BitManip for u8 {
    type Index = BitIndex;
    type Count = BitCount;

    /// Extracts the bit at offset `offset` of `value` and returns true if it was 1, false else.
    ///
    /// `offset` is counted from the right, so 0 is the rightmost bit.
    #[inline(always)]
    fn get_bit(&self, offset: Self::Index) -> bool {
        let offset = offset as u8;
        (*self & (1 << offset)) > 0
    }

    /// Sets the bit at offset `offset` of `value` to 1 if `bit` is true, else to 0.
    #[inline(always)]
    fn set_bit(&self, offset: Self::Index, bit: bool) -> u8 {
        let offset = offset as u8;
        if bit {
            *self | (1 << offset)
        } else {
            *self & !(1 << offset)
        }
    }

    /// Shifts to the right `value` of `shift` bits, then keeps only `count`.
    ///
    /// For example, if value is `12345678`, shift is `3` and count is `2`, the result is `00000045`.
    ///
    /// If `shift` is 0, the value is returned unchanged.
    #[inline(always)]
    fn extract_bits_shift(&self, shift: Self::Index, count: Self::Count) -> u8 {
        let value = *self >> (shift as u8);
        value.extract_bits_mask_lsb(count)
    }

    /// Extracts the MSB `count` bits of `value`.
    ///
    /// For example, if value is `12345678` and count is `3`, the result is `12300000`.
    ///
    /// If `count` is 0, 0 is returned.
    #[inline(always)]
    fn extract_bits_mask_msb(&self, count: Self::Count) -> u8 {
        let count = count as u8;
        if count == 0 {
            0
        } else {
            let mask = !0 << (8 - count);
            *self & mask
        }
    }

    /// Extracts the LSB `count` bits of `value`.
    ///
    /// For example, if value is `12345678` and count is `3`, the result is `00000678`.
    ///
    /// If `count` is 0, 0 is returned.
    #[inline(always)]
    fn extract_bits_mask_lsb(&self, count: Self::Count) -> u8 {
        let count = count as u8;
        if count == 0 {
            0
        } else {
            let mask = !0 >> (8 - count);
            *self & mask
        }
    }
}

#[cfg(test)]
#[allow(clippy::assertions_on_constants)]
mod tests {
    use super::*;
    use BitCount as BC;
    use BitIndex as BI;

    #[test]
    fn test_get_bit() {
        let v: u8 = 0b0000_1111;
        assert!(v.get_bit(BI::_0));
        assert!(v.get_bit(BI::_1));
        assert!(!v.get_bit(BI::_5));
    }

    #[test]
    fn test_set_bit() {
        let v: u8 = 0b0000_1111;
        assert_eq!(v.set_bit(BI::_0, false), 0b0000_1110);
        assert_eq!(v.set_bit(BI::_1, false), 0b0000_1101);
        assert_eq!(v.set_bit(BI::_5, true), 0b0010_1111);
    }

    #[test]
    fn test_extract_bits_shift() {
        let v: u8 = 0b1010_1011;
        assert_eq!(v.extract_bits_shift(BI::_4, BC::_4), 0b0000_1010);
        assert_eq!(v.extract_bits_shift(BI::_2, BC::_6), 0b0010_1010);
        assert_eq!(v.extract_bits_shift(BI::_2, BC::_5), 0b0000_1010);
        assert_eq!(v.extract_bits_shift(BI::_0, BC::_8), 0b1010_1011);
        assert_eq!(v.extract_bits_shift(BI::_0, BC::_6), 0b0010_1011);
        assert_eq!(v.extract_bits_shift(BI::_3, BC::_5), 0b0001_0101);
        assert_eq!(v.extract_bits_shift(BI::_3, BC::_1), 0b0000_0001);
    }

    #[test]
    fn test_extract_bits_mask_msb() {
        let v: u8 = 0b1010_1011;
        assert_eq!(v.extract_bits_mask_msb(BC::_4), 0b1010_0000);
        assert_eq!(v.extract_bits_mask_msb(BC::_2), 0b1000_0000);
        assert_eq!(v.extract_bits_mask_msb(BC::_5), 0b1010_1000);
        assert_eq!(v.extract_bits_mask_msb(BC::_8), 0b1010_1011);
    }

    #[test]
    fn test_extract_bits_mask_lsb() {
        let v: u8 = 0b1010_1011;
        assert_eq!(v.extract_bits_mask_lsb(BC::_4), 0b0000_1011);
        assert_eq!(v.extract_bits_mask_lsb(BC::_2), 0b0000_0011);
        assert_eq!(v.extract_bits_mask_lsb(BC::_7), 0b0010_1011);
        assert_eq!(v.extract_bits_mask_lsb(BC::_8), 0b1010_1011);
    }

    #[test]
    fn test_bit_index_invalid() {
        BI::try_from(8).expect_err("bit index must be 0..=7");
    }

    #[test]
    fn test_bit_count_invalid() {
        BC::try_from(9).expect_err("bit count must be 0..=8");
    }

    #[test]
    fn test_format_fn() {
        assert_eq!(0xFF_u8.bin(), "11111111");
        assert_eq!(0xFFFF_u16.bin(), "1111111111111111");

        assert_eq!(255_u8.hex(), "FF");
        assert_eq!(65535_u16.hex(), "FFFF");
    }
}
