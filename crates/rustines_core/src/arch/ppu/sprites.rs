use std::fmt;

use bitfield::bitfield;
use bytemuck::{Pod, Zeroable};

use crate::bin;

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct OamSprite {
    // NOTE keep the attributes in this order since they are casted using bytemuck!
    pub y: u8,
    pub tile: u8,
    pub attr: SpriteAttr,
    pub x: u8,
}

impl OamSprite {
    const INVALID_SPRITE_DATA: [u8; 4] = (!0u32).to_le_bytes();

    pub fn from_bytes(bytes: &[u8]) -> (&OamSprite, bool) {
        debug_assert_eq!(bytes.len(), 4);
        let valid = bytes != OamSprite::INVALID_SPRITE_DATA;
        let oam_sprite: &OamSprite = bytemuck::from_bytes(bytes);
        (oam_sprite, valid)
    }
}

bitfield! {
    #[derive(Clone, Copy, Pod, Zeroable, Default)]
    #[repr(C)]
    pub struct SpriteAttr(u8);
    impl Debug;
    /** 7 -> Vertical flip */
    pub vertical_flip, _: 7;
    /** 6 -> Horizontal flip */
    pub horizontal_flip, _: 6;
    /** 5  -> Priority (0: in front of background; 1: behind background) */
    pub behind, _: 5;
    /** 0,1 -> Palette (4 to 7) of sprite */
    pub palette, _: 1, 0;
}

#[derive(Default, Debug, Clone, Copy, Zeroable)]
pub struct SpriteShifters {
    pub lo_shifter: u8,
    pub hi_shifter: u8,
    pub x: u8,
    pub attributes: SpriteAttr,
}

impl SpriteShifters {
    pub fn load(&mut self, oam_sprite: &OamSprite, lo_shifter: u8, hi_shifter: u8) {
        self.attributes = oam_sprite.attr;
        self.x = oam_sprite.x;
        self.lo_shifter = lo_shifter;
        self.hi_shifter = hi_shifter;
    }
}

impl fmt::Display for SpriteShifters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SpriteShifters[lo={}, hi={}, x={}, attributes={:?}]",
            bin!(self.lo_shifter),
            bin!(self.hi_shifter),
            self.x,
            self.attributes
        )
    }
}
