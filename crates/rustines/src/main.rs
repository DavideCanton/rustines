mod context;
mod pattern_window;
mod renderer;
mod utils;

use crate::{
    context::RustinesArgs,
    pattern_window::PatternTableWindow,
    renderer::PixelsRenderer,
    utils::{init_logger, read_file},
};
use clap::Parser;
use log::info;
use pixels::{Pixels, SurfaceTexture};
use rustines_core as core;
use rustines_gui_utils::{FpsCounter, FpsLimiter};
use std::{collections::HashMap, path, sync::Arc};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    keyboard::KeyCode,
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

const INNER_W: u32 = 256;
const INNER_H: u32 = 240;

type KeyMap = HashMap<KeyCode, rustines_core::NesKey>;

struct AppState {
    bus: core::Bus,
    cpu: core::Cpu,

    limiter: FpsLimiter,
    counter: FpsCounter,
    logpoint: u32,

    key_map1: KeyMap,
    key_map2: KeyMap,

    pattern_window: PatternTableWindow,
    main_window: Arc<Window>,

    pause: bool,
}

pub fn main() {
    let args = RustinesArgs::parse();

    let _logger_handle = init_logger(args.log_file, args.trace_level);

    let file_path = path::PathBuf::from(&args.file_path);

    info!("Using input file: {}", args.file_path);

    let rom = read_file(&file_path).unwrap();

    let event_loop = EventLoop::new().unwrap();

    let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
    let main_window = Arc::new(
        WindowBuilder::new()
            .with_title("Rustines")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap(),
    );

    let renderer = create_renderer(Arc::clone(&main_window)).unwrap();

    let mut app_state = init_app_state(rom, main_window, renderer, args.trace_boot);

    let mut input = WinitInputHelper::new();

    let _ = event_loop.run(|event, elwt| {
        // update logic
        if input.update(&event) {
            update_logic(&input, elwt, &mut app_state);
        }

        // dispatch events to windows
        match event {
            Event::WindowEvent { window_id, event } if window_id == app_state.main_window.id() => {
                main_window_event(&mut app_state, event)
            }
            Event::WindowEvent { window_id, event }
                if app_state.pattern_window.is_event(window_id) =>
            {
                pattern_window_event(&mut app_state.pattern_window, event)
            }

            _ => {}
        }
    });
}

fn init_app_state(
    rom: rustines_core::NesRom,
    main_window: Arc<Window>,
    renderer: PixelsRenderer,
    trace_boot: bool,
) -> AppState {
    let ppu = core::Ppu::new(Box::new(renderer));
    let apu = core::Apu::default();

    let mut bus = core::Bus::new(rom.mapper, ppu, apu);
    let mut cpu = core::Cpu::new();

    if trace_boot {
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
        logpoint: 1,
        pattern_window: PatternTableWindow::new(),
        main_window,
        pause: false,
    }
}

fn update_logic(
    input: &WinitInputHelper,
    elwt: &EventLoopWindowTarget<()>,
    app_state: &mut AppState,
) {
    // Close events
    if input.key_pressed(KeyCode::Escape) || input.close_requested() {
        // this closes everything, don't really care
        elwt.exit();
    } else {
        map_debug_keys(input, app_state, elwt);

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
}

fn pattern_window_event(pattern_window: &mut PatternTableWindow, event: WindowEvent) {
    match event {
        WindowEvent::RedrawRequested => {
            pattern_window.draw();
        }
        WindowEvent::Resized(size) => {
            pattern_window.resize(size);
        }
        _ => {}
    }
}

fn main_window_event(app_state: &mut AppState, event: WindowEvent) {
    if event == WindowEvent::RedrawRequested {
        // Draw the current frame
        app_state.bus.ppu_mut().renderer().draw();

        if let Some(fps) = app_state.counter.drawn() {
            app_state
                .main_window
                .set_title(&format!("Rustines | FPS: {:.1}", fps));
        }
    }
}

fn map_debug_keys(
    input: &WinitInputHelper,
    app_state: &mut AppState,
    elwt: &EventLoopWindowTarget<()>,
) {
    let debug_keys_state = debug_keys(input);

    let bus = &mut app_state.bus;
    let cpu = &mut app_state.cpu;

    if debug_keys_state.dump_nametables {
        core::debug_utils::debug_dump_nametable(bus);
    }

    if debug_keys_state.dump_palette {
        core::debug_utils::debug_dump_palette(bus);
    }

    if debug_keys_state.dump_oam {
        core::debug_utils::debug_dump_oam(bus);
    }

    if debug_keys_state.toggle_pause {
        app_state.pause = !app_state.pause;
    }

    if debug_keys_state.dump_state {
        core::debug_utils::debug_dump_state(bus, cpu);
    }

    if debug_keys_state.logpoint {
        let logpoint = &mut app_state.logpoint;
        println!("LOGPOINT {}", logpoint);
        info!("LOGPOINT {}", logpoint);
        *logpoint += 1;
        cpu.enable_tracing(true);
        bus.enable_tracing(true);
    }

    if debug_keys_state.show_pattern_window && !app_state.pattern_window.is_displayed() {
        app_state.pattern_window.show(bus.mapper_ref(), elwt, 4);
    }
}

fn create_renderer(window: Arc<Window>) -> Result<PixelsRenderer, String> {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);
    let pixels = Pixels::new(INNER_W, INNER_H, surface_texture).map_err(|e| format!("{}", e))?;

    Ok(PixelsRenderer::new(
        pixels,
        INNER_W as usize,
        INNER_H as usize,
    ))
}

#[derive(Default)]
struct DebugKeyResult {
    dump_nametables: bool,
    dump_palette: bool,
    dump_oam: bool,
    dump_state: bool,
    logpoint: bool,
    show_pattern_window: bool,
    toggle_pause: bool,
}

fn debug_keys(input: &WinitInputHelper) -> DebugKeyResult {
    let mut r = DebugKeyResult::default();

    if input.key_pressed(KeyCode::KeyD) && input.held_shift() {
        r.dump_nametables = true;
    }

    if input.key_pressed(KeyCode::KeyP) && input.held_shift() {
        r.dump_palette = true;
    }

    if input.key_pressed(KeyCode::KeyX) && input.held_shift() {
        r.toggle_pause = true;
    }

    if input.key_pressed(KeyCode::KeyO) && input.held_shift() {
        r.dump_oam = true;
    }

    if input.key_pressed(KeyCode::KeyT) && input.held_shift() {
        r.logpoint = true;
    }

    if input.key_pressed(KeyCode::KeyS) && input.held_shift() {
        r.show_pattern_window = true;
    }

    if input.key_pressed(KeyCode::KeyQ) && input.held_shift() {
        r.dump_state = true;
    }

    r
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
