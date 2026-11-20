use std::sync::Arc;

use rustines_core as core;
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

pub(crate) struct AutoABState {
    pub(crate) active: bool,
    pressed: bool,
    counter: u32,
    interval: u32,
}

impl AutoABState {
    pub(crate) fn new(interval: u32) -> Self {
        AutoABState {
            active: false,
            pressed: false,
            counter: 0,
            interval,
        }
    }

    pub(crate) fn toggle(&mut self) {
        self.active = !self.active;
        self.pressed = self.active;
        self.counter = 0;
    }

    pub(crate) fn process_tick(&mut self, controller: &mut core::NesController) {
        if !self.active {
            return;
        }

        self.counter += 1;

        if self.counter >= self.interval {
            self.counter = 0;
            self.pressed = !self.pressed;

            if self.pressed {
                controller.released(core::NesKey::A);
                controller.released(core::NesKey::B);
            } else {
                controller.pressed(core::NesKey::A);
                controller.pressed(core::NesKey::B);
            }
        }
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
    pub(crate) auto_ab: AutoABState,
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

#[cfg(test)]
mod tests {
    use rustines_core::{NesController, NesKey};

    use super::AutoABState;

    #[test]
    fn test_ab() {
        let mut ctrl = NesController::new(1);
        let mut ab = AutoABState::new(2);
        assert_eq!(ab.counter, 0);
        assert!(!ab.pressed);
        assert!(!ab.active);

        for _ in 0..3 {
            ab.process_tick(&mut ctrl);
            assert_eq!(ctrl.buttons(), &[false; 8]);
        }

        assert_eq!(ab.counter, 0);
        assert!(!ab.pressed);
        assert!(!ab.active);

        ab.toggle();

        assert_eq!(ab.counter, 0);
        assert!(ab.active);
        assert!(ab.pressed);

        ab.process_tick(&mut ctrl);

        assert_eq!(ctrl.buttons(), &[false; 8]);
        assert_eq!(ab.counter, 1);
        assert!(ab.active);
        assert!(ab.pressed);

        ab.process_tick(&mut ctrl);

        let mut exp = [false; 8];
        exp[NesKey::A as usize] = true;
        exp[NesKey::B as usize] = true;
        assert_eq!(ctrl.buttons(), &exp);
        assert_eq!(ab.counter, 0);
        assert!(ab.active);
        assert!(!ab.pressed);

        ab.toggle();

        assert_eq!(ab.counter, 0);
        assert!(!ab.pressed);
        assert!(!ab.active);
    }
}
