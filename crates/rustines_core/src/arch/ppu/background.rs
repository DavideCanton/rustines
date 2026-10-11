use crate::utils::bit_utils::{BitIndex as BI, extract_flag};

#[derive(Default, Debug, Clone, Copy)]
pub struct BackgroundData {
    pub shifter_pattern_lo: u16,
    pub shifter_pattern_hi: u16,
    pub shifter_attrib_lo: u16,
    pub shifter_attrib_hi: u16,
    pub latch_nt: u8,
    pub latch_at: u8,
    pub latch_pl: u8,
    pub latch_ph: u8,
}

impl BackgroundData {
    pub fn load_shifters(&mut self) {
        self.shifter_pattern_lo = (self.shifter_pattern_lo & 0xFF00) | self.latch_pl as u16;
        self.shifter_pattern_hi = (self.shifter_pattern_hi & 0xFF00) | self.latch_ph as u16;

        self.shifter_attrib_lo = (self.shifter_attrib_lo & 0xFF00)
            | if extract_flag(self.latch_at, BI::_0) {
                0xFF
            } else {
                0x00
            };
        self.shifter_attrib_hi = (self.shifter_attrib_hi & 0xFF00)
            | if extract_flag(self.latch_at, BI::_1) {
                0xFF
            } else {
                0x00
            };
    }

    pub fn update_shifters(&mut self) {
        self.shifter_pattern_lo <<= 1;
        self.shifter_pattern_hi <<= 1;
        self.shifter_attrib_lo <<= 1;
        self.shifter_attrib_hi <<= 1;
    }

    pub fn get_pixel_palette(&self, bit_mux: u16) -> (u8, u8) {
        let mut bg_pixel = 0;

        if self.shifter_pattern_lo & bit_mux != 0 {
            bg_pixel |= 0x1;
        }

        if self.shifter_pattern_hi & bit_mux != 0 {
            bg_pixel |= 0x2;
        }

        let mut bg_palette = 0;

        if self.shifter_attrib_lo & bit_mux != 0 {
            bg_palette |= 0x1;
        }

        if self.shifter_attrib_hi & bit_mux != 0 {
            bg_palette |= 0x2;
        }

        (bg_pixel, bg_palette)
    }
}
