use crate::{
    AppState, CpalOutput, Platform,
    app_state::{AppStateInfo, SpeedupState},
    constants::{INNER_SIZE, width_to_size},
    fps_counter::FpsCounter,
    input::{self, handle_input_keys},
    pattern_window::PatternTableWindow,
    platform::PendingRenderer,
    save_game::{load_save_game, save_game},
    types::AppEventLoopProxy,
};
use gilrs::{EventType, Gilrs};
use log::info;

use rustines_core::{
    self as core,
    arch::memento::{MementoReadAndFill, MementoReader},
};
use std::{cell::RefCell, collections::HashSet, path::PathBuf, rc::Rc, sync::Arc};

use web_time::{Duration, Instant};

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{ElementState, MouseButton, StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow},
    keyboard::KeyCode,
    window::{Window, WindowAttributes, WindowId},
};
use winit_input_helper::WinitInputHelper;

const TARGET_FPS: u64 = 60;
const SAVE_INTERVAL: u64 = 10;

#[derive(PartialEq, Eq)]
pub enum CustomEvents {
    Pause,
    Resume,
    LoadedPrgRam(Option<Vec<u8>>),
    LoadedState(Option<Vec<u8>>),
    SavedPrgRam(PathBuf, bool),
    SavedState(PathBuf, bool),
}

pub struct App<P> {
    app_state: Option<AppState>,
    info: Option<AppStateInfo>,
    trace_boot: bool,
    zapper: bool,
    pending_window: Option<Arc<Window>>,
    pending_renderer: PendingRenderer,
    platform: P,
    size: PhysicalSize<u32>,
    gilrs: Gilrs,
    enable_debug_keys: bool,
    proxy: AppEventLoopProxy,
}

impl<P: Platform> App<P> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        rom_name: String,
        nes_rom: core::NesRom,
        mapper: core::MapperBox,
        trace_boot: bool,
        zapper: bool,
        width: u32,
        platform: P,
        enable_debug_keys: bool,
        proxy: AppEventLoopProxy,
    ) -> Self {
        App {
            info: Some(AppStateInfo {
                mapper,
                nes_rom,
                rom_name,
            }),
            app_state: None,
            trace_boot,
            zapper,
            platform,
            size: width_to_size(width),
            pending_window: None,
            pending_renderer: Rc::new(RefCell::new(None)),
            gilrs: Gilrs::new().unwrap(),
            enable_debug_keys,
            proxy,
        }
    }

    fn init_app_state(
        &mut self,
        main_window: Arc<Window>,
        renderer: core::RendererBox,
    ) -> AppState {
        let ppu = core::Ppu::new(renderer);

        let audio: core::AudioOutputBox = match CpalOutput::new() {
            Ok(mut cpal) => {
                cpal.play().expect("Can't play");
                Box::new(cpal)
            }
            Err(e) => {
                eprintln!("Cannot initialize audio: {}", e);
                Box::new(core::NoAudio)
            }
        };

        let apu = core::Apu::new(audio);

        let ctrl2 = if self.zapper {
            core::Controller2::zapper()
        } else {
            core::Controller2::nes_controller()
        };

        let info = self.info.take().unwrap();
        load_save_game(
            &info.nes_rom,
            info.mapper.as_ref(),
            &self.platform,
            &self.proxy,
        );

        let mut bus = core::Bus::new(info.mapper, ppu, apu, ctrl2);
        let mut cpu = core::Cpu::new();

        if self.trace_boot {
            cpu.enable_tracing(true);
            bus.enable_tracing(true);
            bus.ppu_mut().enable_tracing(true);
        }

        AppState {
            rom_name: info.rom_name,
            nes_rom: info.nes_rom,
            bus,
            render_counter: FpsCounter::new(),
            update_counter: FpsCounter::new(),
            cpu,
            key_map1: input::build_keymap_c1(),
            key_map2: input::build_keymap_c2(),
            gamepad_map1: input::build_gamepadmap_c1(),
            log_point: 1,
            pattern_window: PatternTableWindow::new(),
            main_window,
            pause: false,
            main_window_helper: WinitInputHelper::new(),
            cursor_position: None,
            zapper_trigger_pressed: false,
            zapper_needs_update: false,
            frame_duration: Duration::from_nanos(1_000_000_000 / TARGET_FPS),
            speedup: SpeedupState::default(),
            enable_debug_keys: self.enable_debug_keys,
            accumulator: Duration::ZERO,
            last_frame_time: Instant::now(),
            last_save: 0,
            proxy: self.proxy.clone(),
        }
    }
}

impl<P: Platform> ApplicationHandler<CustomEvents> for App<P> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.app_state.is_some() {
            return;
        }

        let main_window = Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title("Rustines")
                        .with_inner_size(self.size)
                        .with_min_inner_size(self.size),
                )
                .unwrap(),
        );

        self.platform
            .init_window_hook(Arc::clone(&main_window), self.size);

        self.pending_window = Some(Arc::clone(&main_window));

        self.platform.init_pending_renderer(
            Arc::clone(&main_window),
            Rc::clone(&self.pending_renderer),
            self.size,
        );
    }

    fn new_events(&mut self, _: &ActiveEventLoop, _: StartCause) {
        if let Some(app_state) = self.app_state.as_mut() {
            app_state.main_window_helper.step();
            app_state.pattern_window.step();
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(app_state) = self.app_state.as_mut() {
            if window_id == app_state.main_window.id() {
                if app_state.bus.zapper_mut().is_some() {
                    match &event {
                        WindowEvent::CursorMoved { position, .. } => {
                            app_state.cursor_position = Some((position.x, position.y));
                            app_state.zapper_needs_update = true;
                        }
                        WindowEvent::CursorLeft { .. } => {
                            app_state.cursor_position = None;
                            app_state.zapper_needs_update = true;
                        }
                        WindowEvent::MouseInput {
                            state,
                            button: MouseButton::Left,
                            ..
                        } => {
                            app_state.zapper_trigger_pressed = *state == ElementState::Pressed;
                            app_state.zapper_needs_update = true;
                        }
                        _ => {}
                    }
                }

                if app_state.main_window_helper.process_window_event(&event) {
                    // check how much time has passed since last frame
                    let now = Instant::now();
                    let dt = now.duration_since(app_state.last_frame_time);
                    app_state.last_frame_time = now;
                    app_state.accumulator += dt;

                    // Draw the current frame
                    app_state.bus.ppu_mut().renderer_mut().draw();

                    app_state.render_counter.update();
                    self.platform.set_title(app_state);
                }
            } else if app_state.pattern_window.owns_window_event(window_id) {
                app_state.pattern_window.window_event(&event);
            }
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: CustomEvents) {
        if let Some(app_state) = self.app_state.as_mut() {
            match event {
                CustomEvents::Pause => {
                    app_state.pause();
                    save_game(
                        &app_state.nes_rom,
                        app_state.bus.mapper_ref(),
                        &self.platform,
                        &app_state.proxy,
                    );
                }
                CustomEvents::Resume => {
                    app_state.resume();
                }
                CustomEvents::LoadedPrgRam(c) => match c {
                    Some(content) => match zstd::decode_all(content.as_slice()) {
                        Ok(decoded) => {
                            info!("Read save file");
                            app_state.bus.mapper_mut().set_prg_ram(&decoded);
                        }
                        Err(e) => info!("Load save decompress failed: {e}"),
                    },
                    None => info!("Load state failed"),
                },
                CustomEvents::LoadedState(c) => match c {
                    Some(content) => match zstd::decode_all(content.as_slice()) {
                        Ok(decoded) => {
                            info!("Read state");
                            let mut reader = MementoReader::new(&decoded);
                            app_state
                                .bus
                                .read_and_fill(&mut reader)
                                .expect("Failed to read saved state");
                            app_state
                                .cpu
                                .read_and_fill(&mut reader)
                                .expect("Failed to read saved state");
                        }
                        Err(e) => info!("Load state decompress failed: {e}"),
                    },
                    None => info!("Load state failed"),
                },
                CustomEvents::SavedState(..) => {
                    info!("Wrote state");
                }
                CustomEvents::SavedPrgRam(..) => {
                    info!("Wrote save file");
                }
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        match self.app_state.as_mut() {
            None => {
                let renderer = self.pending_renderer.borrow_mut().take();
                if let Some(renderer) = renderer {
                    let window = self.pending_window.take().unwrap();
                    self.app_state = Some(self.init_app_state(window, renderer));
                }
            }
            Some(app_state) => {
                handle_inputs(app_state, &self.platform, event_loop);
                handle_gamepad(app_state, &mut self.gilrs);

                // run as many updates as possible in the delta that has passed
                while app_state.accumulator >= app_state.frame_duration {
                    update_logic(app_state, &self.platform);
                    app_state.accumulator -= app_state.frame_duration;
                }

                // request redraw for next
                app_state.main_window.request_redraw();
                app_state.pattern_window.render();
            }
        }

        event_loop.set_control_flow(ControlFlow::Poll);
    }
}

fn update_logic(app_state: &mut AppState, platform: &impl Platform) {
    if !app_state.pause {
        let frame_to_run = if app_state.speedup.active { 4 } else { 1 };
        for _ in 0..frame_to_run {
            app_state.update_counter.update();
            while !app_state.bus.ppu().frame_ready() {
                app_state.cpu.tick(&mut app_state.bus);

                // save if enough frames have passed
                let current_frame = app_state.bus.ppu().current_frame();
                if current_frame != app_state.last_save
                    && current_frame.is_multiple_of(TARGET_FPS * SAVE_INTERVAL)
                {
                    save_game(
                        &app_state.nes_rom,
                        app_state.bus.mapper_ref(),
                        platform,
                        &app_state.proxy,
                    );
                    app_state.last_save = current_frame;
                }
            }
            app_state.bus.ppu_mut().clear_frame_ready();
        }
    }
}

fn handle_inputs(app_state: &mut AppState, platform: &impl Platform, event_loop: &ActiveEventLoop) {
    let input = &mut app_state.main_window_helper;

    input.end_step();

    let esc_pressed = platform.esc_pressed_enabled() && input.key_pressed(KeyCode::Escape);

    if esc_pressed || input.close_requested() {
        save_game(
            &app_state.nes_rom,
            app_state.bus.mapper_ref(),
            platform,
            &app_state.proxy,
        );
        if platform.should_exit() {
            event_loop.exit();
        }
    } else {
        input::handle_special_keys(app_state, event_loop, platform);

        // reborrow
        let input = &app_state.main_window_helper;

        handle_input_keys(input, app_state.bus.controller1_mut(), &app_state.key_map1);
        if let Some(ctrl2) = app_state.bus.controller2_mut() {
            handle_input_keys(input, ctrl2, &app_state.key_map2);
        }

        if app_state.bus.zapper_mut().is_some() && app_state.zapper_needs_update {
            update_zapper_input(app_state);
        }
    }

    app_state.pattern_window.update();
}

fn handle_gamepad(app_state: &mut AppState, gilrs: &mut Gilrs) {
    let mut released = HashSet::new();
    let mut pressed = HashSet::new();
    let map = &app_state.gamepad_map1;

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
            // for now let's ignore axis
            _ => {}
        }
    }

    let ctrl = app_state.bus.controller1_mut();
    for r in released.difference(&pressed) {
        ctrl.released(*r);
    }
    for p in pressed {
        ctrl.pressed(p);
    }
}

fn update_zapper_input(app_state: &mut AppState) {
    let size = app_state.main_window.inner_size();
    let position = app_state.cursor_position.and_then(|(x, y)| {
        (size.width > 0 && size.height > 0).then(|| {
            (
                (x * f64::from(INNER_SIZE.width) / f64::from(size.width)) as usize,
                (y * f64::from(INNER_SIZE.height) / f64::from(size.height)) as usize,
            )
        })
    });

    app_state
        .bus
        .set_zapper_input(position, app_state.zapper_trigger_pressed);
}
