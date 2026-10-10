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
use rustines_core::{self as core, arch::bus::Controller2};
use rustines_gui_utils::FpsCounter;
use std::{
    collections::HashMap,
    path,
    sync::Arc,
    time::{Duration, Instant},
};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{ElementState, MouseButton, StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::KeyCode,
    window::{Window, WindowAttributes, WindowId},
};
use winit_input_helper::WinitInputHelper;

const INNER_W: u32 = 256;
const INNER_H: u32 = 240;
const WIDTH: u32 = 1024;
const RATIO: f64 = INNER_H as f64 / INNER_W as f64;
const HEIGHT: u32 = (WIDTH as f64 * RATIO) as u32;
const TARGET_FPS: f64 = 60.0;

type KeyMap = HashMap<KeyCode, rustines_core::NesKey>;

#[derive(Default)]
struct SpeedupState {
    active: bool,
    changed: bool,
}

impl SpeedupState {
    fn update(&mut self, active: bool) {
        self.changed = active != self.active;
        self.active = active;
    }
}

struct AppState {
    bus: core::Bus,
    cpu: core::Cpu,

    render_counter: FpsCounter,
    update_counter: FpsCounter,
    log_point: u32,

    key_map1: KeyMap,
    key_map2: KeyMap,

    pattern_window: PatternTableWindow,

    main_window: Arc<Window>,
    main_window_helper: WinitInputHelper,

    cursor_position: Option<(f64, f64)>,
    zapper_trigger_pressed: bool,
    zapper_needs_update: bool,

    pause: bool,
    next_frame_time: Instant,
    frame_duration: Duration,
    speedup: SpeedupState,
}

struct App {
    app_state: Option<AppState>,
    mapper: Option<core::MapperBox>,
    trace_boot: bool,
    zapper: bool,
}

impl App {
    fn new(mapper: core::MapperBox, trace_boot: bool, zapper: bool) -> Self {
        App {
            app_state: None,
            mapper: Some(mapper),
            trace_boot,
            zapper,
        }
    }

    fn init_app_state(&mut self, main_window: Arc<Window>, renderer: PixelsRenderer) -> AppState {
        let ppu = core::Ppu::new(Box::new(renderer));
        let apu = core::Apu::default();

        let ctrl2 = if self.zapper {
            Controller2::zapper()
        } else {
            Controller2::nes_controller()
        };

        let mut bus = core::Bus::new(self.mapper.take().unwrap(), ppu, apu, ctrl2);
        let mut cpu = core::Cpu::new();

        if self.trace_boot {
            cpu.enable_tracing(true);
            bus.enable_tracing(true);
            bus.ppu_mut().enable_tracing(true);
        }

        AppState {
            bus,
            render_counter: FpsCounter::new(),
            update_counter: FpsCounter::new(),
            cpu,
            key_map1: build_keymap_c1(),
            key_map2: build_keymap_c2(),
            log_point: 1,
            pattern_window: PatternTableWindow::new(),
            main_window,
            pause: false,
            main_window_helper: WinitInputHelper::new(),
            cursor_position: None,
            zapper_trigger_pressed: false,
            zapper_needs_update: false,
            next_frame_time: Instant::now(),
            frame_duration: Duration::from_secs_f64(1.0 / TARGET_FPS),
            speedup: SpeedupState::default(),
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
                    // Draw the current frame
                    app_state.bus.ppu_mut().renderer().draw();

                    app_state.render_counter.update();

                    let r_fps = format_fps(app_state.render_counter.current_fps(), "FPS");
                    let u_fps =
                        format_fps(app_state.update_counter.current_fps(), "Simulation FPS");

                    app_state
                        .main_window
                        .set_title(&format!("Rustines | {r_fps} | {u_fps}"));
                }
            } else if app_state.pattern_window.owns_window_event(window_id) {
                app_state.pattern_window.window_event(&event);
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(app_state) = self.app_state.as_mut() {
            handle_inputs(app_state, event_loop);

            let now = Instant::now();
            let next_frame = now >= app_state.next_frame_time;

            if app_state.speedup.active || next_frame {
                update_logic(app_state);
            }
            if app_state.speedup.active {
                event_loop.set_control_flow(ControlFlow::Poll);
            } else if next_frame {
                if app_state.speedup.changed {
                    app_state.next_frame_time = now + app_state.frame_duration;
                } else {
                    app_state.next_frame_time += app_state.frame_duration;
                }

                event_loop.set_control_flow(ControlFlow::WaitUntil(app_state.next_frame_time));
            }
        }
    }
}

fn update_logic(app_state: &mut AppState) {
    if !app_state.pause {
        let frame_cnt = if app_state.speedup.active { 4 } else { 1 };
        for _ in 0..frame_cnt {
            app_state.update_counter.update();
            while !app_state.bus.ppu().frame_ready() {
                app_state.cpu.tick(&mut app_state.bus);
            }
            app_state.bus.ppu_mut().clear_frame_ready();
        }
    }

    app_state.main_window.request_redraw();
    app_state.pattern_window.render();
}

pub fn main() {
    let args = RustinesArgs::parse();

    let _logger_handle = init_logger(args.log_file, args.trace_level);

    let file_path = path::PathBuf::from(&args.file_path);

    info!("Using input file: {}", args.file_path);

    let (_, mapper) = read_file(&file_path).unwrap();

    info!(
        "Zapper {}",
        if args.zapper { "enabled" } else { "disabled" }
    );

    let event_loop = EventLoop::new().unwrap();

    let mut app = App::new(mapper, args.trace_boot, args.zapper);

    let _ = event_loop.run_app(&mut app);
}

fn handle_inputs(app_state: &mut AppState, event_loop: &ActiveEventLoop) {
    let input = &mut app_state.main_window_helper;

    input.end_step();

    if input.key_pressed(KeyCode::Escape) || input.close_requested() {
        event_loop.exit();
    } else {
        handle_debug_keys(app_state, event_loop);

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

fn update_zapper_input(app_state: &mut AppState) {
    let size = app_state.main_window.inner_size();
    let position = app_state.cursor_position.and_then(|(x, y)| {
        (size.width > 0 && size.height > 0).then(|| {
            (
                (x * f64::from(INNER_W) / f64::from(size.width)) as usize,
                (y * f64::from(INNER_H) / f64::from(size.height)) as usize,
            )
        })
    });

    app_state
        .bus
        .set_zapper_input(position, app_state.zapper_trigger_pressed);
}

fn handle_debug_keys(app_state: &mut AppState, event_loop: &ActiveEventLoop) {
    let input = &app_state.main_window_helper;

    let bus = &mut app_state.bus;
    let cpu = &mut app_state.cpu;

    app_state.speedup.update(input.key_held(KeyCode::Backquote));

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
            bus.ppu_mut().enable_tracing(true);
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
    // TODO read from config file
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
    // TODO read from config file
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

fn handle_input_keys(
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

fn format_fps(fps: f64, msg: &str) -> String {
    format!("{}: {:.1}", msg, fps)
}
