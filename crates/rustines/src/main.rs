mod args;
mod pattern_window;
mod renderer;
mod utils;

use crate::{
    args::RustinesArgs,
    pattern_window::PatternTableWindow,
    renderer::PixelsRenderer,
    utils::{init_logger, read_file},
};
use clap::Parser;
use log::info;
use pixels::{Pixels, ScalingMode, SurfaceTexture};
use rustines_core as core;
use rustines_gui_utils::{FpsCounter, FpsLimiter};
use std::{collections::HashMap, path, sync::Arc};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::KeyCode,
    window::{Window, WindowAttributes},
};
use winit_input_helper::WinitInputHelper;

const INNER_W: u32 = 256;
const INNER_H: u32 = 240;
const RATIO: f64 = (INNER_H as f64) / (INNER_W as f64);
const WIDTH: u32 = 1024;
const HEIGHT: u32 = ((WIDTH as f64) * RATIO) as u32;

type KeyMap = HashMap<KeyCode, rustines_core::NesKey>;

struct AppState {
    bus: core::Bus,
    cpu: core::Cpu,

    limiter: FpsLimiter,
    counter: FpsCounter,
    log_point: u32,

    key_map1: KeyMap,
    key_map2: KeyMap,

    pattern_window: PatternTableWindow,

    main_window: Arc<Window>,
    main_window_helper: WinitInputHelper,

    pause: bool,
}

struct App {
    app_state: Option<AppState>,
    mapper: Option<core::MapperBox>,
    trace_boot: bool,
}

impl App {
    fn new(mapper: core::MapperBox, trace_boot: bool) -> Self {
        App {
            app_state: None,
            mapper: Some(mapper),
            trace_boot,
        }
    }

    fn init_app_state(&mut self, main_window: Arc<Window>, renderer: PixelsRenderer) -> AppState {
        let ppu = core::Ppu::new(Box::new(renderer));
        let apu = core::Apu::default();

        let mut bus = core::Bus::new(self.mapper.take().unwrap(), ppu, apu);
        let mut cpu = core::Cpu::new();

        if self.trace_boot {
            cpu.enable_tracing(true);
            bus.enable_tracing(true);
        }

        AppState {
            bus,
            counter: FpsCounter::new(),
            cpu,
            key_map1: build_keymap_c1(),
            key_map2: build_keymap_c2(),
            limiter: FpsLimiter::new(60.0),
            log_point: 1,
            pattern_window: PatternTableWindow::new(),
            main_window,
            pause: false,
            main_window_helper: WinitInputHelper::new(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let main_window = Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title("Rustines")
                        .with_inner_size(size)
                        .with_min_inner_size(size),
                )
                .unwrap(),
        );

        let renderer = create_renderer(Arc::clone(&main_window)).unwrap();

        self.app_state = Some(self.init_app_state(main_window, renderer));
    }

    fn new_events(&mut self, _: &ActiveEventLoop, _: StartCause) {
        if let Some(app_state) = self.app_state.as_mut() {
            app_state.main_window_helper.step();
            app_state.pattern_window.step();
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let Some(app_state) = self.app_state.as_mut() {
            if window_id == app_state.main_window.id() {
                if app_state.main_window_helper.process_window_event(&event) {
                    // Draw the current frame
                    app_state.bus.ppu_mut().renderer().draw();

                    if let Some(fps) = app_state.counter.drawn() {
                        app_state
                            .main_window
                            .set_title(&format!("Rustines | FPS: {:.1}", fps));
                    }
                }
            } else if app_state.pattern_window.owns_window_event(window_id) {
                app_state.pattern_window.window_event(&event);
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(app_state) = self.app_state.as_mut() {
            update_logic(app_state, event_loop);
        }
    }
}

pub fn main() {
    let args = RustinesArgs::parse();

    let _logger_handle = init_logger(args.log_file, args.trace_level);

    let file_path = path::PathBuf::from(&args.file_path);

    info!("Using input file: {}", args.file_path);

    let (_, mapper) = read_file(&file_path).unwrap();

    let event_loop = EventLoop::new().unwrap();

    let mut app = App::new(mapper, args.trace_boot);

    let _ = event_loop.run_app(&mut app);
}

fn update_logic(app_state: &mut AppState, event_loop: &ActiveEventLoop) {
    let input = &mut app_state.main_window_helper;

    input.end_step();

    if input.key_pressed(KeyCode::Escape) || input.close_requested() {
        event_loop.exit();
    } else {
        map_debug_keys(app_state, event_loop);

        // reborrow
        let input = &app_state.main_window_helper;

        let bus = &mut app_state.bus;

        map_inputs(input, bus.controller1_mut(), &app_state.key_map1);
        map_inputs(input, bus.controller2_mut(), &app_state.key_map2);

        if !app_state.pause {
            while !bus.ppu_mut().frame_ready() {
                app_state.cpu.tick(bus);
            }
            bus.ppu_mut().clear_frame_ready();
            app_state.limiter.update();
        }

        app_state.main_window.request_redraw();
    }

    app_state.pattern_window.update();
}

fn map_debug_keys(app_state: &mut AppState, event_loop: &ActiveEventLoop) {
    let input = &app_state.main_window_helper;

    let bus = &mut app_state.bus;
    let cpu = &mut app_state.cpu;

    if input.held_shift() {
        if input.key_pressed(KeyCode::KeyD) {
            core::debug_utils::debug_dump_nametables(bus);
        }

        if input.key_pressed(KeyCode::KeyP) {
            core::debug_utils::debug_dump_palette(bus);
        }

        if input.key_pressed(KeyCode::KeyO) {
            core::debug_utils::debug_dump_oam(bus);
        }

        if input.key_pressed(KeyCode::KeyX) {
            app_state.pause = !app_state.pause;
        }

        if input.key_pressed(KeyCode::KeyQ) {
            core::debug_utils::debug_dump_state(bus, cpu);
        }

        if input.key_pressed(KeyCode::KeyT) {
            let log_point = app_state.log_point;
            println!("LOG_POINT {}", log_point);
            info!("LOG_POINT {}", log_point);

            app_state.log_point += 1;

            cpu.enable_tracing(true);
            bus.enable_tracing(true);
        }

        if input.key_pressed(KeyCode::KeyS) {
            app_state
                .pattern_window
                .show(bus.mapper_ref(), event_loop, 4);
        }
    }
}

fn create_renderer(window: Arc<Window>) -> Result<PixelsRenderer, String> {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);
    let mut pixels =
        Pixels::new(INNER_W, INNER_H, surface_texture).map_err(|e| format!("{}", e))?;
    pixels.set_scaling_mode(ScalingMode::Fill);

    Ok(PixelsRenderer::new(
        pixels,
        INNER_W as usize,
        INNER_H as usize,
    ))
}

fn build_keymap_c1() -> HashMap<KeyCode, core::NesKey> {
    use core::NesKey;

    let mut key_map = HashMap::new();
    key_map.insert(KeyCode::KeyZ, NesKey::A);
    key_map.insert(KeyCode::KeyX, NesKey::B);
    key_map.insert(KeyCode::Space, NesKey::Select);
    key_map.insert(KeyCode::Enter, NesKey::Start);
    key_map.insert(KeyCode::ArrowUp, NesKey::Up);
    key_map.insert(KeyCode::ArrowDown, NesKey::Down);
    key_map.insert(KeyCode::ArrowLeft, NesKey::Left);
    key_map.insert(KeyCode::ArrowRight, NesKey::Right);
    key_map
}

fn build_keymap_c2() -> KeyMap {
    use core::NesKey;

    let mut key_map = HashMap::new();
    key_map.insert(KeyCode::KeyC, NesKey::A);
    key_map.insert(KeyCode::KeyV, NesKey::B);
    key_map.insert(KeyCode::KeyB, NesKey::Select);
    key_map.insert(KeyCode::KeyN, NesKey::Start);
    key_map.insert(KeyCode::KeyW, NesKey::Up);
    key_map.insert(KeyCode::KeyS, NesKey::Down);
    key_map.insert(KeyCode::KeyA, NesKey::Left);
    key_map.insert(KeyCode::KeyD, NesKey::Right);
    key_map
}

fn map_inputs(
    input: &WinitInputHelper,
    ctrl: &mut core::NesController,
    key_map: &HashMap<KeyCode, core::NesKey>,
) {
    for k in key_map.keys() {
        if input.key_pressed(*k) {
            ctrl.pressed(*key_map.get(k).unwrap());
        }
        if input.key_released(*k) {
            ctrl.released(*key_map.get(k).unwrap());
        }
    }
}
