pub fn to_u16(low: u8, high: u8) -> u16 {
    let low = low as u16;
    let high = high as u16;

    (high << 8) | low
}

pub fn to_u8_lh(v: u16) -> (u8, u8) {
    let low = (v & 0xFF) as u8;
    let high = ((v >> 8) & 0xFF) as u8;

    (low, high)
}

pub fn to_u32(low: u16, high: u16) -> u32 {
    let low = low as u32;
    let high = high as u32;

    (high << 16) | low
}

pub fn to_u16_lh(v: u32) -> (u16, u16) {
    let low = (v & 0xFFFF) as u16;
    let high = ((v >> 16) & 0xFFFF) as u16;

    (low, high)
}
