use std::{array, mem};

use crate::{
    arch::{
        mappers::mapper::Mapper,
        ppu::{
            OamSprite,
            background::BackgroundData,
            registers::{PpuCtrl, PpuMask, PpuScrollRegister, PpuStatus},
            sprites::SpriteShifters,
        },
        rom_structs::MirroringType,
    },
    compose_rgba, decompose_rgba,
    renderer::Renderer,
    utils::bit_utils::{BitCount as BC, BitIndex as BI, extract_bits_shift, set_flag},
};
use log::trace;

const OPEN_BUS_DECAY_FRAMES: u8 = 25;

pub struct Ppu {
    nametables: [u8; 2048],
    palette_table: [u8; 32],
    oam_data: [u8; 256],

    pub(crate) ctrl: PpuCtrl,
    pub(crate) mask: PpuMask,
    pub(crate) status: PpuStatus,
    pub(crate) open_bus_value: u8,
    open_bus_decay_timer: u8,

    pub(crate) v_reg: PpuScrollRegister,
    pub(crate) t_reg: PpuScrollRegister,
    pub(crate) x_reg: u8,
    pub(crate) w_toggle: bool,

    bg_data: BackgroundData,

    sprite_shifters: [SpriteShifters; 8],
    secondary_oam: [u8; 32],
    sprite_in_range: [bool; 8],
    secondary_oam_addr: u8,
    oam_data_latch: u8,
    sprite_zero_current: bool,
    sprite_zero_next: bool,
    secondary_oam_full: bool,
    oam_end: bool,
    to_copy: u8,
    suppress_vblank: bool,

    pub(crate) oam_addr: u8,
    pub(crate) data_buffer: u8,

    pub(crate) scanline: i16,
    pub(crate) cycle: u16,
    pub(crate) frame: u16,

    pub(crate) nmi_request: bool,
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

            v_reg: PpuScrollRegister(0),
            t_reg: PpuScrollRegister(0),
            x_reg: 0,
            w_toggle: false,

            bg_data: BackgroundData::default(),
            secondary_oam: [0; 32],
            secondary_oam_addr: 0,
            sprite_shifters: array::from_fn(SpriteShifters::new),
            sprite_in_range: [false; 8],
            oam_data_latch: 0,
            sprite_zero_current: false,
            sprite_zero_next: false,
            oam_end: false,
            secondary_oam_full: false,
            to_copy: 0,
            suppress_vblank: false,

            oam_addr: 0,
            data_buffer: 0,

            scanline: -1,
            cycle: 0,
            frame: 0,

            nmi_request: false,
            frame_ready: false,
            is_odd_frame: false,

            zapper_position: None,
            zapper_light_timer: 0,

            renderer,
            tracing_enabled: false,
        }
    }

    pub fn take_nmi_requested(&mut self) -> bool {
        mem::replace(&mut self.nmi_request, false)
    }

    pub fn clear_nmi(&mut self) {
        self.nmi_request = false;
    }

    pub fn current_frame(&self) -> u16 {
        self.frame
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

    pub fn renderer(&self) -> &dyn Renderer {
        self.renderer.as_ref()
    }

    pub fn renderer_mut(&mut self) -> &mut dyn Renderer {
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
            self.is_prerender_scanline() && self.mask.rendering_enabled() && self.is_odd_frame;
        let offset = if skip_last { 0 } else { 1 };
        340 + offset
    }

    pub fn tick(&mut self, mapper: &mut dyn Mapper) {
        self.zapper_light_timer = self.zapper_light_timer.saturating_sub(1);
        let rendering_enabled = self.mask.rendering_enabled();

        let is_prerender_scanline = self.is_prerender_scanline();
        let is_visible_scanline = self.is_visible_scanline();

        // the ppu always emits the dot before updating the registers
        if is_visible_scanline && self.is_visible_cycle() {
            self.render_pixel(mapper);
        }

        let cycle = self.cycle;

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
            if !self.suppress_vblank {
                self.status.set_vblank_started(true);
                if self.ctrl.vblank_nmi_enable() {
                    self.nmi_request = true;
                }
            }

            self.suppress_vblank = false;
        }

        self.increase_cycle();
    }

    fn handle_sprites(&mut self, mapper: &mut dyn Mapper) {
        let cycle = self.cycle;

        match cycle {
            1 => {
                // start clearing secondary oam at cycle 1
                self.secondary_oam_addr = 0;
                self.sprite_zero_next = false;
                self.oam_end = false;
                self.secondary_oam_full = false;
                self.sprite_in_range.fill(false);
            }
            2..=64 => {
                if cycle.is_multiple_of(2) {
                    // secondary oam initialization, copy 0xFF every even cycle and increase
                    // secondary oam address
                    self.secondary_oam[self.secondary_oam_addr as usize] = 0xFF;
                    self.secondary_oam_addr += 1;
                }
                if cycle == 64 {
                    // end clearing phase, start searching
                    self.secondary_oam_addr = 0;
                }
            }
            65..=256 => {
                if !cycle.is_multiple_of(2) {
                    self.oam_data_latch = self.oam_data[self.oam_addr as usize];
                } else {
                    if self.to_copy > 0 {
                        self.copy_sprite_byte();
                        self.to_copy -= 1;
                    } else {
                        if self.search_next_sprite() {
                            self.to_copy = 3;
                        }
                    }
                }
            }
            257..=320 => {
                if (cycle - 257).is_multiple_of(8) {
                    // fetch a sprite every 8 cycles
                    let sprite_idx = ((cycle - 257) >> 3) as usize;
                    self.fill_sprite_data(sprite_idx, mapper);
                }
                if !cycle.is_multiple_of(2) {
                    self.sprite_zero_current = self.sprite_zero_next;
                }
                // clear oamaddr every cycle
                self.oam_addr = 0;
            }
            _ => {}
        }
    }

    fn copy_sprite_byte(&mut self) {
        let mut m = self.oam_addr & 0x3;
        let mut n = self.oam_addr >> 2;

        if !self.secondary_oam_full {
            self.secondary_oam[self.secondary_oam_addr as usize] = self.oam_data_latch;
            self.secondary_oam_addr += 1;
            (m, n) = self.incr(m, n);
        }

        self.oam_addr = (n << 2) | m;
    }

    fn incr(&mut self, mut m: u8, mut n: u8) -> (u8, u8) {
        m += 1;
        if m == 4 {
            m = 0;
            n = self.incr_n(n);
        }
        (m, n)
    }

    fn incr_n(&mut self, mut n: u8) -> u8 {
        n = (n + 1) & 0x3F;
        if n == 0 {
            self.oam_end = true;
        }
        if self.secondary_oam_addr == 32 {
            self.secondary_oam_full = true;
            self.secondary_oam_addr = 0;
        }
        n
    }

    fn search_next_sprite(&mut self) -> bool {
        debug_assert!(self.cycle.is_multiple_of(2));

        if self.oam_end {
            return false;
        }

        let oam_value = self.oam_data_latch;
        let mut found = false;

        let mut m = self.oam_addr & 0x3;
        let mut n = self.oam_addr >> 2;

        if !self.secondary_oam_full {
            self.secondary_oam[self.secondary_oam_addr as usize] = oam_value;
        }
        if self.is_sprite_in_range_y(oam_value) {
            if self.secondary_oam_full {
                self.status.set_sprite_overflow(true);
            } else {
                found = true;
                self.sprite_in_range[(self.secondary_oam_addr >> 2) as usize] = true;
                // sprite zero is hit only if it's in range at cycle 66 (i.e. it's the first)
                // and only on visible scanlines. If this is the 239, the next is not visible
                if self.cycle == 66 && self.scanline < 239 {
                    self.sprite_zero_next = true;
                }
                (m, n) = self.incr(m, n);
                self.secondary_oam_addr += 1;
            }
        } else {
            n = self.incr_n(n);
            m = 0;
        }

        self.oam_addr = (n << 2) | m;

        found
    }

    fn is_sprite_in_range_y(&self, value: u8) -> bool {
        let sprite_y = value as i16;
        let sprite_height = self.ctrl.sprite_height() as i16;
        self.scanline >= sprite_y && self.scanline < sprite_y + sprite_height
    }

    fn cycle_load_data(&mut self, mapper: &mut dyn Mapper) {
        match self.cycle % 8 {
            // cycles 1-2: load nametable data
            2 => {
                let nt_address = 0x2000
                    | ((self.v_reg.nametable_select() as u16) << 10)
                    | ((self.v_reg.coarse_y() as u16) << 5)
                    | (self.v_reg.coarse_x() as u16);
                self.bg_data.latch_nt = self.vram_read(nt_address, mapper);
            }
            // cycles 3-4: load attribute table data
            4 => {
                let at_address = 0x23C0
                    | ((self.v_reg.nametable_select() as u16) << 10)
                    | (((self.v_reg.coarse_y() as u16) >> 2) & 0x07) << 3
                    | (((self.v_reg.coarse_x() as u16) >> 2) & 0x07);
                let attribute = self.vram_read(at_address, mapper);

                let coarse_x = self.v_reg.coarse_x();
                let coarse_y = self.v_reg.coarse_y();
                let shift = ((coarse_y & 0x02) << 1) | (coarse_x & 0x02);
                self.bg_data.latch_at = (attribute >> shift) & 0x03;
            }
            // cycles 5-6: load bg low part
            6 => {
                let end_y = self.v_reg.fine_y() as u16;

                let table_select = if self.ctrl.bg_pattern_table() { 1 } else { 0 };
                let address = (table_select << 12) | ((self.bg_data.latch_nt as u16) << 4) | end_y;
                self.bg_data.latch_pl = self.vram_read(address, mapper);
            }
            // cycles 7-8: load bg high part, then transfer everything in the shifters
            // then increment vram x scroll
            0 => {
                let end_y = self.v_reg.fine_y() as u16;

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

    fn transfer_scroll_x(&mut self) {
        self.v_reg.set_unused(false);
        self.v_reg.set_coarse_x(self.t_reg.coarse_x());
        self.v_reg.set_nametable_select(
            (self.v_reg.nametable_select() & 0x02) | (self.t_reg.nametable_select() & 0x01),
        );
    }

    fn transfer_scroll_y(&mut self) {
        self.v_reg.set_unused(false);
        self.v_reg.set_coarse_y(self.t_reg.coarse_y());
        self.v_reg
            .set_nametable_select(self.t_reg.nametable_select());
        self.v_reg.set_fine_y(self.t_reg.fine_y());
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
                self.t_reg.set_nametable_select(value & 0x03);

                if !old_nmi_enable && self.ctrl.vblank_nmi_enable() && self.status.vblank_started()
                {
                    self.nmi_request = true;
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
                    self.t_reg.set_scroll_y(value);
                } else {
                    self.x_reg = value & 0x7;
                    self.t_reg.set_scroll_x(value);
                }
                self.w_toggle = !self.w_toggle;
            }
            6 => {
                if self.w_toggle {
                    self.t_reg.set_address_low(value);
                    self.v_reg = self.t_reg;
                } else {
                    self.t_reg.set_address_high(value);
                }
                self.w_toggle = !self.w_toggle;
            }
            7 => {
                let current_addr = self.v_reg.address();

                self.vram_write(current_addr, value, mapper);

                let increment = if self.ctrl.vram_address_incr() { 32 } else { 1 };
                self.v_reg.increment_address(increment);
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

                if self.scanline == 241 {
                    // in this case the read happens before the vblank is actually set, so force
                    // it to false
                    if self.cycle == 1 {
                        data = set_flag(data, BI::_7, false);
                    }
                    // reading $2002 at this scanline skips the vblank for this frame
                    if self.cycle <= 1 {
                        self.suppress_vblank = true;
                    }
                } else {
                    if !(1..=3).contains(&self.cycle) {
                        self.nmi_request = false;
                    }
                }

                self.status.set_vblank_started(false);
                self.w_toggle = false;

                self.write_open_bus(data);
                data
            }
            4 => {
                if self.mask.rendering_enabled() && (2..=64).contains(&self.cycle) {
                    0xFF
                } else {
                    self.oam_data[self.oam_addr as usize]
                }
            }
            7 => {
                let mut data = self.vram_buffer_shadow(mapper);

                let current_addr = self.v_reg.address();

                // when reading through $2007, buffer the nametable at addr - 0x1000
                if current_addr >= 0x3F00 {
                    // when reading palette data, the upper two bits of the open bus are preserved
                    data = (data & 0x3F) | (self.open_bus_value & 0xC0);
                    self.data_buffer = self.vram_read(current_addr - 0x1000, mapper);
                } else {
                    self.data_buffer = self.vram_read(current_addr, mapper);
                }

                let increment = if self.ctrl.vram_address_incr() { 32 } else { 1 };

                self.v_reg.increment_address(increment);
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

    pub fn vram_read(&self, addr: u16, mapper: &dyn Mapper) -> u8 {
        let orig_addr = addr;

        let resolved_addr = addr & 0x3FFF;

        let ret = match resolved_addr {
            0x0000..=0x1FFF => mapper.fetch_ppu(resolved_addr),
            0x2000..=0x3EFF => {
                let idx = mirror_nametable_addr(resolved_addr, mapper.mirroring_mode());
                self.nametables[idx]
            }
            0x3F00..=0x3FFF => {
                let palette_addr = normalize_palette_address(resolved_addr);
                self.palette_table[palette_addr]
            }
            _ => 0,
        };

        if self.tracing_enabled {
            trace!("PPU VRAM READ {orig_addr:04X} {resolved_addr:04X} {ret:04X}");
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

        let (bg_pixel, bg_palette) = if self.mask.show_background() && valid_bg_x {
            let bit_mux = 0x8000 >> self.x_reg;
            self.bg_data.get_pixel_palette(bit_mux)
        } else {
            (0, 0)
        };

        let valid_sprite_x = x_pos >= 8 || self.mask.show_sprites_leftmost();

        let mut sprite_pixel = 0;
        let mut sprite_palette = 0;
        let mut sprite_behind = false;
        let mut sprite_zero = false;

        if self.mask.show_sprites() {
            for (i, sprite) in self.sprite_shifters.iter_mut().enumerate() {
                if !sprite.reached_x() {
                    sprite.dec_x();
                } else {
                    let pixel = sprite.get_bit_and_shift();

                    if valid_sprite_x && sprite_pixel == 0 && pixel != 0 {
                        sprite_pixel = pixel;
                        sprite_palette = sprite.palette();
                        sprite_behind = sprite.behind();
                        sprite_zero = i == 0;
                    }
                }
            }
        }

        let bg_data = (bg_palette, bg_pixel);
        let sp_data = (sprite_palette, sprite_pixel);

        let (palette, pixel) = match (bg_pixel, sprite_pixel) {
            (0, 0) => (0, 0),
            (0, _) => sp_data,
            (_, 0) => bg_data,
            _ => {
                if self.sprite_zero_current && sprite_zero {
                    let left_allowed =
                        self.mask.show_background_leftmost() && self.mask.show_sprites_leftmost();

                    let clip_left = if !left_allowed { x_pos < 8 } else { false };

                    // zero hit cannot happen on X=255
                    if self.mask.rendering_enabled() && !clip_left && x_pos < 255 {
                        self.status.set_sprite_zero_hit(true);
                    }
                }

                if sprite_behind { bg_data } else { sp_data }
            }
        };

        let palette_addr = compute_palette_vram_address(palette, pixel);
        let color_index = self.vram_read(palette_addr, mapper);

        let rgb_color = read_palette_by_color_id(&self.mask, color_index);

        let zapper_hit = self.zapper_position.is_some_and(|(x, y)| {
            x_pos.abs_diff(x) <= ZAPPER_SENSOR_RADIUS
                && y_pos.abs_diff(y) <= ZAPPER_SENSOR_RADIUS
                && is_zapper_light(rgb_color)
        });
        if zapper_hit {
            self.zapper_light_timer = ZAPPER_LIGHT_PERSISTENCE;
        }

        self.renderer.render_pixel(x_pos, y_pos, rgb_color);
    }

    fn fill_sprite_data(&mut self, sprite_idx: usize, mapper: &mut dyn Mapper) {
        let addr = sprite_idx * 4;
        let oam_sprite = OamSprite::from_bytes(&self.secondary_oam[addr..addr + 4]);

        let tile_index = oam_sprite.tile;
        let attributes = oam_sprite.attr;

        let flip_vertical = attributes.vertical_flip();
        let flip_horizontal = attributes.horizontal_flip();

        let mut sprite_row = self.scanline - (oam_sprite.y as i16);

        if flip_vertical {
            let sprite_height = self.ctrl.sprite_height() as i16;
            sprite_row = (sprite_height - 1) - sprite_row;
        }

        let mut sprite_row = sprite_row as u16;

        let (table_base, actual_tile) = {
            if self.ctrl.sprite_size() {
                let table_base = ((tile_index & 0x01) as u16) << 12;
                let mut actual_tile = (tile_index & 0xFE) as u16;

                if sprite_row >= 8 {
                    actual_tile += 1;
                    sprite_row -= 8;
                }
                (table_base, actual_tile)
            } else {
                let table_base = if self.ctrl.sprite_pattern_table() {
                    0x1000
                } else {
                    0
                };
                let actual_tile = tile_index as u16;
                (table_base, actual_tile)
            }
        };
        let address = table_base | (actual_tile << 4) | (sprite_row & 0x7);

        let mut lo_shifter = self.vram_read(address, mapper);
        let mut hi_shifter = self.vram_read(address | 8, mapper);

        if flip_horizontal {
            lo_shifter = lo_shifter.reverse_bits();
            hi_shifter = hi_shifter.reverse_bits();
        }

        if !self.sprite_in_range[sprite_idx] {
            lo_shifter = 0;
            hi_shifter = 0;
        }

        self.sprite_shifters[sprite_idx].load(oam_sprite, lo_shifter, hi_shifter);
    }

    fn increment_vram_address_x(&mut self) {
        let coarse_x = self.v_reg.coarse_x();
        if coarse_x == 0x1F {
            // in this case, wraps the last 5 bits to 0
            self.v_reg.set_coarse_x(0);
            // switch the horizontal nametable
            self.v_reg
                .set_nametable_select(self.v_reg.nametable_select() ^ 1);
        } else {
            // increments the x part, that are the last 5 bits
            self.v_reg.set_coarse_x(coarse_x + 1);
        }
    }

    fn increment_vram_address_y(&mut self) {
        let fine_y = self.v_reg.fine_y();
        if fine_y != 7 {
            // if the y part is not maxed, simply increment it
            self.v_reg.set_fine_y(fine_y + 1);
        } else {
            // in this case, wrap the y bits to 0
            self.v_reg.set_fine_y(0);

            // compute coarse y
            let mut y = self.v_reg.coarse_y();

            if y == 29 {
                // clear coarse y
                y = 0;
                // switch the vertical nametable
                self.v_reg
                    .set_nametable_select(self.v_reg.nametable_select() ^ 2);
            } else if y == 31 {
                // clear coarse y without switching
                y = 0;
            } else {
                // increment coarse y
                y += 1;
            }

            // put back coarse y
            self.v_reg.set_coarse_y(y);
        }
    }

    pub(crate) fn oam_data(&self) -> &[u8; 256] {
        &self.oam_data
    }

    pub(crate) fn secondary_oam_data(&self) -> &[u8; 32] {
        &self.secondary_oam
    }

    pub(crate) fn status_bits_shadow(&self) -> u8 {
        (self.status.0 & 0b1110_0000) | (self.open_bus_value & 0b0001_1111)
    }

    pub(crate) fn vram_buffer_shadow(&self, mapper: &dyn Mapper) -> u8 {
        let current_addr = self.v_reg.address();
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
    let [r, g, b, _] = decompose_rgba(rgba);
    let r = r as u32;
    let g = g as u32;
    let b = b as u32;
    (r * 299 + g * 587 + b * 114) / 1000 >= ZAPPER_LIGHT_THRESHOLD
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
    let [r, g, b, a] = decompose_rgba(color);

    let mut r2 = r;
    let mut g2 = g;
    let mut b2 = b;

    fn emphasize(col: u8) -> u8 {
        col + 0x55
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

    compose_rgba(r2, g2, b2, a)
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
    use super::{PpuMask, PpuScrollRegister};
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

        ppu.t_reg = PpuScrollRegister(0b0111_1111_1111_1111);
        ppu.v_reg = PpuScrollRegister(0b0111_1111_1111_1111);
        ppu.x_reg = 0b1111_1111;
        ppu.w_toggle = true;

        let mut mapper = FakeMapper;

        ppu.cpu_write(0, 0, &mut mapper);

        assert_eq!(ppu.t_reg.0, 0b0111_0011_1111_1111);
        assert_eq!(ppu.v_reg.0, 0b0111_1111_1111_1111);
        assert_eq!(ppu.x_reg, 0b1111_1111);
        assert!(ppu.w_toggle);

        ppu.cpu_read(2, &mapper);

        assert_eq!(ppu.t_reg.0, 0b0111_0011_1111_1111);
        assert_eq!(ppu.v_reg.0, 0b0111_1111_1111_1111);
        assert_eq!(ppu.x_reg, 0b1111_1111);
        assert!(!ppu.w_toggle);

        ppu.cpu_write(5, 0b0111_1101, &mut mapper);

        assert_eq!(ppu.t_reg.0, 0b0111_0010_0000_1111);
        assert_eq!(ppu.v_reg.0, 0b0111_1111_1111_1111);
        assert_eq!(ppu.x_reg, 0b0000_0101);
        assert!(ppu.w_toggle);

        ppu.cpu_write(5, 0b0101_1110, &mut mapper);

        assert_eq!(ppu.t_reg.0, 0b0110_0001_0110_1111);
        assert_eq!(ppu.v_reg.0, 0b0111_1111_1111_1111);
        assert_eq!(ppu.x_reg, 0b0000_0101);
        assert!(!ppu.w_toggle);

        ppu.cpu_write(6, 0b0011_1101, &mut mapper);

        assert_eq!(ppu.t_reg.0, 0b0011_1101_0110_1111);
        assert_eq!(ppu.v_reg.0, 0b0111_1111_1111_1111);
        assert_eq!(ppu.x_reg, 0b0000_0101);
        assert!(ppu.w_toggle);

        ppu.cpu_write(6, 0b11110000, &mut mapper);

        assert_eq!(ppu.t_reg.0, 0b0011_1101_1111_0000);
        assert_eq!(ppu.v_reg.0, 0b0011_1101_1111_0000);
        assert_eq!(ppu.x_reg, 0b0000_0101);
        assert!(!ppu.w_toggle);
    }

    #[test]
    fn vram_scroll_increments_wrap_coarse_coordinates_and_toggle_nametables() {
        let mut ppu = Ppu::new(Box::new(NoopRenderer));
        ppu.v_reg.set_coarse_x(0x1F);
        ppu.v_reg.set_coarse_y(29);
        ppu.v_reg.set_nametable_select(1);
        ppu.v_reg.set_fine_y(7);

        ppu.increment_vram_address_x();
        assert_eq!(ppu.v_reg.coarse_x(), 0);
        assert_eq!(ppu.v_reg.nametable_select(), 0);

        ppu.increment_vram_address_y();
        assert_eq!(ppu.v_reg.fine_y(), 0);
        assert_eq!(ppu.v_reg.coarse_y(), 0);
        assert_eq!(ppu.v_reg.nametable_select(), 2);

        ppu.v_reg.set_coarse_y(31);
        ppu.v_reg.set_nametable_select(3);
        ppu.v_reg.set_fine_y(7);
        ppu.increment_vram_address_y();
        assert_eq!(ppu.v_reg.fine_y(), 0);
        assert_eq!(ppu.v_reg.coarse_y(), 0);
        assert_eq!(ppu.v_reg.nametable_select(), 3);
    }

    #[test]
    fn sprite_evaluation_copies_in_range_sprites_and_skips_empty_slots() {
        let mut ppu = Ppu::new(Box::new(NoopRenderer));
        let mut mapper = FakeMapper;

        ppu.scanline = 10;
        ppu.mask = PpuMask(0x10);
        ppu.oam_data[0] = 1;
        ppu.oam_data[4..8].copy_from_slice(&[8, 0x2A, 0x80, 0x34]);

        for _ in 0..=256 {
            ppu.tick(&mut mapper);
        }

        assert_eq!(&ppu.secondary_oam[..4], &[8, 0x2A, 0x80, 0x34]);

        for _ in 257..=320 {
            ppu.tick(&mut mapper);
        }

        assert_eq!(ppu.sprite_shifters[0].x, 0x34);
    }

    #[test]
    fn sprite_evaluation_flags_overflow_on_ninth_sprite() {
        let mut ppu = Ppu::new(Box::new(NoopRenderer));
        let mut mapper = FakeMapper;

        ppu.mask = PpuMask(0x10);
        ppu.scanline = 10;

        for sprite in 0..9 {
            ppu.oam_data[sprite * 4] = 8;
        }

        for _ in 0..=256 {
            ppu.tick(&mut mapper);
        }

        assert!(ppu.status.sprite_overflow());
        assert!(ppu.secondary_oam_full);
    }
}
