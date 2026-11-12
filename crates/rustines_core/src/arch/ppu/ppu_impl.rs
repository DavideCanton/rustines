use std::{array, mem};

use crate::{
    arch::{
        mappers::mapper::Mapper,
        memento::{
            MementoReadAndFill, MementoReader, MementoResult, MementoWriteable, MementoWriter,
        },
        ppu::{
            OamSprite,
            background::BackgroundData,
            registers::{PpuCtrl, PpuMask, PpuScrollRegister, PpuStatus},
            sprites::SpriteShifters,
        },
        rom_structs::MirroringType,
    },
    compose_rgba, decompose_rgba,
    renderer::{Renderer, RendererBox},
    utils::bit_utils::{BitCount as BC, BitIndex as BI, BitManip},
};
use log::trace;

#[derive(Debug)]
enum RenderType {
    Sprite,
    Background,
    NoRender,
}

const OPEN_BUS_DECAY_FRAMES: u8 = 25;

pub struct PpuState {
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

    pub(crate) bg_data: BackgroundData,

    pub(crate) sprite_shifters: [SpriteShifters; 8],
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
    pub(crate) frame: u64,

    pub(crate) nmi_request: bool,
    pub(crate) frame_ready: bool,
    pub(crate) is_odd_frame: bool,

    zapper_position: Option<(usize, usize)>,
    zapper_light_timer: usize,

    tracing_enabled: bool,
    debug_render_sprites_enabled: bool,
    debug_render_background_enabled: bool,
}

impl PpuState {
    pub fn new() -> Self {
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

            tracing_enabled: false,
            debug_render_background_enabled: true,
            debug_render_sprites_enabled: true,
        }
    }
}

impl MementoWriteable for PpuState {
    fn write_memento(&self, writer: &mut dyn MementoWriter) -> MementoResult {
        self.nametables.write_memento(writer)?;
        self.palette_table.write_memento(writer)?;
        self.oam_data.write_memento(writer)?;
        self.ctrl.0.write_memento(writer)?;
        self.mask.0.write_memento(writer)?;
        self.status.0.write_memento(writer)?;
        self.open_bus_value.write_memento(writer)?;
        self.open_bus_decay_timer.write_memento(writer)?;
        self.v_reg.0.write_memento(writer)?;
        self.t_reg.0.write_memento(writer)?;
        self.x_reg.write_memento(writer)?;
        self.w_toggle.write_memento(writer)?;
        self.bg_data.write_memento(writer)?;
        self.sprite_shifters.write_memento(writer)?;
        self.secondary_oam.write_memento(writer)?;
        self.secondary_oam_addr.write_memento(writer)?;
        self.oam_data_latch.write_memento(writer)?;
        self.sprite_zero_current.write_memento(writer)?;
        self.sprite_zero_next.write_memento(writer)?;
        self.secondary_oam_full.write_memento(writer)?;
        self.oam_end.write_memento(writer)?;
        self.to_copy.write_memento(writer)?;
        self.suppress_vblank.write_memento(writer)?;
        self.oam_addr.write_memento(writer)?;
        self.data_buffer.write_memento(writer)?;
        (self.scanline as u16).write_memento(writer)?;
        self.cycle.write_memento(writer)?;
        self.frame.write_memento(writer)?;
        self.nmi_request.write_memento(writer)?;
        self.frame_ready.write_memento(writer)?;
        self.is_odd_frame.write_memento(writer)?;
        self.zapper_position.write_memento(writer)?;
        self.zapper_light_timer.write_memento(writer)?;
        self.tracing_enabled.write_memento(writer)?;
        self.debug_render_sprites_enabled.write_memento(writer)?;
        self.debug_render_background_enabled.write_memento(writer)?;
        Ok(())
    }
}

impl MementoReadAndFill for PpuState {
    fn read_and_fill(&mut self, reader: &mut MementoReader) -> MementoResult {
        self.nametables = reader.read_memento()?;
        self.palette_table = reader.read_memento()?;
        self.oam_data = reader.read_memento()?;
        self.ctrl.0 = reader.read_memento()?;
        self.mask.0 = reader.read_memento()?;
        self.status.0 = reader.read_memento()?;
        self.open_bus_value = reader.read_memento()?;
        self.open_bus_decay_timer = reader.read_memento()?;
        self.v_reg.0 = reader.read_memento()?;
        self.t_reg.0 = reader.read_memento()?;
        self.x_reg = reader.read_memento()?;
        self.w_toggle = reader.read_memento()?;
        self.bg_data.read_and_fill(reader)?;
        for shifter in &mut self.sprite_shifters {
            shifter.read_and_fill(reader)?;
        }
        self.secondary_oam = reader.read_memento()?;
        self.secondary_oam_addr = reader.read_memento()?;
        self.oam_data_latch = reader.read_memento()?;
        self.sprite_zero_current = reader.read_memento()?;
        self.sprite_zero_next = reader.read_memento()?;
        self.secondary_oam_full = reader.read_memento()?;
        self.oam_end = reader.read_memento()?;
        self.to_copy = reader.read_memento()?;
        self.suppress_vblank = reader.read_memento()?;
        self.oam_addr = reader.read_memento()?;
        self.data_buffer = reader.read_memento()?;
        self.scanline = reader.read_memento::<u16>()? as i16;
        self.cycle = reader.read_memento()?;
        self.frame = reader.read_memento()?;
        self.nmi_request = reader.read_memento()?;
        self.frame_ready = reader.read_memento()?;
        self.is_odd_frame = reader.read_memento()?;
        self.zapper_position = reader.read_memento()?;
        self.zapper_light_timer = reader.read_memento()?;
        self.tracing_enabled = reader.read_memento()?;
        self.debug_render_sprites_enabled = reader.read_memento()?;
        self.debug_render_background_enabled = reader.read_memento()?;
        Ok(())
    }
}

pub struct Ppu {
    pub(crate) state: PpuState,
    renderer: RendererBox,
}

const ZAPPER_LIGHT_PERSISTENCE: usize = 26 * 341;
const ZAPPER_SENSOR_RADIUS: usize = 4;
const ZAPPER_LIGHT_THRESHOLD: u32 = 128;

impl Ppu {
    pub fn new(renderer: RendererBox) -> Self {
        Self {
            state: PpuState::new(),
            renderer,
        }
    }

    pub fn reset(&mut self) {
        self.state = PpuState::new();
    }

    pub fn state(&self) -> &PpuState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut PpuState {
        &mut self.state
    }

    pub fn take_nmi_requested(&mut self) -> bool {
        mem::replace(&mut self.state.nmi_request, false)
    }

    pub fn clear_nmi(&mut self) {
        self.state.nmi_request = false;
    }

    pub fn current_frame(&self) -> u64 {
        self.state.frame
    }

    pub fn frame_ready(&self) -> bool {
        self.state.frame_ready
    }

    pub fn clear_frame_ready(&mut self) {
        self.state.frame_ready = false;
    }

    pub fn enable_tracing(&mut self, tracing_enabled: bool) {
        self.state.tracing_enabled = tracing_enabled;
    }

    pub fn renderer(&self) -> &dyn Renderer {
        self.renderer.as_ref()
    }

    pub fn renderer_mut(&mut self) -> &mut dyn Renderer {
        self.renderer.as_mut()
    }

    pub fn set_zapper_position(&mut self, position: Option<(usize, usize)>) {
        if self.state.zapper_position != position {
            self.state.zapper_light_timer = 0;
        }
        self.state.zapper_position = position;
    }

    pub fn zapper_light_detected(&self) -> bool {
        self.state.zapper_light_timer > 0
    }

    pub fn palette_table(&self) -> &[u8; 32] {
        &self.state.palette_table
    }

    /// Returns true if the current scanline is the prerender one (-1)
    fn is_prerender_scanline(&self) -> bool {
        self.state.scanline == -1
    }

    /// Returns true if the current scanline is between 0 and 239
    fn is_visible_scanline(&self) -> bool {
        (0..=239).contains(&self.state.scanline)
    }

    /// Returns true if the current cycle is between 1 and 256
    fn is_visible_cycle(&self) -> bool {
        (1..=256).contains(&self.state.cycle)
    }

    fn max_cycles_for_current_scanline(&self) -> u16 {
        // the prerender scanline skips the last cycle on odd frames
        let skip_last = self.is_prerender_scanline()
            && self.state.mask.rendering_enabled()
            && self.state.is_odd_frame;
        let offset = if skip_last { 0 } else { 1 };
        340 + offset
    }

    pub fn tick(&mut self, mapper: &mut dyn Mapper) {
        self.state.zapper_light_timer = self.state.zapper_light_timer.saturating_sub(1);
        let rendering_enabled = self.state.mask.rendering_enabled();

        let is_prerender_scanline = self.is_prerender_scanline();
        let is_visible_scanline = self.is_visible_scanline();

        // the ppu always emits the dot before updating the registers
        if is_visible_scanline && self.is_visible_cycle() {
            self.render_pixel(mapper);
        }

        let cycle = self.state.cycle;

        if is_visible_scanline || is_prerender_scanline {
            // in the prerender scanline, second cycle, the vblank is cleared along with other
            // flags
            if is_prerender_scanline && cycle == 1 {
                self.state.status.set_vblank_started(false);
                self.state.status.set_sprite_zero_hit(false);
                self.state.status.set_sprite_overflow(false);
            }

            if rendering_enabled {
                // every cycle, the shifters must be updated so that the next write writes
                // the correct bit
                if (2..=257).contains(&cycle) || (322..=337).contains(&cycle) {
                    self.state.bg_data.update_shifters();
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
                    self.do_sprite_evaluation(mapper);
                }
            }
        }

        // VBlank is ALWAYS set at scanline 241, cycle 1
        if self.state.scanline == 241 && self.state.cycle == 1 {
            if !self.state.suppress_vblank {
                self.state.status.set_vblank_started(true);
                if self.state.ctrl.vblank_nmi_enable() {
                    self.state.nmi_request = true;
                }
            }

            self.state.suppress_vblank = false;
        }

        self.increase_cycle();
    }

    fn do_sprite_evaluation(&mut self, mapper: &mut dyn Mapper) {
        let cycle = self.state.cycle;

        match cycle {
            1 => {
                // start clearing secondary oam at cycle 1
                self.state.secondary_oam_addr = 0;
                self.state.sprite_zero_next = false;
                self.state.oam_end = false;
                self.state.secondary_oam_full = false;
                self.state.sprite_in_range.fill(false);
            }
            2..=64 => {
                if cycle.is_multiple_of(2) {
                    // secondary oam initialization, copy 0xFF every even cycle and increase
                    // secondary oam address
                    self.state.secondary_oam[self.state.secondary_oam_addr as usize] = 0xFF;
                    self.state.secondary_oam_addr += 1;
                }
                if cycle == 64 {
                    // end clearing phase, start searching
                    self.state.secondary_oam_addr = 0;
                }
            }
            65..=256 => {
                if !cycle.is_multiple_of(2) {
                    self.state.oam_data_latch = self.state.oam_data[self.state.oam_addr as usize];
                } else {
                    if self.state.to_copy > 0 {
                        self.copy_sprite_byte();
                        self.state.to_copy -= 1;
                    } else {
                        if self.search_next_sprite() {
                            self.state.to_copy = 3;
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
                    self.state.sprite_zero_current = self.state.sprite_zero_next;
                }
                // clear oamaddr every cycle
                self.state.oam_addr = 0;
            }
            _ => {}
        }
    }

    fn copy_sprite_byte(&mut self) {
        let mut m = self.state.oam_addr & 0x3;
        let mut n = self.state.oam_addr >> 2;

        if !self.state.secondary_oam_full {
            self.state.secondary_oam[self.state.secondary_oam_addr as usize] =
                self.state.oam_data_latch;
            self.state.secondary_oam_addr += 1;
            (m, n) = self.incr(m, n);
        }

        self.state.oam_addr = (n << 2) | m;
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
            self.state.oam_end = true;
        }
        if self.state.secondary_oam_addr == 32 {
            self.state.secondary_oam_full = true;
            self.state.secondary_oam_addr = 0;
        }
        n
    }

    fn search_next_sprite(&mut self) -> bool {
        debug_assert!(self.state.cycle.is_multiple_of(2));

        if self.state.oam_end {
            return false;
        }

        let oam_value = self.state.oam_data_latch;
        let mut found = false;

        let mut m = self.state.oam_addr & 0x3;
        let mut n = self.state.oam_addr >> 2;

        if !self.state.secondary_oam_full {
            self.state.secondary_oam[self.state.secondary_oam_addr as usize] = oam_value;
        }
        if self.is_sprite_in_range_y(oam_value) {
            if self.state.secondary_oam_full {
                self.state.status.set_sprite_overflow(true);
            } else {
                found = true;
                self.state.sprite_in_range[(self.state.secondary_oam_addr >> 2) as usize] = true;
                // sprite zero is hit only if it's in range at cycle 66 (i.e. it's the first)
                // and only on visible scanlines. If this is the 239, the next is not visible
                if self.state.cycle == 66 && self.state.scanline < 239 {
                    self.state.sprite_zero_next = true;
                }
                (m, n) = self.incr(m, n);
                self.state.secondary_oam_addr += 1;
            }
        } else {
            n = self.incr_n(n);
            m = 0;
        }

        self.state.oam_addr = (n << 2) | m;

        found
    }

    fn is_sprite_in_range_y(&self, value: u8) -> bool {
        let sprite_y = value as i16;
        let sprite_height = self.state.ctrl.sprite_height() as i16;
        self.state.scanline >= sprite_y && self.state.scanline < sprite_y + sprite_height
    }

    fn cycle_load_data(&mut self, mapper: &mut dyn Mapper) {
        match self.state.cycle % 8 {
            // cycles 1-2: load nametable data
            2 => {
                let nt_address = 0x2000 | (self.state.v_reg.0 & 0x0FFF);
                self.state.bg_data.latch_nt = self.vram_read(nt_address, mapper);
            }
            // cycles 3-4: load attribute table data
            4 => {
                let at_address = 0x23C0
                    | ((self.state.v_reg.nametable_select() as u16) << 10)
                    | (((self.state.v_reg.coarse_y() as u16) >> 2) & 0x07) << 3
                    | (((self.state.v_reg.coarse_x() as u16) >> 2) & 0x07);
                let attribute = self.vram_read(at_address, mapper);

                let coarse_x = self.state.v_reg.coarse_x();
                let coarse_y = self.state.v_reg.coarse_y();
                let shift = ((coarse_y & 0x02) << 1) | (coarse_x & 0x02);
                self.state.bg_data.latch_at = (attribute >> shift) & 0x03;
            }
            // cycles 5-6: load bg low part
            6 => {
                let fine_y = self.state.v_reg.fine_y() as u16;
                let table_select = self.state.ctrl.bg_pattern_table() as u16;
                let address =
                    (table_select << 12) | ((self.state.bg_data.latch_nt as u16) << 4) | fine_y;
                self.state.bg_data.latch_pl = self.vram_read(address, mapper);
            }
            // cycles 7-8: load bg high part, then transfer everything in the shifters
            // then increment vram x scroll
            0 => {
                let end_y = self.state.v_reg.fine_y() as u16;

                let table_select = self.state.ctrl.bg_pattern_table() as u16;
                let address =
                    (table_select << 12) | ((self.state.bg_data.latch_nt as u16) << 4) | end_y | 8;
                self.state.bg_data.latch_ph = self.vram_read(address, mapper);
                self.state.bg_data.load_shifters();
                self.increment_vram_address_x();
            }
            _ => {}
        }
    }

    fn transfer_scroll_x(&mut self) {
        self.state.v_reg.set_unused(false);
        self.state.v_reg.set_coarse_x(self.state.t_reg.coarse_x());
        self.state
            .v_reg
            .set_nametable_select_x(self.state.t_reg.nametable_select_x());
    }

    fn transfer_scroll_y(&mut self) {
        self.state.v_reg.set_unused(false);
        self.state.v_reg.set_coarse_y(self.state.t_reg.coarse_y());
        self.state
            .v_reg
            .set_nametable_select_y(self.state.t_reg.nametable_select_y());
        self.state.v_reg.set_fine_y(self.state.t_reg.fine_y());
    }

    fn increase_cycle(&mut self) {
        self.state.cycle += 1;

        if self.state.cycle >= self.max_cycles_for_current_scanline() {
            self.state.cycle = 0;
            self.state.scanline += 1;

            if self.state.scanline == 261 {
                self.state.scanline = -1;
                self.state.frame_ready = true;
                self.state.frame += 1;
                self.state.is_odd_frame = !self.state.is_odd_frame;
                self.handle_open_bus_decay();
            }
        }
    }

    fn handle_open_bus_decay(&mut self) {
        if self.state.open_bus_decay_timer > 0 {
            self.state.open_bus_decay_timer -= 1;
            if self.state.open_bus_decay_timer == 0 {
                self.state.open_bus_value = 0;
            }
        }
    }

    pub fn cpu_write(&mut self, reg_index: u8, value: u8, mapper: &mut dyn Mapper) {
        if self.state.tracing_enabled {
            trace!("PPU CPU WRITE {reg_index:04X} <- {value:04X}");
        }
        self.write_open_bus(value);
        match reg_index {
            0 => {
                let old_nmi_enable = self.state.ctrl.vblank_nmi_enable();

                let ctrl = PpuCtrl(value);
                self.state.ctrl = ctrl;
                self.state.t_reg.set_nametable_select(value & 0x03);

                if !old_nmi_enable
                    && self.state.ctrl.vblank_nmi_enable()
                    && self.state.status.vblank_started()
                {
                    self.state.nmi_request = true;
                }
            }
            1 => {
                self.state.mask = PpuMask(value);
            }
            2 => {}
            3 => {
                self.state.oam_addr = value;
            }
            4 => {
                self.state.oam_data[self.state.oam_addr as usize] = value;
                self.state.oam_addr = self.state.oam_addr.wrapping_add(1);
            }
            5 => {
                if self.state.w_toggle {
                    self.state.t_reg.set_scroll_y(value);
                } else {
                    self.state.x_reg = value & 0x7;
                    self.state.t_reg.set_scroll_x(value);
                }
                self.state.w_toggle = !self.state.w_toggle;
            }
            6 => {
                if self.state.w_toggle {
                    self.state.t_reg.set_address_low(value);
                    // TODO delay this?
                    self.state.v_reg = self.state.t_reg;
                } else {
                    self.state.t_reg.set_address_high(value);
                }
                self.state.w_toggle = !self.state.w_toggle;
            }
            7 => {
                let current_addr = self.state.v_reg.address();

                self.vram_write(current_addr, value, mapper);

                let increment = if self.state.ctrl.vram_address_incr() {
                    32
                } else {
                    1
                };
                self.state.v_reg.increment_address(increment);
            }
            _ => unreachable!(),
        }
    }

    fn write_open_bus(&mut self, value: u8) {
        self.state.open_bus_value = value;
        self.state.open_bus_decay_timer = OPEN_BUS_DECAY_FRAMES;
    }

    pub fn cpu_read(&mut self, reg_index: u8, mapper: &dyn Mapper) -> u8 {
        let ret = match reg_index {
            2 => {
                let mut data = self.status_bits_shadow();

                if self.state.scanline == 241 {
                    // in this case the read happens before the vblank is actually set, so force
                    // it to false
                    if self.state.cycle == 1 {
                        data = data.set_bit(BI::_7, false);
                    }
                    // reading $2002 at this scanline skips the vblank for this frame
                    if self.state.cycle <= 1 {
                        self.state.suppress_vblank = true;
                    }
                } else {
                    if !(1..=3).contains(&self.state.cycle) {
                        self.state.nmi_request = false;
                    }
                }

                self.state.status.set_vblank_started(false);
                self.state.w_toggle = false;

                self.write_open_bus(data);
                data
            }
            4 => {
                if self.state.mask.rendering_enabled() && (2..=64).contains(&self.state.cycle) {
                    0xFF
                } else {
                    self.state.oam_data[self.state.oam_addr as usize]
                }
            }
            7 => {
                let mut data = self.vram_buffer_shadow(mapper);

                let current_addr = self.state.v_reg.address();

                // when reading through $2007, buffer the nametable at addr - 0x1000
                if current_addr >= 0x3F00 {
                    // when reading palette data, the upper two bits of the open bus are preserved
                    data = (data & 0x3F) | (self.state.open_bus_value & 0xC0);
                    self.state.data_buffer = self.vram_read(current_addr - 0x1000, mapper);
                } else {
                    self.state.data_buffer = self.vram_read(current_addr, mapper);
                }

                let increment = if self.state.ctrl.vram_address_incr() {
                    32
                } else {
                    1
                };

                self.state.v_reg.increment_address(increment);
                self.write_open_bus(data);
                data
            }
            _ => self.state.open_bus_value,
        };

        if self.state.tracing_enabled {
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
                self.state.nametables[idx]
            }
            0x3F00..=0x3FFF => {
                let palette_addr = normalize_palette_address(resolved_addr);
                self.state.palette_table[palette_addr]
            }
            _ => 0,
        };

        if self.state.tracing_enabled {
            trace!("PPU VRAM READ {resolved_addr:04X}({orig_addr:04X}) -> {ret:04X}");
        }

        ret
    }

    pub fn vram_write(&mut self, mut addr: u16, value: u8, mapper: &mut dyn Mapper) {
        let orig_addr = addr;
        addr &= 0x3FFF;

        if self.state.tracing_enabled {
            trace!("PPU VRAM WRITE {addr:04X}({orig_addr:04X}) <- {value:04X}");
        }

        match addr {
            0x0000..=0x1FFF => {
                mapper.store_ppu(addr, value);
            }
            0x2000..=0x3EFF => {
                let idx = mirror_nametable_addr(addr, mapper.mirroring_mode());
                self.state.nametables[idx] = value;
            }
            0x3F00..=0x3FFF => {
                let palette_addr = normalize_palette_address(addr);
                self.state.palette_table[palette_addr] = value;
            }
            _ => unreachable!(),
        }
    }

    fn render_pixel(&mut self, mapper: &dyn Mapper) {
        let x_pos = (self.state.cycle - 1) as usize;
        let y_pos = self.state.scanline as usize;

        let valid_bg_x = x_pos >= 8 || self.state.mask.show_background_leftmost();

        let (bg_pixel, bg_palette) = if self.state.mask.show_background() && valid_bg_x {
            let bit_mux = 0x8000 >> self.state.x_reg;
            self.state.bg_data.get_pixel_palette(bit_mux)
        } else {
            (0, 0)
        };

        let valid_sprite_x = x_pos >= 8 || self.state.mask.show_sprites_leftmost();

        let mut sprite_pixel = 0;
        let mut sprite_palette = 0;
        let mut sprite_behind = false;
        let mut sprite_zero = false;

        if self.state.mask.show_sprites() {
            for (i, sprite) in self.state.sprite_shifters.iter_mut().enumerate() {
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

        let render_type = match (bg_pixel, sprite_pixel) {
            (0, 0) => RenderType::NoRender,
            (0, _) => RenderType::Sprite,
            (_, 0) => RenderType::Background,
            _ => {
                if self.state.sprite_zero_current && sprite_zero {
                    let left_allowed = self.state.mask.show_background_leftmost()
                        && self.state.mask.show_sprites_leftmost();

                    let clip_left = if !left_allowed { x_pos < 8 } else { false };

                    // zero hit cannot happen on X=255
                    if self.state.mask.rendering_enabled() && !clip_left && x_pos < 255 {
                        self.state.status.set_sprite_zero_hit(true);
                    }
                }

                if sprite_behind {
                    RenderType::Background
                } else {
                    RenderType::Sprite
                }
            }
        };

        let (palette, pixel) = match render_type {
            RenderType::Background if self.state.debug_render_background_enabled => {
                (bg_palette, bg_pixel)
            }
            RenderType::Sprite if self.state.debug_render_sprites_enabled => {
                (sprite_palette, sprite_pixel)
            }
            _ => (0, 0),
        };

        let palette_addr = compute_palette_vram_address(palette, pixel);
        let color_index = self.vram_read(palette_addr, mapper);

        let rgb_color = read_palette_by_color_id(&self.state.mask, color_index);

        let zapper_hit = self.state.zapper_position.is_some_and(|(x, y)| {
            x_pos.abs_diff(x) <= ZAPPER_SENSOR_RADIUS
                && y_pos.abs_diff(y) <= ZAPPER_SENSOR_RADIUS
                && is_zapper_light(rgb_color)
        });
        if zapper_hit {
            self.state.zapper_light_timer = ZAPPER_LIGHT_PERSISTENCE;
        }

        self.renderer.render_pixel(x_pos, y_pos, rgb_color);
    }

    fn fill_sprite_data(&mut self, sprite_idx: usize, mapper: &mut dyn Mapper) {
        let addr = sprite_idx * 4;
        let oam_sprite = OamSprite::from_bytes(&self.state.secondary_oam[addr..addr + 4]);

        let tile_index = oam_sprite.tile;
        let attributes = oam_sprite.attr;

        let flip_vertical = attributes.vertical_flip();
        let flip_horizontal = attributes.horizontal_flip();

        let mut sprite_row = self.state.scanline - (oam_sprite.y as i16);

        if flip_vertical {
            let sprite_height = self.state.ctrl.sprite_height() as i16;
            sprite_row = (sprite_height - 1) - sprite_row;
        }

        let mut sprite_row = sprite_row as u16;

        let (table_base, actual_tile) = {
            if self.state.ctrl.sprite_size() {
                let table_base = ((tile_index & 0x01) as u16) << 12;
                let mut actual_tile = (tile_index & 0xFE) as u16;

                if sprite_row >= 8 {
                    actual_tile += 1;
                    sprite_row -= 8;
                }
                (table_base, actual_tile)
            } else {
                let table_base = if self.state.ctrl.sprite_pattern_table() {
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

        if !self.state.sprite_in_range[sprite_idx] {
            lo_shifter = 0;
            hi_shifter = 0;
        }

        // load the shifters only if the next scanline is visible, to avoid rendering of scanline
        // 240 on scanline 0
        if self.state.scanline < 239 {
            self.state.sprite_shifters[sprite_idx].load(oam_sprite, lo_shifter, hi_shifter);
        }
    }

    fn increment_vram_address_x(&mut self) {
        let coarse_x = self.state.v_reg.coarse_x();
        if coarse_x == 0x1F {
            // in this case, wraps the last 5 bits to 0
            self.state.v_reg.set_coarse_x(0);
            // switch the horizontal nametable
            self.state.v_reg.switch_nametable_x();
        } else {
            // increments the x part, that are the last 5 bits
            self.state.v_reg.set_coarse_x(coarse_x + 1);
        }
    }

    fn increment_vram_address_y(&mut self) {
        let fine_y = self.state.v_reg.fine_y();
        if fine_y < 7 {
            // if the y part is not maxed, simply increment it
            self.state.v_reg.set_fine_y(fine_y + 1);
        } else {
            // in this case, wrap the y bits to 0
            self.state.v_reg.set_fine_y(0);

            // compute coarse y
            let mut y = self.state.v_reg.coarse_y();

            if y == 29 {
                // clear coarse y
                y = 0;
                // switch the vertical nametable
                self.state.v_reg.switch_nametable_y();
            } else if y == 31 {
                // clear coarse y without switching
                y = 0;
            } else {
                // increment coarse y
                y += 1;
            }

            // put back coarse y
            self.state.v_reg.set_coarse_y(y);
        }
    }

    pub(crate) fn oam_data(&self) -> &[u8; 256] {
        &self.state.oam_data
    }

    pub(crate) fn secondary_oam_data(&self) -> &[u8; 32] {
        &self.state.secondary_oam
    }

    pub(crate) fn status_bits_shadow(&self) -> u8 {
        (self.state.status.0 & 0b1110_0000) | (self.state.open_bus_value & 0b0001_1111)
    }

    pub(crate) fn vram_buffer_shadow(&self, mapper: &dyn Mapper) -> u8 {
        let current_addr = self.state.v_reg.address();
        if current_addr >= 0x3F00 {
            self.vram_read(current_addr, mapper)
        } else {
            self.state.data_buffer
        }
    }

    pub fn toggle_sprite_render(&mut self) {
        self.state.debug_render_sprites_enabled = !self.state.debug_render_sprites_enabled;
    }

    pub fn toggle_background_render(&mut self) {
        self.state.debug_render_background_enabled = !self.state.debug_render_background_enabled;
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

    let bit_low = byte_low.extract_bits_shift(bit_shift, BC::_1);
    let bit_high = byte_high.extract_bits_shift(bit_shift, BC::_1);

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
    if mask.any_emphasis() {
        apply_emphasis(color, mask)
    } else {
        color
    }
}

fn is_zapper_light(rgba: u32) -> bool {
    let [r, g, b, _] = decompose_rgba(rgba);
    let r = r as u32;
    let g = g as u32;
    let b = b as u32;
    (r * 299 + g * 587 + b * 114) / 1000 >= ZAPPER_LIGHT_THRESHOLD
}

pub fn mirror_nametable_addr(addr: u16, mode: MirroringType) -> usize {
    let nametable = (addr >> 10) & 0x03;
    let offset = (addr & 0x03FF) as usize;
    let physical_nametable = match mode {
        MirroringType::Horizontal => nametable >> 1,
        MirroringType::Vertical => nametable & 0x01,
        MirroringType::SingleScreenLower => 0,
        MirroringType::SingleScreenUpper => 1,
    };

    (physical_nametable as usize) * 0x0400 + offset
}

#[inline(always)]
fn apply_emphasis(color: u32, mask: &PpuMask) -> u32 {
    let [r, g, b, a] = decompose_rgba(color);

    let mut r2 = r;
    let mut g2 = g;
    let mut b2 = b;

    if mask.emphasis_red() {
        r2 = emphasize_color(r2);
    }
    if mask.emphasis_green() {
        g2 = emphasize_color(g2);
    }
    if mask.emphasis_blue() {
        b2 = emphasize_color(b2);
    }

    compose_rgba(r2, g2, b2, a)
}

#[inline(always)]
fn emphasize_color(col: u8) -> u8 {
    col + 0x55
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
    use crate::{
        Mapper, MirroringType, NoopRenderer, Ppu,
        arch::memento::{
            MementoReadAndFill, MementoReader, MementoResult, MementoWriteable, MementoWriter,
        },
        utils::named::Named,
    };

    struct FakeMapper;

    impl Named for FakeMapper {
        fn name(&self) -> &str {
            "foo"
        }
    }

    impl Mapper for FakeMapper {
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

    impl MementoReadAndFill for FakeMapper {
        fn read_and_fill(&mut self, _data: &mut MementoReader) -> MementoResult {
            Ok(())
        }
    }

    impl MementoWriteable for FakeMapper {
        fn write_memento(&self, _writer: &mut dyn MementoWriter) -> MementoResult {
            Ok(())
        }
    }

    #[test]
    fn test_sequence() {
        let renderer = NoopRenderer;
        let mut ppu = Ppu::new(Box::new(renderer));

        ppu.state.t_reg = PpuScrollRegister(0b0111_1111_1111_1111);
        ppu.state.v_reg = PpuScrollRegister(0b0111_1111_1111_1111);
        ppu.state.x_reg = 0b1111_1111;
        ppu.state.w_toggle = true;

        let mut mapper = FakeMapper;

        ppu.cpu_write(0, 0, &mut mapper);

        assert_eq!(ppu.state.t_reg.0, 0b0111_0011_1111_1111);
        assert_eq!(ppu.state.v_reg.0, 0b0111_1111_1111_1111);
        assert_eq!(ppu.state.x_reg, 0b1111_1111);
        assert!(ppu.state.w_toggle);

        ppu.cpu_read(2, &mapper);

        assert_eq!(ppu.state.t_reg.0, 0b0111_0011_1111_1111);
        assert_eq!(ppu.state.v_reg.0, 0b0111_1111_1111_1111);
        assert_eq!(ppu.state.x_reg, 0b1111_1111);
        assert!(!ppu.state.w_toggle);

        ppu.cpu_write(5, 0b0111_1101, &mut mapper);

        assert_eq!(ppu.state.t_reg.0, 0b0111_0011_1110_1111);
        assert_eq!(ppu.state.v_reg.0, 0b0111_1111_1111_1111);
        assert_eq!(ppu.state.x_reg, 0b0000_0101);
        assert!(ppu.state.w_toggle);

        ppu.cpu_write(5, 0b0101_1110, &mut mapper);

        assert_eq!(ppu.state.t_reg.0, 0b0110_0001_0110_1111);
        assert_eq!(ppu.state.v_reg.0, 0b0111_1111_1111_1111);
        assert_eq!(ppu.state.x_reg, 0b0000_0101);
        assert!(!ppu.state.w_toggle);

        ppu.cpu_write(6, 0b0011_1101, &mut mapper);

        assert_eq!(ppu.state.t_reg.0, 0b0011_1101_0110_1111);
        assert_eq!(ppu.state.v_reg.0, 0b0111_1111_1111_1111);
        assert_eq!(ppu.state.x_reg, 0b0000_0101);
        assert!(ppu.state.w_toggle);

        ppu.cpu_write(6, 0b11110000, &mut mapper);

        assert_eq!(ppu.state.t_reg.0, 0b0011_1101_1111_0000);
        assert_eq!(ppu.state.v_reg.0, 0b0011_1101_1111_0000);
        assert_eq!(ppu.state.x_reg, 0b0000_0101);
        assert!(!ppu.state.w_toggle);
    }

    #[test]
    fn vram_scroll_increments_wrap_coarse_coordinates_and_toggle_nametables() {
        let mut ppu = Ppu::new(Box::new(NoopRenderer));
        ppu.state.v_reg.set_coarse_x(0x1F);
        ppu.state.v_reg.set_coarse_y(29);
        ppu.state.v_reg.set_nametable_select(1);
        ppu.state.v_reg.set_fine_y(7);

        ppu.increment_vram_address_x();
        assert_eq!(ppu.state.v_reg.coarse_x(), 0);
        assert_eq!(ppu.state.v_reg.nametable_select(), 0);

        ppu.increment_vram_address_y();
        assert_eq!(ppu.state.v_reg.fine_y(), 0);
        assert_eq!(ppu.state.v_reg.coarse_y(), 0);
        assert_eq!(ppu.state.v_reg.nametable_select(), 2);

        ppu.state.v_reg.set_coarse_y(31);
        ppu.state.v_reg.set_nametable_select(3);
        ppu.state.v_reg.set_fine_y(7);
        ppu.increment_vram_address_y();
        assert_eq!(ppu.state.v_reg.fine_y(), 0);
        assert_eq!(ppu.state.v_reg.coarse_y(), 0);
        assert_eq!(ppu.state.v_reg.nametable_select(), 3);
    }

    #[test]
    fn sprite_evaluation_copies_in_range_sprites_and_skips_empty_slots() {
        let mut ppu = Ppu::new(Box::new(NoopRenderer));
        let mut mapper = FakeMapper;

        ppu.state.scanline = 10;
        ppu.state.mask = PpuMask(0x10);
        ppu.state.oam_data[0] = 1;
        ppu.state.oam_data[4..8].copy_from_slice(&[8, 0x2A, 0x80, 0x34]);

        for _ in 0..=256 {
            ppu.tick(&mut mapper);
        }

        assert_eq!(&ppu.state.secondary_oam[..4], &[8, 0x2A, 0x80, 0x34]);

        for _ in 257..=320 {
            ppu.tick(&mut mapper);
        }

        assert_eq!(ppu.state.sprite_shifters[0].x, 0x34);
    }

    #[test]
    fn sprite_evaluation_flags_overflow_on_ninth_sprite() {
        let mut ppu = Ppu::new(Box::new(NoopRenderer));
        let mut mapper = FakeMapper;

        ppu.state.mask = PpuMask(0x10);
        ppu.state.scanline = 10;

        for sprite in 0..9 {
            ppu.state.oam_data[sprite * 4] = 8;
        }

        for _ in 0..=256 {
            ppu.tick(&mut mapper);
        }

        assert!(ppu.state.status.sprite_overflow());
        assert!(ppu.state.secondary_oam_full);
    }
}
