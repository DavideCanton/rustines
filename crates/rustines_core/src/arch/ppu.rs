use crate::{
    arch::{mappers::mapper::Mapper, rom_structs::MirroringType},
    renderer::Renderer,
    utils::bit_utils::{
        BitCount as BC, BitIndex as BI, extract_bits_shift, extract_flag, set_flag,
    },
};
use bitfield::bitfield;
use bytemuck::{Pod, Zeroable};

const OPEN_BUS_DECAY_FRAMES: u8 = 25;

bitfield! {
    #[derive(Clone, Copy, Pod, Zeroable)]
    #[repr(C)]
    struct SpriteAttr(u8);
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

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct Sprite {
    y: u8,
    tile: u8,
    attr: SpriteAttr,
    x: u8,
}

impl Sprite {
    pub fn pattern_table(&self) -> bool {
        extract_flag(self.tile, BI::_0)
    }
}

impl From<[u8; 4]> for Sprite {
    fn from(value: [u8; 4]) -> Self {
        bytemuck::cast(value)
    }
}

bitfield! {
    #[derive(Clone, Copy)]
    pub(crate) struct PpuCtrl(u8);
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

bitfield! {
    #[derive(Clone, Copy)]
    pub(crate) struct PpuMask(u8);
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

bitfield! {
    #[derive(Clone, Copy)]
    pub(crate) struct PpuStatus(u8);
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

pub struct Ppu {
    nametables: [u8; 2048],
    palette_table: [u8; 32],
    oam_data: [u8; 256],

    pub(crate) ctrl: PpuCtrl,
    pub(crate) mask: PpuMask,
    pub(crate) status: PpuStatus,
    pub(crate) open_bus_value: u8,
    open_bus_decay_timer: u8,

    pub(crate) vram_address: u16,
    pub(crate) temp_address: u16,
    pub(crate) oam_addr: u8,
    pub(crate) end_x: u8,
    pub(crate) data_buffer: u8,

    pub(crate) scanline: i16,
    pub(crate) cycle: u16,

    pub(crate) nmi_interrupt: bool,
    pub(crate) frame_ready: bool,
    pub(crate) is_odd_frame: bool,
    pub(crate) address_latch: bool,

    renderer: Box<dyn Renderer>,
}

impl Ppu {
    pub fn new(renderer: Box<dyn Renderer>) -> Self {
        Self {
            nametables: [0; 2048],
            palette_table: [0; 32],
            oam_data: [0; 256],

            ctrl: PpuCtrl(0),
            mask: PpuMask(0),
            status: PpuStatus(0),
            open_bus_value: 0,
            open_bus_decay_timer: 0,

            vram_address: 0,
            temp_address: 0,
            oam_addr: 0,
            end_x: 0,
            address_latch: false,
            data_buffer: 0,

            scanline: -1,
            cycle: 0,

            nmi_interrupt: false,
            frame_ready: false,
            is_odd_frame: false,
            renderer,
        }
    }

    pub fn nmi_requested(&self) -> bool {
        self.nmi_interrupt
    }

    pub fn clear_nmi(&mut self) {
        self.nmi_interrupt = false;
    }

    pub fn frame_ready(&self) -> bool {
        self.frame_ready
    }

    pub fn clear_frame_ready(&mut self) {
        self.frame_ready = false;
    }

    pub fn renderer(&mut self) -> &mut dyn Renderer {
        self.renderer.as_mut()
    }

    pub fn palette_table(&self) -> &[u8; 32] {
        &self.palette_table
    }

    pub fn tick(&mut self, mapper: &mut dyn Mapper) {
        self.cycle += 1;

        let rendering_enabled = self.mask.show_background() || self.mask.show_sprites();

        if rendering_enabled && self.scanline == -1 {
            if self.cycle == 256 {
                self.increment_vram_address_y();
            }
            if self.cycle == 257 {
                self.vram_address = (self.vram_address & 0b1111_1011_1110_0000)
                    | (self.temp_address & 0b0000_0100_0001_1111);
            }
            if self.cycle == 304 {
                self.vram_address = (self.vram_address & 0b1000_0100_0001_1111)
                    | (self.temp_address & 0b0111_1011_1110_0000);
            }
        }

        let max_cycles_for_this_scanline = 340
            + if self.scanline == -1 && rendering_enabled && self.is_odd_frame {
                0
            } else {
                1
            };

        if self.cycle >= max_cycles_for_this_scanline {
            self.cycle = 0;
            self.scanline += 1;

            if self.scanline > 260 {
                self.scanline = -1;
                self.frame_ready = true;
                self.is_odd_frame = !self.is_odd_frame;

                if self.open_bus_decay_timer > 0 {
                    self.open_bus_decay_timer -= 1;
                    if self.open_bus_decay_timer == 0 {
                        self.open_bus_value = 0;
                    }
                }
            }
        }

        if self.scanline >= 0 && self.scanline <= 239 && self.cycle == 256 {
            self.render_scanline(mapper);
        }

        if self.scanline == 241 && self.cycle == 1 {
            self.status.set_vblank_started(true);
            if self.ctrl.vblank_nmi_enable() {
                self.nmi_interrupt = true;
            }
        }

        if self.scanline == -1 && self.cycle == 1 {
            self.status.set_vblank_started(false);
            self.status.set_sprite_zero_hit(false);
            self.nmi_interrupt = false;
        }
        if !rendering_enabled {
            self.status.set_sprite_zero_hit(false);
        }
    }

    pub fn cpu_write(&mut self, reg_index: u8, value: u8, mapper: &dyn Mapper) {
        self.write_open_bus(value);
        match reg_index {
            0 => {
                let ctrl = PpuCtrl(value);
                if ctrl.ppu_write_ext() {
                    panic!("Bit 6 of PPUCTRL should NEVER be set");
                }
                self.ctrl = ctrl;
            }
            1 => self.mask = PpuMask(value),
            2 => {}
            3 => {
                self.oam_addr = value;
            }
            4 => {
                self.oam_data[self.oam_addr as usize] = value;
                self.oam_addr = self.oam_addr.wrapping_add(1);
            }
            5 => {
                if self.address_latch {
                    self.temp_address = (self.temp_address & 0b1000_1100_0001_1111)
                        | (((value & 0b0000_0111) as u16) << 12)
                        | (((value & 0b1111_1000) as u16) << 2);
                    self.address_latch = false;
                } else {
                    self.end_x = value & 0b0000_0111;
                    self.temp_address =
                        (self.temp_address & 0b1111_1111_1110_0000) | ((value >> 3) as u16);
                    self.address_latch = true;
                }
            }
            6 => {
                if self.address_latch {
                    self.temp_address =
                        (self.temp_address & 0b1111_1111_0000_0000) | (value as u16);
                    self.vram_address = self.temp_address & 0b0011_1111_1111_1111;
                    self.address_latch = false;
                } else {
                    self.temp_address = (self.temp_address & 0b0000_0000_1111_1111)
                        | (((value & 0b0011_1111) as u16) << 8);
                    self.address_latch = true;
                }
            }
            7 => {
                let current_addr = self.vram_address & 0b0011_1111_1111_1111;
                self.vram_write(current_addr, value, mapper);

                if current_addr >= 0x3F00 {
                    self.vram_write(current_addr - 0x1000, value, mapper);
                }

                let increment = if self.ctrl.vram_address_incr() { 32 } else { 1 };
                self.vram_address = self.vram_address.wrapping_add(increment);
            }
            _ => unreachable!(),
        }
    }

    fn write_open_bus(&mut self, value: u8) {
        self.open_bus_value = value;
        self.open_bus_decay_timer = OPEN_BUS_DECAY_FRAMES;
    }

    pub fn cpu_read(&mut self, reg_index: u8, mapper: &dyn Mapper) -> u8 {
        match reg_index {
            2 => {
                let mut data = self.status_bits_shadow();

                if self.cycle == 1 {
                    if self.scanline == 241 {
                        data &= 0b0111_1111;
                    } else if self.scanline == -1 {
                        data |= 0b1000_0000;
                    }
                }

                self.status.set_vblank_started(false);
                self.address_latch = false;

                if !(self.scanline == 241 && ((1..=3).contains(&self.cycle))) {
                    self.nmi_interrupt = false;
                }

                self.write_open_bus(data);
                data
            }
            7 => {
                let mut data = self.vram_buffer_shadow(mapper);

                let current_addr = self.vram_address & 0b0011_1111_1111_1111;

                // when reading through $2007, buffer the nametable at addr - 0x1000
                if current_addr >= 0x3F00 {
                    // if bit 0 of mask is 0, greyscale mode is enabled, mask the lower bits
                    if self.mask.grayscale() {
                        data &= 0b0011_0000;
                    }
                    // when reading palette data, the upper two bits of the open bus are preserved
                    data = (data & 0b0011_1111) | (self.open_bus_value & 0b1100_0000);
                    self.data_buffer = self.vram_read(current_addr - 0x1000, mapper);
                } else {
                    self.data_buffer = self.vram_read(current_addr, mapper);
                }

                self.vram_address += if self.ctrl.vram_address_incr() { 32 } else { 1 };
                self.write_open_bus(data);
                data
            }
            _ => self.open_bus_value,
        }
    }

    pub fn vram_read(&self, mut addr: u16, mapper: &dyn Mapper) -> u8 {
        addr &= 0b0011_1111_1111_1111;

        match addr {
            0x0000..=0x1FFF => mapper.fetch_chr_rom(addr),
            0x2000..=0x3EFF => {
                let idx = self.mirror_nametable_addr(addr, mapper.mirroring_mode());
                self.nametables[idx]
            }
            0x3F00..=0x3FFF => {
                let mut palette_addr = (addr & 0b0000_0000_0001_1111) as usize;

                if palette_addr >= 0x10 && (palette_addr & 0b0000_0011) == 0 {
                    palette_addr -= 0x10;
                }

                self.palette_table[palette_addr]
            }
            _ => 0,
        }
    }

    pub fn vram_write(&mut self, mut addr: u16, value: u8, mapper: &dyn Mapper) {
        addr &= 0b0011_1111_1111_1111;
        match addr {
            0x0000..=0x1FFF => {
                // TODO
                // if mapper.has_chr_ram() {
                //     chr_memory[address as usize] = data;
                // } else {
                // }
            }
            0x2000..=0x3EFF => {
                let idx = self.mirror_nametable_addr(addr, mapper.mirroring_mode());
                self.nametables[idx] = value;
            }
            0x3F00..=0x3FFF => {
                let mut palette_addr = (addr & 0b0000_0000_0001_1111) as usize;

                if palette_addr >= 0x10 && (palette_addr & 0b0000_0011) == 0 {
                    palette_addr -= 0x10;
                }

                self.palette_table[palette_addr] = value;
            }
            _ => {}
        }
    }

    fn render_scanline(&mut self, mapper: &dyn Mapper) {
        // TODO
        // if chr_rom.is_empty() {
        //     return;
        // }

        let bg_enabled = self.mask.show_background();
        let sprites_enabled = self.mask.show_sprites();
        if !bg_enabled && !sprites_enabled {
            return;
        }

        let y = self.scanline as usize;
        let tile_y = (y / 8) as u16;
        let pixel_y = (y % 8) as u16;

        let base_nametable_addr = 0x2000 + (self.ctrl.nametable_addr() as u16 * 0x0400);

        let visible_sprites = self.get_sprites_on_scanline();

        let bg_clip_left_8 = self.mask.show_background_leftmost();
        let sprite_clip_left_8 = self.mask.show_sprites_leftmost();

        for x in 0..=255 {
            let tile_x = (x / 8) as u16;
            let pixel_x = (x % 8) as u16;

            let nametable_index = tile_y * 32 + tile_x;
            let tile_id = self.vram_read(base_nametable_addr + nametable_index, mapper) as u16;

            let attribute_table_base = base_nametable_addr + 0x03C0;

            let attr_addr = attribute_table_base + ((tile_y / 4) * 8) + (tile_x / 4);
            let attribute_byte = self.vram_read(attr_addr, mapper);

            let shift = ((tile_y & 2) << 1) | (tile_x & 2);
            let palette_index = ((attribute_byte >> shift) & 0b0000_0011) as u16;

            let pattern_table_base = if self.ctrl.bg_pattern_table() {
                0x1000
            } else {
                0x0000
            };
            let tile_addr = pattern_table_base + (tile_id * 16) + pixel_y;

            let byte_low = mapper.fetch_chr_rom(tile_addr);
            let byte_high = mapper.fetch_chr_rom(tile_addr + 8);

            let color_index = get_color_index(byte_low, byte_high, pixel_x as u8);

            let palette_offset = if color_index == 0 {
                0
            } else {
                palette_index * 4
            };

            let palette_color_id =
                self.vram_read(0x3F00 + palette_offset + color_index as u16, mapper);
            let background_rgb = NES_PALETTE[(palette_color_id & 0b0011_1111) as usize];

            let mut pixel_color = background_rgb;

            for (sprite_index, sprite) in visible_sprites.iter().enumerate() {
                if x >= sprite.x as usize && x < sprite.x as usize + 8 {
                    let mut pixel_x = (x - sprite.x as usize) as u16;
                    // sprite y is delayed by 1 scanline
                    let mut pixel_y = (y as i16 - sprite.y as i16 - 1) as u16;

                    if sprite.attr.horizontal_flip() {
                        pixel_x = 7 - pixel_x;
                    }
                    if sprite.attr.vertical_flip() {
                        pixel_y = 7 - pixel_y;
                    }

                    let tile_addr = sprite_tile_addr(
                        sprite,
                        self.ctrl.sprite_size(),
                        self.ctrl.sprite_pattern_table(),
                    );
                    let tile_addr = if self.ctrl.sprite_size() {
                        let mut s_y = pixel_y;
                        let mut offset = 0;
                        if s_y >= 8 {
                            offset = 16;
                            s_y -= 8;
                        }
                        tile_addr + offset + s_y
                    } else {
                        tile_addr + pixel_y
                    };

                    let byte_low = mapper.fetch_chr_rom(tile_addr);
                    let byte_high = mapper.fetch_chr_rom(tile_addr + 8);
                    let sprite_pixel_bits = get_color_index(byte_low, byte_high, pixel_x as u8);

                    if sprite_pixel_bits != 0 {
                        if sprite_index == 0
                            && bg_enabled
                            && sprites_enabled
                            && color_index != 0
                            && x < 255
                            && (x >= 8 || !(bg_clip_left_8 || sprite_clip_left_8))
                        {
                            self.status.set_sprite_zero_hit(true);
                        }

                        let palette_num = sprite.attr.palette() as u16;
                        // sprite palettes are at address 0x3F10
                        let palette_addr = 0x3F10 + (palette_num << 2) + sprite_pixel_bits as u16;
                        let color_id = self.vram_read(palette_addr, mapper);

                        if !sprite.attr.behind() || color_index == 0 {
                            pixel_color = NES_PALETTE[(color_id & 0b0011_1111) as usize];
                            break;
                        }
                    }
                }
            }

            self.renderer.render_pixel(x, y, pixel_color);
        }
    }

    fn increment_vram_address_y(&mut self) {
        if (self.vram_address & 0b0111_0000_0000_0000) != 0x7000 {
            self.vram_address += 0x1000;
        } else {
            self.vram_address &= 0b1000_1111_1111_1111;
            let mut y = (self.vram_address & 0b0000_0011_1110_0000) >> 5;
            if y == 29 {
                y = 0;
                self.vram_address ^= 0x0800;
            } else if y == 31 {
                y = 0;
            } else {
                y += 1;
            }
            self.vram_address = (self.vram_address & 0b1111_1100_0001_1111) | (y << 5);
        }
    }

    fn mirror_nametable_addr(&self, addr: u16, mode: MirroringType) -> usize {
        let title_addr = addr & 0b0000_1111_1111_1111;
        match mode {
            MirroringType::Horizontal => {
                let mut idx = title_addr as usize;
                if (0x0400..0x0C00).contains(&title_addr) {
                    idx -= 0x0400;
                } else if title_addr >= 0x0C00 {
                    idx -= 0x0800;
                }
                idx
            }
            MirroringType::Vertical => {
                let mut idx = title_addr as usize;
                if title_addr >= 0x0800 {
                    idx -= 0x0800;
                }
                idx
            }
        }
    }

    fn get_sprites_on_scanline(&self) -> Vec<Sprite> {
        let mut sprites = Vec::new();

        // SAFETY: self.oam_data has always a length multiple of 4
        for chunk in unsafe { self.oam_data.as_chunks_unchecked::<4>() } {
            let sprite: Sprite = (*chunk).into();

            let sprite_y = sprite.y as i16 + 1;

            if self.scanline >= sprite_y && self.scanline < sprite_y + 8 {
                sprites.push(sprite);
                if sprites.len() == 8 {
                    break;
                }
            }
        }
        sprites
    }

    pub(crate) fn dma_copy(&mut self, buf: &[u8]) {
        self.oam_data.copy_from_slice(buf);
    }

    pub(crate) fn oam_data(&self) -> &[u8] {
        &self.oam_data
    }

    pub(crate) fn status_bits_shadow(&self) -> u8 {
        (self.status.0 & 0b1110_0000) | (self.open_bus_value & 0b0001_1111)
    }

    pub(crate) fn vram_buffer_shadow(&self, mapper: &dyn Mapper) -> u8 {
        let current_addr = self.vram_address & 0b0011_1111_1111_1111;
        if current_addr >= 0x3F00 {
            self.vram_read(current_addr, mapper)
        } else {
            self.data_buffer
        }
    }
}

pub fn get_color_index(byte_low: u8, byte_high: u8, pixel_x: u8) -> u8 {
    let bit_shift: BI = (7 - pixel_x).try_into().unwrap();

    let bit_low = extract_bits_shift(byte_low, bit_shift, BC::_1);
    let bit_high = extract_bits_shift(byte_high, bit_shift, BC::_1);

    (bit_high << 1) | bit_low
}

pub fn sprite_tile_addr(sprite: &Sprite, sprite_size: bool, sprite_pattern_table: bool) -> u16 {
    if sprite_size {
        let table = if sprite.pattern_table() { 0x1000 } else { 0 };
        let tile = set_flag(sprite.tile, BI::_0, false) as u16;
        table + (tile * 16)
    } else {
        let table = if sprite_pattern_table { 0x1000 } else { 0 };
        table + (sprite.tile as u16 * 16)
    }
}

const NES_PALETTE: [u32; 64] = [
    0x545454FF, 0x001E74FF, 0x081090FF, 0x300088FF, 0x440064FF, 0x5C0030FF, 0x540400FF, 0x3C1800FF,
    0x202A00FF, 0x083A00FF, 0x004000FF, 0x003C24FF, 0x00325CFF, 0x000000FF, 0x000000FF, 0x000000FF,
    0x989698FF, 0x084CC4FF, 0x303CE4FF, 0x5C1EDFFF, 0x8814B4FF, 0xA01478FF, 0x9C2028FF, 0x843C00FF,
    0x605A00FF, 0x347200FF, 0x187C00FF, 0x047858FF, 0x0068ACFF, 0x000000FF, 0x000000FF, 0x000000FF,
    0xECEEECFF, 0x4C9AF4FF, 0x788CF4FF, 0xB06CF4FF, 0xE45CE4FF, 0xF45CB4FF, 0xF46D64FF, 0xE48C24FF,
    0xC4AA00FF, 0x90C200FF, 0x68D224FF, 0x4CD278FF, 0x4CC2D4FF, 0x3C3C3CFF, 0x000000FF, 0x000000FF,
    0xECEEECFF, 0xA8CCF4FF, 0xBCC4F4FF, 0xD4B4F4FF, 0xECB0ECFF, 0xF4B0D4FF, 0xF4B8B4FF, 0xECC490FF,
    0xE4D080FF, 0xCCDC80FF, 0xBCE290FF, 0xACE2B4FF, 0xACDAECFF, 0xA8A8A8FF, 0x000000FF, 0x000000FF,
];
