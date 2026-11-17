use std::sync::Arc;

use rustines_core::{self as core};
use winit::window::Window;
use winit_input_helper::WinitInputHelper;

use crate::{
    fps_counter::FpsCounter, input::InputMap, pattern_window::PatternTableWindow,
    types::AppEventLoopProxy,
};
use web_time::{Duration, Instant};

#[derive(Default)]
pub(crate) struct SpeedupState {
    pub(crate) active: bool,
    changed: bool,
}

impl SpeedupState {
    pub(crate) fn update(&mut self, active: bool) {
        self.changed = active != self.active;
        self.active = active;
    }
}

pub struct AppState {
    pub(crate) rom_name: String,
    pub(crate) nes_rom: core::NesRom,

    pub(crate) bus: core::Bus,
    pub(crate) cpu: core::Cpu,

    pub(crate) render_counter: FpsCounter,
    pub(crate) update_counter: FpsCounter,
    pub(crate) log_point: u32,

    pub(crate) input_map: InputMap,

    pub(crate) pattern_window: PatternTableWindow,

    pub(crate) main_window: Arc<Window>,
    pub(crate) main_window_helper: WinitInputHelper,

    pub(crate) cursor_position: Option<(f64, f64)>,
    pub(crate) zapper_trigger_pressed: bool,
    pub(crate) zapper_needs_update: bool,
    pub(crate) enable_debug_keys: bool,

    pub(crate) pause: bool,
    pub(crate) frame_duration: Duration,
    pub(crate) speedup: SpeedupState,
    pub(crate) last_frame_time: Instant,
    pub(crate) accumulator: Duration,
    pub(crate) last_save: u64,
    pub(crate) proxy: AppEventLoopProxy,
}

impl AppState {
    pub fn main_window(&self) -> &Window {
        &self.main_window
    }

    pub fn rom_name(&self) -> &str {
        &self.rom_name
    }

    pub fn render_fps(&self) -> f64 {
        self.render_counter.current_fps()
    }

    pub fn update_fps(&self) -> f64 {
        self.update_counter.current_fps()
    }

    pub fn pause(&mut self) {
        self.pause = true;
        self.accumulator = Duration::ZERO;
    }

    pub fn resume(&mut self) {
        self.pause = false;
        self.last_frame_time = Instant::now();
    }
}

pub(crate) struct AppStateInfo {
    pub(crate) nes_rom: core::NesRom,
    pub(crate) mapper: core::MapperBox,
    pub(crate) rom_name: String,
}
