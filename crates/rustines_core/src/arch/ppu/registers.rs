use bitfield::bitfield;

use crate::utils::bit_utils::{BitIndex, extract_flag};

bitfield! {
    /// ```text
    /// yyy N N YYYYY XXXXX
    /// ||| | | ||||| +++++-- coarse X scroll
    /// ||| | | +++++-------- coarse Y scroll
    /// ||| | +-------------- nametable select X
    /// ||| +---------------- nametable select Y
    /// +++----------------- fine Y scroll
    /// ```
    #[derive(Clone, Copy)]
    pub(crate) struct PpuScrollRegister(pub u16);
    impl Debug;
    u8;
    pub coarse_x, set_coarse_x: 4, 0;
    pub coarse_y, set_coarse_y: 9, 5;
    pub nametable_select_x, set_nametable_select_x: 10;
    pub nametable_select_y, set_nametable_select_y: 11;
    pub fine_y, set_fine_y: 14, 12;
    pub unused, set_unused: 15;
}

impl PpuScrollRegister {
    pub fn address(&self) -> u16 {
        self.0 & 0x3FFF
    }

    pub fn nametable_select(&self) -> u8 {
        let y = self.nametable_select_y() as u8;
        let x = self.nametable_select_x() as u8;
        (y << 1) | x
    }

    pub fn switch_nametable_y(&mut self) {
        self.set_nametable_select_y(!self.nametable_select_y());
    }

    pub fn switch_nametable_x(&mut self) {
        self.set_nametable_select_x(!self.nametable_select_x());
    }

    pub fn set_nametable_select(&mut self, value: u8) {
        self.set_nametable_select_x(extract_flag(value, BitIndex::_0));
        self.set_nametable_select_y(extract_flag(value, BitIndex::_1));
    }

    pub fn set_address_high(&mut self, value: u8) {
        let value = value & 0x3F;
        self.0 = ((value as u16) << 8) | (self.0 & 0xFF);
        self.0 &= 0x3FFF;
    }

    pub fn set_address_low(&mut self, value: u8) {
        self.0 = (self.0 & 0xFF00) | (value as u16);
        self.0 &= 0x3FFF;
    }

    pub fn set_scroll_x(&mut self, value: u8) {
        self.set_unused(false);
        self.set_coarse_x(value >> 3);
    }

    pub fn set_scroll_y(&mut self, value: u8) {
        self.set_fine_y(value & 0x07);
        self.set_coarse_y(value >> 3);
    }

    pub fn increment_address(&mut self, increment: u16) {
        // TODO check coarse y and x increment
        self.0 = self.0.wrapping_add(increment) & 0x7FFF;
    }
}

bitfield! {
    #[derive(Clone, Copy)]
    pub(crate) struct PpuCtrl(pub u8);
    impl Debug;
    /** 7 -> Vblank NMI enable (0: off, 1: on) */
    pub vblank_nmi_enable, _: 7;
    /** 6 -> PPU master/slave select (0: read backdrop from EXT pins; 1: output color on EXT pins) */
    pub ppu_write_ext, _: 6;
    /** 5  -> Sprite size (0: 8x8 pixels; 1: 8x16 pixels – see PPU OAM#Byte 1) */
    pub sprite_size, _: 5;
    /** 4 -> Background pattern table address (0: $0000; 1: $1000) */
    pub bg_pattern_table, _: 4;
    /** 3 -> Sprite pattern table address for 8x8 sprites (0: $0000; 1: $1000; ignored in 8x16 mode) */
    pub sprite_pattern_table, _: 3;
    /** 2 -> VRAM address increment per CPU read/write of PPUDATA (0: add 1, going across; 1: add 32, going down) */
    pub vram_address_incr, _: 2;
    /** 10 -> Base nametable address (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00) */
    pub nametable_addr, _: 1, 0;
}

impl PpuCtrl {
    pub fn sprite_height(&self) -> u16 {
        if self.sprite_size() { 16 } else { 8 }
    }
}

bitfield! {
    #[derive(Clone, Copy)]
    pub(crate) struct PpuMask(pub u8);
    impl Debug;
    /** 7 -> Emphasize blue */
    pub emphasis_blue, set_emphasis_blue: 7;
    /** 6 -> Emphasize green (red on PAL/Dendy) */
    pub emphasis_green, set_emphasis_green: 6;
    /** 5 -> Emphasize red (green on PAL/Dendy) */
    pub emphasis_red, set_emphasis_red: 5;
    /** 4 -> 1: Enable sprite rendering */
    pub show_sprites, set_show_sprites: 4;
    /** 3 -> 1: Enable background rendering */
    pub show_background, set_show_background: 3;
    /** 2 -> 1: Show sprites in leftmost 8 pixels of screen, 0: Hide */
    pub show_sprites_leftmost, set_show_sprites_leftmost: 2;
    /** 1 -> 1: Show background in leftmost 8 pixels of screen, 0: Hide */
    pub show_background_leftmost, set_show_background_leftmost: 1;
    /** 0 -> Greyscale (0: normal color, 1: greyscale) */
    pub grayscale, set_grayscale: 0;
}

impl PpuMask {
    pub fn rendering_enabled(&self) -> bool {
        self.show_background() || self.show_sprites()
    }
}

bitfield! {
    #[derive(Clone, Copy)]
    pub(crate) struct PpuStatus(pub u8);
    impl Debug;
    /** 7 -> Vblank flag, cleared on read. Unreliable. */
    pub vblank_started, set_vblank_started: 7;
    /** 6 -> Sprite 0 hit flag */
    pub sprite_zero_hit, set_sprite_zero_hit: 6;
    /** 5 -> Sprite overflow flag */
    pub sprite_overflow, set_sprite_overflow: 5;
    /** 4-0 -> (PPU open bus or 2C05 PPU identifier) */
    pub open_bus, _: 4, 0;
}
