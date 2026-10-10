use crate::{
    arch::{mappers::mapper::Mapper, rom_structs::MirroringType},
    renderer::Renderer,
    utils::bit_utils::{
        BitCount as BC, BitIndex as BI, extract_bits_shift, extract_flag, set_flag,
    },
};
use bitfield::bitfield;
use bytemuck::{Pod, Zeroable};
use log::trace;

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
pub struct OamSprite {
    // NOTE keep the attributes in this order since they are casted using bytemuck!
    y: u8,
    tile: u8,
    attr: SpriteAttr,
    x: u8,
}

impl OamSprite {
    fn sprite_row(&self, next_scanline: i16) -> i16 {
        next_scanline - (self.y as i16) - 1
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

#[derive(Default, Debug, Clone, Copy, Zeroable)]
struct SpriteData {
    shifter_pattern_lo: u8,
    shifter_pattern_hi: u8,
    active_row: u8,
    active_x: u8,
    behind: bool,
    active_palette: u8,
    active_is_zero: bool,
}

impl SpriteData {
    fn from_oam_sprite(oam_sprite: &OamSprite, sprite_row: u8, is_sprite_zero: bool) -> Self {
        let mut data = SpriteData::default();

        if is_sprite_zero {
            data.active_is_zero = true;
        }

        data.active_row = sprite_row;
        data.active_x = oam_sprite.x;

        let attributes = oam_sprite.attr;
        data.active_palette = attributes.palette();
        data.behind = attributes.behind();

        data.shifter_pattern_lo = oam_sprite.tile;
        data.shifter_pattern_hi = attributes.0;

        data
    }

    fn update_shifter(&mut self, x_pos: usize) {
        if self.in_bounds_x(x_pos) {
            self.shifter_pattern_lo <<= 1;
            self.shifter_pattern_hi <<= 1;
        }
    }

    fn in_bounds_x(&self, x_pos: usize) -> bool {
        let sprite_x = self.active_x as usize;
        x_pos >= sprite_x && x_pos < sprite_x + 8
    }

    fn get_pixel(&self) -> u8 {
        let mut pixel = 0;

        if extract_flag(self.shifter_pattern_lo, BI::_7) {
            pixel |= 0x1;
        }

        if extract_flag(self.shifter_pattern_hi, BI::_7) {
            pixel |= 0x2;
        }

        pixel
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct BackgroundData {
    shifter_pattern_lo: u16,
    shifter_pattern_hi: u16,
    shifter_attrib_lo: u16,
    shifter_attrib_hi: u16,
    latch_nt: u8,
    latch_at: u8,
    latch_pl: u8,
    latch_ph: u8,
}
impl BackgroundData {
    fn load_shifters(&mut self) {
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

    fn update_shifters(&mut self) {
        self.shifter_pattern_lo <<= 1;
        self.shifter_pattern_hi <<= 1;
        self.shifter_attrib_lo <<= 1;
        self.shifter_attrib_hi <<= 1;
    }

    fn get_pixel_palette(&self, bit_mux: u16) -> (u8, u8) {
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum SpritePhase {
    Idle,
    Clearing,
    Searching,
    Copying,
    Fetching,
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

    /// ```text
    /// yyy NN YYYYY XXXXX
    /// ||| || ||||| +++++-- coarse X scroll
    /// ||| || +++++-------- coarse Y scroll
    /// ||| ++-------------- nametable select
    /// +++----------------- fine Y scroll
    /// ```
    pub(crate) v_reg: u16,
    pub(crate) t_reg: u16,
    pub(crate) x_reg: u8,
    pub(crate) w_toggle: bool,

    bg_data: BackgroundData,

    sprite_data: Vec<SpriteData>,
    secondary_oam: [u8; 32],
    n: u8,
    m: u8,
    secondary_oam_addr: u8,
    sprites_found: usize,
    sprite_phase: SpritePhase,

    pub(crate) oam_addr: u8,
    pub(crate) data_buffer: u8,

    pub(crate) scanline: i16,
    pub(crate) cycle: u16,
    pub(crate) frame: u16,

    pub(crate) nmi_interrupt: bool,
    pub(crate) frame_ready: bool,
    pub(crate) is_odd_frame: bool,

    zapper_position: Option<(usize, usize)>,
    zapper_light_timer: usize,

    renderer: Box<dyn Renderer>,
    tracing_enabled: bool,
}

const ZAPPER_LIGHT_PERSISTENCE: usize = 26 * 341;
const ZAPPER_SENSOR_RADIUS: usize = 4;
const ZAPPER_LIGHT_THRESHOLD: u32 = 128;

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

            v_reg: 0,
            t_reg: 0,
            x_reg: 0,
            w_toggle: false,

            bg_data: BackgroundData::default(),
            m: 0,
            n: 0,
            secondary_oam: [0; 32],
            secondary_oam_addr: 0,
            sprite_phase: SpritePhase::Idle,
            sprites_found: 0,
            sprite_data: Vec::with_capacity(8),

            oam_addr: 0,
            data_buffer: 0,

            scanline: -1,
            cycle: 0,
            frame: 0,

            nmi_interrupt: false,
            frame_ready: false,
            is_odd_frame: false,

            zapper_position: None,
            zapper_light_timer: 0,

            renderer,
            tracing_enabled: false,
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

    pub fn enable_tracing(&mut self, tracing_enabled: bool) {
        self.tracing_enabled = tracing_enabled;
    }

    pub fn renderer(&mut self) -> &mut dyn Renderer {
        self.renderer.as_mut()
    }

    pub fn set_zapper_position(&mut self, position: Option<(usize, usize)>) {
        if self.zapper_position != position {
            self.zapper_light_timer = 0;
        }
        self.zapper_position = position;
    }

    pub fn zapper_light_detected(&self) -> bool {
        self.zapper_light_timer > 0
    }

    pub fn palette_table(&self) -> &[u8; 32] {
        &self.palette_table
    }

    /// Returns true if the current scanline is the prerender one (-1)
    fn is_prerender_scanline(&self) -> bool {
        self.scanline == -1
    }

    /// Returns true if the current scanline is between 0 and 239
    fn is_visible_scanline(&self) -> bool {
        (0..=239).contains(&self.scanline)
    }

    /// Returns true if the current cycle is between 1 and 256
    fn is_visible_cycle(&self) -> bool {
        (1..=256).contains(&self.cycle)
    }

    fn max_cycles_for_current_scanline(&self) -> u16 {
        // the prerender scanline skips the last cycle on odd frames
        let skip_last =
            self.is_prerender_scanline() && self.rendering_enabled() && self.is_odd_frame;
        let offset = if skip_last { 0 } else { 1 };
        340 + offset
    }

    pub fn tick(&mut self, mapper: &mut dyn Mapper) {
        self.zapper_light_timer = self.zapper_light_timer.saturating_sub(1);
        let rendering_enabled = self.mask.show_background() || self.mask.show_sprites();

        let is_prerender_scanline = self.is_prerender_scanline();
        let is_visible_scanline = self.is_visible_scanline();

        // the ppu always emits the dot before updating the registers
        if is_visible_scanline && self.is_visible_cycle() {
            self.render_pixel(mapper);
        }

        let cycle = self.cycle;

        if cycle == 257 {
            self.sprite_data.clear();
        }

        if is_visible_scanline || is_prerender_scanline {
            // in the prerender scanline, second cycle, the vblank is cleared along with other
            // flags
            if is_prerender_scanline && cycle == 1 {
                self.status.set_vblank_started(false);
                self.status.set_sprite_zero_hit(false);
                self.status.set_sprite_overflow(false);
            }

            if rendering_enabled {
                // every cycle, the shifters must be updated so that the next write writes
                // the correct bit
                if (2..=257).contains(&cycle) || (322..=337).contains(&cycle) {
                    self.bg_data.update_shifters();
                }

                if self.is_visible_cycle() || (321..=336).contains(&cycle) {
                    self.cycle_load_data(mapper);
                }

                // cycle 256 in visible and pre render scanlines increments y scroll too
                if cycle == 256 {
                    self.increment_vram_address_y();
                }
                // cycle 257 in visible and pre render scanlines transfers the x scroll
                if cycle == 257 {
                    self.transfer_scroll_x();
                }

                // prerender scanline continuously transfers the y scroll between 280 and 304
                if is_prerender_scanline && (280..=304).contains(&cycle) {
                    self.transfer_scroll_y();
                }

                if is_visible_scanline {
                    self.handle_sprites(mapper);
                }
            }
        }

        // VBlank is ALWAYS set at scanline 241, cycle 1
        if self.scanline == 241 && self.cycle == 1 {
            self.status.set_vblank_started(true);
            if self.ctrl.vblank_nmi_enable() {
                self.nmi_interrupt = true;
            }
        }

        self.increase_cycle();
    }

    fn handle_sprites(&mut self, mapper: &mut dyn Mapper) {
        use SpritePhase::*;

        let cycle = self.cycle;

        self.sprite_phase = match self.sprite_phase {
            Idle if cycle == 1 => {
                // start clearing secondary oam at cycle 1
                self.secondary_oam_addr = 0;
                Clearing
            }
            Clearing if cycle.is_multiple_of(2) => {
                // secondary oam initialization, copy 0xFF every even cycle and increase
                // secondary oam address
                self.store_secondary_oam(0xFF);
                Clearing
            }
            Clearing if cycle == 65 => {
                // end clearing phase, start searching
                self.sprites_found = 0;
                self.secondary_oam_addr = 0;
                self.n = 0;
                self.m = 0;
                Searching
            }
            Searching if cycle == 257 => {
                // end search phase, start fetching
                // NOTE: at cycle 257 also fetch sprite 0
                self.fetch_sprite(mapper, cycle);
                Fetching
            }
            Searching => {
                // search for next sprite, if found start copying phase else continue searching
                if self.search_next_sprite() {
                    Copying
                } else {
                    Searching
                }
            }
            Copying => {
                debug_assert!(self.sprites_found < 9 && self.n < 64);
                let oam_addr = (self.n as usize) * 4 + (self.m as usize);
                // copy from oam to secondary oam
                self.store_secondary_oam(self.oam_data[oam_addr]);
                self.m += 1;
                if self.m == 4 {
                    // end copying the current sprite, resume to search the next
                    self.m = 0;
                    self.n += 1;
                    self.sprites_found += 1;
                    Searching
                } else {
                    Copying
                }
            }
            Fetching if (cycle - 257).is_multiple_of(8) => {
                // fetch a sprite every 8 cycles
                self.fetch_sprite(mapper, cycle);
                Fetching
            }
            Fetching if cycle == 320 => Idle,
            phase => phase,
        };
    }

    fn store_secondary_oam(&mut self, value: u8) {
        self.secondary_oam[self.secondary_oam_addr as usize] = value;
        self.secondary_oam_addr += 1;
    }

    fn search_next_sprite(&mut self) -> bool {
        let mut found = false;

        // if sprites_found is 9, no need to search for more sprites as more than 8 are not
        // supported. Don't stop at 8, try searching for the ninth just to set the sprite
        // overflow flag if found, without actually copying it
        // if self.n is >= 64, we arrived at the end of the primary oam. No more searching
        if self.sprites_found < 9 && self.n < 64 {
            let oam_addr = (self.n as usize) * 4 + (self.m as usize);
            let cur_sprite_y = self.oam_data[oam_addr];
            let sprite_height = if self.ctrl.sprite_size() { 16 } else { 8 };

            let sprite_row = self.scanline - cur_sprite_y as i16;
            if (0..sprite_height).contains(&sprite_row) {
                if self.sprites_found < 8 {
                    found = true;
                    self.secondary_oam[self.secondary_oam_addr as usize] = cur_sprite_y;
                    self.secondary_oam_addr += 1;
                    self.m = 1;
                } else {
                    self.status.set_sprite_overflow(true);
                    self.sprites_found += 1;
                }
            } else {
                self.n += 1;
            }
        }

        found
    }

    fn fetch_sprite(&mut self, mapper: &mut dyn Mapper, cycle: u16) {
        // sprite evaluation
        let sprite_idx = ((cycle - 257) >> 3) as usize;
        if sprite_idx < self.sprites_found.min(8) {
            self.fill_sprite_data(sprite_idx, mapper);
        }
    }

    fn cycle_load_data(&mut self, mapper: &mut dyn Mapper) {
        match self.cycle % 8 {
            // cycles 1-2: load nametable data
            2 => {
                let nt_address = 0x2000 | (self.v_reg & 0x0FFF);
                self.bg_data.latch_nt = self.vram_read(nt_address, mapper);
            }
            // cycles 3-4: load attribute table data
            4 => {
                let at_address = 0x23C0
                    | (self.v_reg & 0x0C00)
                    | ((self.v_reg >> 4) & 0x38)
                    | ((self.v_reg >> 2) & 0x07);
                let attribute = self.vram_read(at_address, mapper);

                let coarse_x = self.v_reg & 0x001F;
                let coarse_y = (self.v_reg >> 5) & 0x001F;
                let shift = ((coarse_y & 0x02) << 1) | (coarse_x & 0x02);
                self.bg_data.latch_at = (attribute >> shift) & 0x03;
            }
            // cycles 5-6: load bg low part
            6 => {
                let end_y = (self.v_reg >> 12) & 0x07;

                let table_select = if self.ctrl.bg_pattern_table() { 1 } else { 0 };
                let address = (table_select << 12) | ((self.bg_data.latch_nt as u16) << 4) | end_y;
                self.bg_data.latch_pl = self.vram_read(address, mapper);
            }
            // cycles 7-8: load bg high part, then transfer everything in the shifters
            // then increment vram x scroll
            0 => {
                let end_y = (self.v_reg >> 12) & 0x07;

                let table_select = if self.ctrl.bg_pattern_table() { 1 } else { 0 };
                let address =
                    (table_select << 12) | ((self.bg_data.latch_nt as u16) << 4) | end_y | 8;
                self.bg_data.latch_ph = self.vram_read(address, mapper);
                self.bg_data.load_shifters();
                self.increment_vram_address_x();
            }
            _ => {}
        }
    }

    fn update_sprite_shifters(&mut self) {
        let x_pos = (self.cycle - 1) as usize;

        for sprite_data in &mut self.sprite_data {
            sprite_data.update_shifter(x_pos);
        }
    }

    fn transfer_scroll_x(&mut self) {
        self.v_reg = (self.v_reg & 0x7BE0) | (self.t_reg & 0x041F);
    }

    fn transfer_scroll_y(&mut self) {
        self.v_reg = (self.v_reg & 0x041F) | (self.t_reg & 0x7BE0);
    }

    fn increase_cycle(&mut self) {
        self.cycle += 1;

        if self.cycle >= self.max_cycles_for_current_scanline() {
            self.cycle = 0;
            self.scanline += 1;

            if self.scanline == 261 {
                self.scanline = -1;
                self.frame_ready = true;
                self.frame += 1;
                self.is_odd_frame = !self.is_odd_frame;
                self.handle_open_bus_decay();
            }
        }
    }

    fn rendering_enabled(&self) -> bool {
        self.mask.show_background() || self.mask.show_sprites()
    }

    fn handle_open_bus_decay(&mut self) {
        if self.open_bus_decay_timer > 0 {
            self.open_bus_decay_timer -= 1;
            if self.open_bus_decay_timer == 0 {
                self.open_bus_value = 0;
            }
        }
    }

    pub fn cpu_write(&mut self, reg_index: u8, value: u8, mapper: &mut dyn Mapper) {
        if self.tracing_enabled {
            trace!("PPU CPU WRITE {reg_index:04X} {value:04X}");
        }
        self.write_open_bus(value);
        match reg_index {
            0 => {
                let old_nmi_enable = self.ctrl.vblank_nmi_enable();

                let ctrl = PpuCtrl(value);
                if ctrl.ppu_write_ext() {
                    panic!("Bit 6 of PPUCTRL should NEVER be set");
                }
                self.ctrl = ctrl;
                self.t_reg = (self.t_reg & !0xC00) | (((value & 0x3) as u16) << 10);

                if !old_nmi_enable && self.ctrl.vblank_nmi_enable() && self.status.vblank_started()
                {
                    self.nmi_interrupt = true;
                }
            }
            1 => {
                self.mask = PpuMask(value);
            }
            2 => {}
            3 => {
                self.oam_addr = value;
            }
            4 => {
                self.oam_data[self.oam_addr as usize] = value;
                self.oam_addr = self.oam_addr.wrapping_add(1);
            }
            5 => {
                if self.w_toggle {
                    self.t_reg = (self.t_reg & 0x0C1F)
                        | (((value & 0x7) as u16) << 12)
                        | (((value & 0xF8) as u16) << 2);
                } else {
                    self.x_reg = value & 0x7;
                    self.t_reg = (self.t_reg & !0x1F) | ((value >> 3) as u16);
                }
                self.w_toggle = !self.w_toggle;
            }
            6 => {
                if self.w_toggle {
                    self.t_reg = (self.t_reg & 0xFF00) | (value as u16);
                    self.v_reg = self.t_reg;
                } else {
                    self.t_reg = (self.t_reg & 0x00FF) | (((value & 0x3F) as u16) << 8);
                }
                self.w_toggle = !self.w_toggle;
            }
            7 => {
                let current_addr = self.v_reg & 0x3FFF;

                self.vram_write(current_addr, value, mapper);

                let increment = if self.ctrl.vram_address_incr() { 32 } else { 1 };
                self.v_reg = (self.v_reg.wrapping_add(increment)) & 0x7FFF;
            }
            _ => unreachable!(),
        }
    }

    fn write_open_bus(&mut self, value: u8) {
        self.open_bus_value = value;
        self.open_bus_decay_timer = OPEN_BUS_DECAY_FRAMES;
    }

    pub fn cpu_read(&mut self, reg_index: u8, mapper: &dyn Mapper) -> u8 {
        let ret = match reg_index {
            2 => {
                let mut data = self.status_bits_shadow();

                if self.cycle == 1 && (self.scanline == 241 || self.scanline == -1) {
                    data = set_flag(data, BI::_7, false);
                }

                self.status.set_vblank_started(false);
                self.w_toggle = false;

                if !(self.scanline == 241 && ((1..=3).contains(&self.cycle))) {
                    self.nmi_interrupt = false;
                }

                self.write_open_bus(data);
                data
            }
            4 => {
                if self.sprite_phase == SpritePhase::Clearing {
                    0xFF
                } else {
                    self.oam_data[self.oam_addr as usize]
                }
            }
            7 => {
                let mut data = self.vram_buffer_shadow(mapper);

                let current_addr = self.v_reg & 0x3FFF;

                // when reading through $2007, buffer the nametable at addr - 0x1000
                if current_addr >= 0x3F00 {
                    // when reading palette data, the upper two bits of the open bus are preserved
                    data = (data & 0x3F) | (self.open_bus_value & 0xC0);
                    self.data_buffer = self.vram_read(current_addr - 0x1000, mapper);
                } else {
                    self.data_buffer = self.vram_read(current_addr, mapper);
                }

                self.v_reg += if self.ctrl.vram_address_incr() { 32 } else { 1 };
                self.write_open_bus(data);
                data
            }
            _ => self.open_bus_value,
        };

        if self.tracing_enabled {
            trace!("PPU CPU READ {reg_index:04X} -> {ret:04X}");
        }

        ret
    }

    pub fn vram_read(&self, mut addr: u16, mapper: &dyn Mapper) -> u8 {
        let orig_addr = addr;

        addr &= 0x3FFF;

        let ret = match addr {
            0x0000..=0x1FFF => mapper.fetch_ppu(addr),
            0x2000..=0x3EFF => {
                let idx = mirror_nametable_addr(addr, mapper.mirroring_mode());
                self.nametables[idx]
            }
            0x3F00..=0x3FFF => {
                let palette_addr = normalize_palette_address(addr);
                self.palette_table[palette_addr]
            }
            _ => 0,
        };

        if self.tracing_enabled {
            trace!("PPU VRAM READ {orig_addr:04X} {addr:04X} {ret:04X}");
        }

        ret
    }

    pub fn vram_write(&mut self, mut addr: u16, value: u8, mapper: &mut dyn Mapper) {
        let orig_addr = addr;
        addr &= 0x3FFF;

        if self.tracing_enabled {
            trace!("PPU VRAM WRITE {orig_addr:04X} {addr:04X} {value:04X}");
        }

        match addr {
            0x0000..=0x1FFF => {
                mapper.store_ppu(addr, value);
            }
            0x2000..=0x3EFF => {
                let idx = mirror_nametable_addr(addr, mapper.mirroring_mode());
                self.nametables[idx] = value;
            }
            0x3F00..=0x3FFF => {
                let palette_addr = normalize_palette_address(addr);
                self.palette_table[palette_addr] = value;
            }
            _ => unreachable!(),
        }
    }

    fn render_pixel(&mut self, mapper: &dyn Mapper) {
        let x_pos = (self.cycle - 1) as usize;
        let y_pos = self.scanline as usize;
        let valid_bg_x = x_pos >= 8 || self.mask.show_background_leftmost();

        let (bg_pixel, bg_palette) = if !self.mask.show_background() || !valid_bg_x {
            (0, 0)
        } else {
            let bit_mux = 0x8000 >> self.x_reg;
            self.bg_data.get_pixel_palette(bit_mux)
        };

        let mut sprite_pixel = 0;
        let mut sprite_palette = 0;
        let mut sprite_behind = false;
        let mut is_sprite_zero = false;

        let valid_sprite_x = x_pos >= 8 || self.mask.show_sprites_leftmost();

        if self.mask.show_sprites() && valid_sprite_x {
            for sprite in &self.sprite_data {
                if sprite.in_bounds_x(x_pos) {
                    let pixel = sprite.get_pixel();

                    if pixel != 0 {
                        sprite_pixel = pixel;
                        sprite_palette = sprite.active_palette | 0x04;
                        sprite_behind = sprite.behind;
                        is_sprite_zero = sprite.active_is_zero;
                        break;
                    }
                }
            }
        }

        self.update_sprite_shifters();

        let (palette, pixel) = {
            if bg_pixel == 0 && sprite_pixel == 0 {
                (0, 0)
            } else if bg_pixel == 0 && sprite_pixel != 0 {
                (sprite_palette, sprite_pixel)
            } else if bg_pixel != 0 && sprite_pixel == 0 {
                (bg_palette, bg_pixel)
            } else {
                if is_sprite_zero {
                    let rendering_enabled = self.mask.show_background() && self.mask.show_sprites();

                    let mut clip_left = false;
                    if !self.mask.show_background_leftmost() || !self.mask.show_sprites_leftmost() {
                        clip_left = x_pos < 8;
                    }

                    let valid_cycle = self.cycle >= 1 && self.cycle <= 254;

                    if rendering_enabled && !clip_left && valid_cycle {
                        self.status.set_sprite_zero_hit(true);
                    }
                }

                if sprite_behind {
                    (bg_palette, bg_pixel)
                } else {
                    (sprite_palette, sprite_pixel)
                }
            }
        };

        let palette_addr = compute_palette_vram_address(palette, pixel);
        let color_index = self.vram_read(palette_addr, mapper);

        let rgb_color = read_palette_by_color_id(&self.mask, color_index);

        if self.zapper_position.is_some_and(|(aim_x, aim_y)| {
            x_pos.abs_diff(aim_x) <= ZAPPER_SENSOR_RADIUS
                && y_pos.abs_diff(aim_y) <= ZAPPER_SENSOR_RADIUS
                && is_zapper_light(rgb_color)
        }) {
            self.zapper_light_timer = ZAPPER_LIGHT_PERSISTENCE;
        }

        self.renderer.render_pixel(x_pos, y_pos, rgb_color);
    }

    fn fill_sprite_data(&mut self, sprite_idx: usize, mapper: &mut dyn Mapper) {
        let next_scanline = self.scanline + 1;
        let addr = sprite_idx * 4;
        let oam_sprite: &OamSprite = bytemuck::from_bytes(&self.secondary_oam[addr..addr + 4]);
        let sprite_row = oam_sprite.sprite_row(next_scanline) as u8;

        let mut sprite = SpriteData::from_oam_sprite(oam_sprite, sprite_row, sprite_idx == 0);

        let tile_index = sprite.shifter_pattern_lo;
        let attributes: &SpriteAttr = bytemuck::cast_ref(&sprite.shifter_pattern_hi);

        let flip_vertical = attributes.vertical_flip();
        let flip_horizontal = attributes.horizontal_flip();

        let mut row = sprite.active_row as u16;

        let (table_base, actual_tile) = {
            if self.ctrl.sprite_size() {
                if flip_vertical {
                    row = 15 - row;
                }

                let table_base = ((tile_index & 0x01) as u16) << 12;
                let mut actual_tile = (tile_index & 0xFE) as u16;

                if row >= 8 {
                    actual_tile += 1;
                    row -= 8;
                }
                (table_base, actual_tile)
            } else {
                if flip_vertical {
                    row = 7 - row;
                }
                let table_base = if self.ctrl.sprite_pattern_table() {
                    0x1000
                } else {
                    0
                };
                let actual_tile = tile_index as u16;
                (table_base, actual_tile)
            }
        };
        let address = table_base | (actual_tile << 4) | row;

        let mut pattern_lo = self.vram_read(address, mapper);
        let mut pattern_hi = self.vram_read(address | 8, mapper);

        if flip_horizontal {
            pattern_lo = pattern_lo.reverse_bits();
            pattern_hi = pattern_hi.reverse_bits();
        }

        sprite.shifter_pattern_lo = pattern_lo;
        sprite.shifter_pattern_hi = pattern_hi;

        self.sprite_data.push(sprite);
    }

    fn increment_vram_address_x(&mut self) {
        let mut v = self.v_reg;
        if (v & 0x1F) == 0x1F {
            // in this case, wraps the last 5 bits to 0
            v &= !0x1F;
            // switch the horizontal nametable
            v ^= 0x0400;
        } else {
            // increments the x part, that are the last 5 bits
            v += 1;
        }
        self.v_reg = v;
    }

    fn increment_vram_address_y(&mut self) {
        let mut v = self.v_reg;

        if (v & 0x7000) != 0x7000 {
            // if the y part is not maxed, simply increment it
            v += 0x1000;
        } else {
            // in this case, wrap the y bits to 0
            v &= !0x7000;

            // compute coarse y
            let mut y = (v & 0x3E0) >> 5;

            if y == 29 {
                // clear coarse y
                y = 0;
                // switch the vertical nametable
                v ^= 0x0800;
            } else if y == 31 {
                // clear coarse y without switching
                y = 0;
            } else {
                // increment coarse y
                y += 1;
            }

            // put back coarse y
            v = (v & !0x03E0) | (y << 5);
        }
        self.v_reg = v;
    }

    pub(crate) fn oam_data(&self) -> &[u8; 256] {
        &self.oam_data
    }

    pub(crate) fn status_bits_shadow(&self) -> u8 {
        (self.status.0 & 0b1110_0000) | (self.open_bus_value & 0b0001_1111)
    }

    pub(crate) fn vram_buffer_shadow(&self, mapper: &dyn Mapper) -> u8 {
        let current_addr = self.v_reg & 0b0011_1111_1111_1111;
        if current_addr >= 0x3F00 {
            self.vram_read(current_addr, mapper)
        } else {
            self.data_buffer
        }
    }
}

fn compute_palette_vram_address(palette: u8, pixel: u8) -> u16 {
    let mut offset: u16 = 0;

    if pixel != 0 {
        offset = (palette << 2) as u16 + pixel as u16;
    }

    0x3F00 + offset
}

pub fn get_color_index(byte_low: u8, byte_high: u8, pixel_x: u8) -> u8 {
    let bit_shift: BI = (7 - pixel_x).try_into().unwrap();

    let bit_low = extract_bits_shift(byte_low, bit_shift, BC::_1);
    let bit_high = extract_bits_shift(byte_high, bit_shift, BC::_1);

    (bit_high << 1) | bit_low
}

/// Normalizes the address and applies mirroring.
fn normalize_palette_address(addr: u16) -> usize {
    let mut palette_addr = (addr & 0b1_1111) as usize;

    // 0x3F0x is equal to 0x3F1x for x in [0, 4, 8, C]
    // these 4 values have the two lower bits equal to 0, so in these cases apply mirroring by
    // mapping 0x3F1x to 0x3F0x
    if palette_addr >= 0x10 && (palette_addr & 0b11) == 0 {
        palette_addr -= 0x10;
    }

    palette_addr
}

fn read_palette_by_color_id(mask: &PpuMask, color_id: u8) -> u32 {
    let color_id = if mask.grayscale() {
        color_id & 0b0011_0000
    } else {
        color_id & 0b0011_1111
    };
    let color = NES_PALETTE[color_id as usize];
    apply_emphasis(color, mask)
}

fn is_zapper_light(rgba: u32) -> bool {
    let red = (rgba >> 24) & 0xFF;
    let green = (rgba >> 16) & 0xFF;
    let blue = (rgba >> 8) & 0xFF;
    (red * 299 + green * 587 + blue * 114) / 1000 >= ZAPPER_LIGHT_THRESHOLD
}

fn mirror_nametable_addr(addr: u16, mode: MirroringType) -> usize {
    let nametable = (addr >> 10) & 0x03;
    let offset = (addr & 0x03FF) as usize;
    let physical_nametable = match mode {
        MirroringType::Horizontal => nametable >> 1,
        MirroringType::Vertical => nametable & 0x01,
    };

    (physical_nametable as usize) * 0x0400 + offset
}

fn apply_emphasis(color: u32, mask: &PpuMask) -> u32 {
    let r = (color >> 24) & 0xFF;
    let g = (color >> 16) & 0xFF;
    let b = (color >> 8) & 0xFF;

    let mut r2 = r;
    let mut g2 = g;
    let mut b2 = b;

    fn emphasize(col: u32) -> u32 {
        (col + 0x55).min(0xFF)
    }

    if mask.emphasis_red() {
        r2 = emphasize(r2);
    }
    if mask.emphasis_green() {
        g2 = emphasize(g2);
    }
    if mask.emphasis_blue() {
        b2 = emphasize(b2);
    }

    (r2 << 24) | (g2 << 16) | (b2 << 8) | 0xFF
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

#[cfg(test)]
mod tests {
    use super::PpuMask;
    use crate::{Mapper, MirroringType, NoopRenderer, Ppu, utils::named::Named};

    struct FakeMapper;

    impl Named for FakeMapper {
        fn name(&self) -> &str {
            "foo"
        }
    }

    impl Mapper for FakeMapper {
        fn prg_rom(&self) -> &[u8] {
            todo!()
        }

        fn chr_rom(&self) -> &[u8] {
            todo!()
        }

        fn mirroring_mode(&self) -> MirroringType {
            MirroringType::Vertical
        }

        fn fetch_cpu(&self, _addr: u16) -> u8 {
            todo!()
        }

        fn store_cpu(&mut self, _addr: u16, _val: u8) {
            todo!()
        }

        fn fetch_ppu(&self, _addr: u16) -> u8 {
            0
        }

        fn store_ppu(&mut self, _addr: u16, _val: u8) {
            todo!()
        }
    }

    #[test]
    fn test_sequence() {
        let renderer = NoopRenderer;
        let mut ppu = Ppu::new(Box::new(renderer));

        ppu.t_reg = 0b0111_1111_1111_1111;
        ppu.v_reg = 0b0111_1111_1111_1111;
        ppu.x_reg = 0b1111_1111;
        ppu.w_toggle = true;

        let mut mapper = FakeMapper;

        ppu.cpu_write(0, 0, &mut mapper);

        assert_eq!(ppu.t_reg, 0b0111_0011_1111_1111);
        assert_eq!(ppu.v_reg, 0b0111_1111_1111_1111);
        assert_eq!(ppu.x_reg, 0b1111_1111);
        assert!(ppu.w_toggle);

        ppu.cpu_read(2, &mapper);

        assert_eq!(ppu.t_reg, 0b0111_0011_1111_1111);
        assert_eq!(ppu.v_reg, 0b0111_1111_1111_1111);
        assert_eq!(ppu.x_reg, 0b1111_1111);
        assert!(!ppu.w_toggle);

        ppu.cpu_write(5, 0b0111_1101, &mut mapper);

        assert_eq!(ppu.t_reg, 0b0111_0011_1110_1111);
        assert_eq!(ppu.v_reg, 0b0111_1111_1111_1111);
        assert_eq!(ppu.x_reg, 0b0000_0101);
        assert!(ppu.w_toggle);

        ppu.cpu_write(5, 0b0101_1110, &mut mapper);

        assert_eq!(ppu.t_reg, 0b0110_0001_0110_1111);
        assert_eq!(ppu.v_reg, 0b0111_1111_1111_1111);
        assert_eq!(ppu.x_reg, 0b0000_0101);
        assert!(!ppu.w_toggle);

        ppu.cpu_write(6, 0b0011_1101, &mut mapper);

        assert_eq!(ppu.t_reg, 0b0011_1101_0110_1111);
        assert_eq!(ppu.v_reg, 0b0111_1111_1111_1111);
        assert_eq!(ppu.x_reg, 0b0000_0101);
        assert!(ppu.w_toggle);

        ppu.cpu_write(6, 0b11110000, &mut mapper);

        assert_eq!(ppu.t_reg, 0b0011_1101_1111_0000);
        assert_eq!(ppu.v_reg, 0b0011_1101_1111_0000);
        assert_eq!(ppu.x_reg, 0b0000_0101);
        assert!(!ppu.w_toggle);
    }

    #[test]
    fn sprite_evaluation_copies_in_range_sprites_and_skips_empty_slots() {
        let mut ppu = Ppu::new(Box::new(NoopRenderer));
        let mut mapper = FakeMapper;

        ppu.mask = PpuMask(0x10);
        ppu.scanline = 10;
        ppu.cycle = 65;
        ppu.oam_data[0] = 1;
        ppu.oam_data[4..8].copy_from_slice(&[8, 0x2A, 0x80, 0x34]);

        for _ in 65..=256 {
            ppu.tick(&mut mapper);
        }

        assert_eq!(ppu.sprites_found, 1);
        assert_eq!(&ppu.secondary_oam[..4], &[8, 0x2A, 0x80, 0x34]);
        assert_eq!(ppu.n, 64);

        for _ in 257..=320 {
            ppu.tick(&mut mapper);
        }

        assert_eq!(ppu.sprite_data[0].active_x, 0x34);
        assert!(ppu.sprite_data[0].active_is_zero);
        assert_eq!(ppu.sprite_data[1].active_x, 0);
        assert!(!ppu.sprite_data[1].active_is_zero);
    }

    #[test]
    fn sprite_evaluation_flags_overflow_on_ninth_sprite() {
        let mut ppu = Ppu::new(Box::new(NoopRenderer));
        let mut mapper = FakeMapper;

        ppu.mask = PpuMask(0x10);
        ppu.scanline = 10;
        ppu.cycle = 65;
        for sprite in 0..9 {
            ppu.oam_data[sprite * 4] = 8;
        }

        for _ in 65..=256 {
            ppu.tick(&mut mapper);
        }

        assert!(ppu.status.sprite_overflow());
        assert_eq!(ppu.sprites_found, 9);
        assert_eq!(ppu.secondary_oam_addr, 32);
    }
}
