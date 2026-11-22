use std::{collections::HashSet, fs::File, sync::Arc};

use gilrs::{EventType, Gilrs};
use log::{error, info, warn};
use rustines_core::{self as core, MementoReadAndFill, MementoWriteable};
use winit::{
    event::{ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow},
    keyboard::KeyCode,
    window::{Window, WindowId},
};
use winit_input_helper::WinitInputHelper;

use crate::{
    AppEventLoopProxy, CustomEvents, Platform,
    app_structs::{AutoABState, SpeedupState},
    constants as C,
    fps_counter::FpsCounter,
    input::InputMap,
    pattern_window::PatternTableWindow,
    save_game::{load_save_state, save_game, save_state},
    states::{State, state::PlatformBox},
    types::KeyMap,
};
use web_time::{Duration, Instant};

pub struct RunningState {
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

    pub(crate) gilrs: Option<Gilrs>,
    pub(crate) platform: PlatformBox,
}

impl RunningState {
    pub fn main_window(&self) -> &Window {
        &self.main_window
    }

    pub fn pause(&mut self) {
        self.pause = true;
        self.accumulator = Duration::ZERO;
    }

    pub fn resume(&mut self) {
        self.pause = false;
        self.last_frame_time = Instant::now();
    }

    pub fn platform(&self) -> &dyn Platform {
        self.platform.as_ref()
    }

    fn fill_with_saved_state(&mut self, decoded: Vec<u8>) {
        let mut rollback_writer = core::MementoWriter::new();
        self.bus.write_memento(&mut rollback_writer);
        self.cpu.write_memento(&mut rollback_writer);
        let rollback_buffer = rollback_writer.into_vec();

        let mut reader = core::MementoReader::new(&decoded);
        let res = self.fill_data(&mut reader);

        match res {
            Ok(()) => {
                info!("State loaded successfully");
            }
            Err(e) => {
                error!("Load state failed: {e:?}");
                let mut rollback_reader = core::MementoReader::new(&rollback_buffer);
                self.fill_data(&mut rollback_reader)
                    .expect("Rollback failed");
            }
        }
    }

    fn save_game(&mut self) {
        save_game(
            &self.nes_rom,
            self.bus.mapper_ref(),
            self.platform.as_ref(),
            &self.proxy,
        );
    }

    fn fill_data(&mut self, reader: &mut core::MementoReader) -> core::MementoResult {
        self.bus.read_and_fill(reader)?;
        self.cpu.read_and_fill(reader)
    }

    fn update_logic(&mut self) {
        if !self.pause {
            let frame_to_run = if self.speedup.active {
                C::FRAMES_MULTIPLIER_SPEEDUP
            } else {
                1
            };
            for _ in 0..frame_to_run {
                self.update_counter.update();
                while !self.bus.ppu().frame_ready() {
                    self.cpu.tick(&mut self.bus);

                    self.auto_ab.process_tick(self.bus.controller1_mut());

                    // save if enough frames have passed
                    let current_frame = self.bus.ppu().current_frame();
                    if current_frame != self.last_save
                        && current_frame.is_multiple_of(C::TARGET_FPS * C::SAVE_INTERVAL)
                    {
                        self.save_game();
                        self.last_save = current_frame;
                    }
                }
                self.bus.ppu_mut().clear_frame_ready();
            }
        }
    }

    fn handle_inputs(&mut self, event_loop: &ActiveEventLoop) {
        let input = &mut self.main_window_helper;

        input.end_step();

        let esc_pressed = self.platform.esc_pressed_enabled() && input.key_pressed(KeyCode::Escape);

        if esc_pressed || input.close_requested() {
            self.save_game();
            if self.platform.should_exit() {
                event_loop.exit();
            }
        } else {
            self.handle_special_keys(event_loop);

            // reborrow
            let input = &self.main_window_helper;

            handle_input_keys(
                input,
                self.bus.controller1_mut(),
                self.input_map.key_map_1(),
            );
            if let Some(ctrl2) = self.bus.controller2_mut() {
                handle_input_keys(input, ctrl2, self.input_map.key_map_2());
            }

            if self.bus.zapper_mut().is_some() && self.zapper_needs_update {
                self.update_zapper_input();
            }
        }

        self.pattern_window.update();
    }

    fn handle_gamepad(&mut self) {
        if let Some(gilrs) = self.gilrs.as_mut() {
            let mut released = HashSet::new();
            let mut pressed = HashSet::new();
            let map = self.input_map.gamepad_map_1();

            while let Some(ev) = gilrs.next_event() {
                match ev.event {
                    EventType::ButtonPressed(b, _) => {
                        if let Some(v) = map.get(&b) {
                            pressed.insert(*v);
                        }
                    }
                    EventType::ButtonReleased(b, _) => {
                        if let Some(v) = map.get(&b) {
                            released.insert(*v);
                        }
                    }
                    // TODO: Handle axis events for analog sticks
                    _ => {}
                }
            }

            let ctrl = self.bus.controller1_mut();
            for r in released.difference(&pressed) {
                ctrl.released(*r);
            }
            for p in pressed {
                ctrl.pressed(p);
            }
        }
    }

    pub(crate) fn handle_special_keys(&mut self, event_loop: &ActiveEventLoop) {
        // keep this enabled even when debug keys are disabled
        self.speedup
            .update(self.main_window_helper.key_held(KeyCode::Backquote));

        if self.enable_debug_keys {
            self.handle_debug_keys(event_loop);
        }

        if self.main_window_helper.key_pressed(KeyCode::F5) {
            save_state(
                &self.nes_rom,
                &self.bus,
                &self.cpu,
                &self.proxy,
                self.platform(),
            );
        }
        if self.main_window_helper.key_pressed(KeyCode::F9) {
            load_save_state(&self.nes_rom, &self.proxy, self.platform());
        }

        if self.main_window_helper.held_shift() {
            if self.main_window_helper.key_pressed(KeyCode::F12) {
                self.auto_ab.toggle();
                if self.auto_ab.active {
                    self.display_message("Auto-A/B mode enabled");
                } else {
                    self.display_message("Auto-A/B mode disabled");
                }
            }

            if self.main_window_helper.key_pressed(KeyCode::Digit1) {
                let enabled = self.bus.apu_mut().mixer_mut().pulse1().toggle();
                self.display_message(&format!(
                    "{} pulse 1",
                    if enabled { "Enabled" } else { "Disabled" }
                ));
            }
            if self.main_window_helper.key_pressed(KeyCode::Digit2) {
                let enabled = self.bus.apu_mut().mixer_mut().pulse2().toggle();
                self.display_message(&format!(
                    "{} pulse 2",
                    if enabled { "Enabled" } else { "Disabled" }
                ));
            }
            if self.main_window_helper.key_pressed(KeyCode::Digit3) {
                let enabled = self.bus.apu_mut().mixer_mut().triangle().toggle();
                self.display_message(&format!(
                    "{} triangle",
                    if enabled { "Enabled" } else { "Disabled" }
                ));
            }
            if self.main_window_helper.key_pressed(KeyCode::Digit4) {
                let enabled = self.bus.apu_mut().mixer_mut().noise().toggle();

                self.display_message(&format!(
                    "{} noise",
                    if enabled { "Enabled" } else { "Disabled" }
                ));
            }
            if self.main_window_helper.key_pressed(KeyCode::Digit5) {
                let enabled = self.bus.apu_mut().mixer_mut().dmc().toggle();
                self.display_message(&format!(
                    "{} dmc",
                    if enabled { "Enabled" } else { "Disabled" }
                ));
            }
            if self.main_window_helper.key_pressed(KeyCode::Digit0) {
                self.bus.apu_mut().mixer_mut().enable_all();
                self.display_message("Enabled all audio channels");
            }

            if self.main_window_helper.key_pressed(KeyCode::Digit6) {
                let enabled = self.bus.ppu_mut().toggle_sprite_render();
                self.display_message(&format!(
                    "{} sprite rendering",
                    if enabled { "Enabled" } else { "Disabled" }
                ));
            }
            if self.main_window_helper.key_pressed(KeyCode::Digit7) {
                let enabled = self.bus.ppu_mut().toggle_background_render();
                self.display_message(&format!(
                    "{} background rendering",
                    if enabled { "Enabled" } else { "Disabled" }
                ));
            }

            if self.main_window_helper.key_pressed(KeyCode::KeyR) {
                self.bus.reset();
                self.cpu.reset();
                self.display_message("Resetting");
            }
        }
    }

    fn display_message(&mut self, msg: &str) {
        self.bus.ppu_mut().renderer_mut().display_message(
            msg,
            C::MESSAGE_SECONDS,
            C::MESSAGE_FONT_SIZE,
            C::MESSAGE_LOCATION,
        );
    }

    fn handle_debug_keys(&mut self, event_loop: &ActiveEventLoop) {
        if self.main_window_helper.held_shift() {
            if self.main_window_helper.key_pressed(KeyCode::KeyX) {
                if self.pause {
                    self.resume();
                    self.display_message("Resumed");
                } else {
                    self.pause();
                    self.display_message("Paused");
                }
            }

            if self.main_window_helper.key_pressed(KeyCode::KeyD) {
                core::debug_utils::debug_dump_nametables(&self.bus);
                core::debug_utils::debug_dump_attribute_tables(&self.bus);
                self.display_message("Dumped NT + AT");
            }

            if self.main_window_helper.key_pressed(KeyCode::KeyP) {
                core::debug_utils::debug_dump_palette(&self.bus);
                self.display_message("Dumped palette");
            }

            if self.main_window_helper.key_pressed(KeyCode::KeyO) {
                core::debug_utils::debug_dump_oam(self.bus.ppu());
                self.display_message("Dumped OAM");
            }

            if self.main_window_helper.key_pressed(KeyCode::KeyQ) {
                core::debug_utils::debug_dump_state(&self.bus, &self.cpu);
                self.display_message("Dumped state");
            }

            if self.main_window_helper.key_pressed(KeyCode::KeyF) {
                let mut file = File::create("frame.ppm").expect("cannot create file");
                self.bus
                    .ppu()
                    .renderer()
                    .dump_frame(&mut file)
                    .expect("cannot dump frame");
                self.display_message("Wrote frame to file");
            }

            if self.main_window_helper.key_pressed(KeyCode::KeyT) {
                let log_point = self.log_point;
                println!("LOG_POINT {}", log_point);
                info!("LOG_POINT {}", log_point);

                self.log_point += 1;

                self.cpu.enable_tracing(true);
                self.bus.enable_tracing(true);
                self.bus.ppu_mut().enable_tracing(true);

                self.display_message("Set logpoint");
            }

            if self.main_window_helper.key_pressed(KeyCode::KeyS) {
                self.pattern_window
                    .show(self.bus.mapper_ref(), event_loop, 4);

                self.display_message("Displayed pattern window");
            }
        }
    }

    fn update_zapper_input(&mut self) {
        let size = self.main_window.inner_size();
        let position = self.cursor_position.and_then(|(x, y)| {
            (size.width > 0 && size.height > 0).then(|| {
                (
                    (x * f64::from(C::INNER_SIZE.width) / f64::from(size.width)) as usize,
                    (y * f64::from(C::INNER_SIZE.height) / f64::from(size.height)) as usize,
                )
            })
        });

        self.bus
            .set_zapper_input(position, self.zapper_trigger_pressed);
    }
}

impl State for RunningState {
    fn new_events(mut self: Box<Self>, _: &ActiveEventLoop) -> Box<dyn State> {
        self.main_window_helper.step();
        self.pattern_window.step();
        self
    }

    fn window_event(
        mut self: Box<Self>,
        _event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) -> Box<dyn State> {
        if window_id == self.main_window.id() {
            if self.bus.zapper_mut().is_some() {
                match &event {
                    WindowEvent::CursorMoved { position, .. } => {
                        self.cursor_position = Some((position.x, position.y));
                        self.zapper_needs_update = true;
                    }
                    WindowEvent::CursorLeft { .. } => {
                        self.cursor_position = None;
                        self.zapper_needs_update = true;
                    }
                    WindowEvent::MouseInput {
                        state,
                        button: MouseButton::Left,
                        ..
                    } => {
                        self.zapper_trigger_pressed = *state == ElementState::Pressed;
                        self.zapper_needs_update = true;
                    }
                    _ => {}
                }
            }

            if self.main_window_helper.process_window_event(&event) {
                // check how much time has passed since last frame
                let now = Instant::now();
                let dt = now.duration_since(self.last_frame_time);
                self.last_frame_time = now;
                self.accumulator += dt;

                // Draw the current frame
                self.bus.ppu_mut().renderer_mut().draw();

                self.render_counter.update();
                self.platform.set_title(
                    self.main_window(),
                    &self.rom_name,
                    self.render_counter.current_fps(),
                    self.update_counter.current_fps(),
                );
            }
        } else if self.pattern_window.owns_window_event(window_id) {
            self.pattern_window.window_event(&event);
        }

        self
    }

    fn user_event(
        mut self: Box<Self>,
        _event_loop: &ActiveEventLoop,
        event: CustomEvents,
    ) -> Box<dyn State> {
        match event {
            CustomEvents::Pause => {
                self.pause();
                self.save_game();
            }
            CustomEvents::Resume => {
                self.resume();
            }
            CustomEvents::SaveGameRead(c) => match c {
                Some(content) => match zstd::decode_all(content.as_slice()) {
                    Ok(decoded) => {
                        info!("Read save file");
                        self.bus.mapper_mut().set_prg_ram(&decoded);
                    }
                    Err(e) => error!("Load save decompress failed: {e}"),
                },
                None => error!("Load state failed"),
            },
            CustomEvents::SaveStateRead(c) => match c {
                Some(content) => match zstd::decode_all(content.as_slice()) {
                    Ok(decoded) => {
                        self.display_message("Loaded state");
                        self.fill_with_saved_state(decoded);
                    }
                    Err(e) => {
                        self.display_message("Load state decompress failed");
                        error!("Load state decompress failed: {e}")
                    }
                },
                None => {
                    self.display_message("Load state failed");
                    error!("Load state failed")
                }
            },
            CustomEvents::SaveStateWritten(..) => {
                self.display_message("Saved state");
            }
            CustomEvents::SaveGameWritten(..) => {
                info!("Wrote save file");
            }
            CustomEvents::ConfigWritten(..) => info!("Saved config"),
            _ => {
                warn!("Unexpected event during runtime: {:?}", event);
            }
        }
        self
    }

    fn about_to_wait(mut self: Box<Self>, event_loop: &ActiveEventLoop) -> Box<dyn State> {
        self.handle_inputs(event_loop);
        self.handle_gamepad();

        // run as many updates as possible in the delta that has passed
        while self.accumulator >= self.frame_duration {
            self.update_logic();
            self.accumulator -= self.frame_duration;
        }

        // request redraw for next
        self.main_window.request_redraw();
        self.pattern_window.render();

        event_loop.set_control_flow(ControlFlow::Poll);

        self
    }

    fn resumed(self: Box<Self>, _event_loop: &ActiveEventLoop) -> Box<dyn State> {
        self
    }
}

fn handle_input_keys(input: &WinitInputHelper, ctrl: &mut core::NesController, key_map: &KeyMap) {
    for (k, v) in key_map.iter() {
        if input.key_pressed(*k) {
            ctrl.pressed(*v);
        }
        if input.key_released(*k) {
            ctrl.released(*v);
        }
    }
}
