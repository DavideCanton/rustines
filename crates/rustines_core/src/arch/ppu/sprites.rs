use std::fmt;

use bitfield::bitfield;
use bytemuck::{Pod, Zeroable};

use crate::{
    arch::{
        memento::{MementoReader, MementoReadAndFill, MementoWriteable, MementoWriter},
        ppu::utils::get_bits_mask,
    },
    utils::bit_utils::NumberFormat,
};

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct OamSprite {
    // NOTE keep the attributes in this order since they are casted using bytemuck!
    pub y: u8,
    pub tile: u8,
    pub attr: SpriteAttributes,
    pub x: u8,
}

impl OamSprite {
    pub fn from_bytes(bytes: &[u8]) -> &OamSprite {
        debug_assert_eq!(bytes.len(), 4);
        let oam_sprite: &OamSprite = bytemuck::from_bytes(bytes);
        oam_sprite
    }
}

bitfield! {
    #[derive(Clone, Copy, Pod, Zeroable, Default)]
    #[repr(C)]
    pub struct SpriteAttributes(u8);
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

#[derive(Debug, Clone, Copy, Zeroable)]
pub struct SpriteShifters {
    #[allow(unused)]
    pub(super) index: usize,
    pub(super) lo_shifter: u8,
    pub(super) hi_shifter: u8,
    pub(super) x: u8,
    pub(super) attributes: SpriteAttributes,
}

impl SpriteShifters {
    pub const fn new(index: usize) -> Self {
        Self {
            index,
            lo_shifter: 0,
            hi_shifter: 0,
            x: 0,
            attributes: SpriteAttributes(0),
        }
    }

    pub fn load(&mut self, oam_sprite: &OamSprite, lo_shifter: u8, hi_shifter: u8) {
        self.attributes = oam_sprite.attr;
        self.x = oam_sprite.x;
        self.lo_shifter = lo_shifter;
        self.hi_shifter = hi_shifter;
    }

    pub fn palette(&self) -> u8 {
        self.attributes.palette() | 0x4
    }

    pub fn behind(&self) -> bool {
        self.attributes.behind()
    }

    pub fn reached_x(&self) -> bool {
        self.x == 0
    }

    pub fn dec_x(&mut self) {
        self.x -= 1;
    }

    pub fn get_bit_and_shift(&mut self) -> u8 {
        let pixel = get_bits_mask(self.lo_shifter, self.hi_shifter, 1 << 7);

        self.lo_shifter <<= 1;
        self.hi_shifter <<= 1;

        pixel
    }
}

impl fmt::Display for SpriteShifters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SpriteShifters[lo={}, hi={}, x={}, attributes={:?}]",
            self.lo_shifter.bin(),
            self.hi_shifter.bin(),
            self.x,
            self.attributes
        )
    }
}
impl MementoWriteable for SpriteShifters {
    fn write_memento<W: MementoWriter>(&self, writer: &mut W) {
        writer.write_memento(self.lo_shifter);
        writer.write_memento(self.hi_shifter);
        writer.write_memento(self.x);
        writer.write_memento(self.attributes.0);
    }
}

impl MementoReadAndFill for SpriteShifters {
    fn read_and_fill(&mut self, data: &mut MementoReader) {
        self.lo_shifter = data.read_memento();
        self.hi_shifter = data.read_memento();
        self.x = data.read_memento();
        self.attributes.0 = data.read_memento();
    }
}
