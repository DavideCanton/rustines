mod context;
mod renderer;

use crate::{context::RustinesArgs, renderer::PixelsRenderer};
use clap::Parser;
use flexi_logger::{DeferredNow, FileSpec, LogSpecBuilder, Logger, LoggerHandle, WriteMode};
use log::{LevelFilter, Record, info};
use pixels::{Pixels, ScalingMode, SurfaceTexture};
use rustines_core::{self as core, Mapper, arch::debug_utils::dump_pattern_tables};
use rustines_gui_utils::{FpsCounter, FpsLimiter};
use std::{collections::HashMap, fs, io, path, sync::Arc};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    keyboard::KeyCode,
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

#[must_use]
fn init_logger(file: Option<String>, trace: u8) -> LoggerHandle {
    let mut log_spec_builder = LogSpecBuilder::new();

    log_spec_builder
        .default(LevelFilter::Debug)
        .module("wgpu", LevelFilter::Warn)
        .module("winit", LevelFilter::Warn)
        .module("naga", LevelFilter::Warn);

    if trace > 0 {
        log_spec_builder.module("rustines_core::arch::instr_tracer", LevelFilter::Trace);
    }
    if trace > 1 {
        log_spec_builder.module("rustines_core::arch::bus", LevelFilter::Trace);
        log_spec_builder.module("rustines_core::arch::controller", LevelFilter::Trace);
    }

    let log_spec = log_spec_builder.build();

    let mut logger_builder = Logger::with(log_spec);

    if let Some(file) = file {
        logger_builder = logger_builder.log_to_file(
            FileSpec::try_from(file)
                .expect("Cannot create filespec")
                .suppress_timestamp(),
        );
    }

    logger_builder
        .write_mode(WriteMode::Async)
        .format(my_format)
        .start()
        .expect("Failed to start logger")
}

fn read_file(file_path: &path::Path) -> Result<core::NesRom, String> {
    let ext = match file_path.extension() {
        Some(ext) => ext.to_str().unwrap_or(""),
        None => "",
    };

    let mut file = fs::File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;

    let loader = core::decode_loader(ext);

    let rom = loader
        .load_rom_struct(&mut file)
        .map_err(|e| format!("Failed to load ROM: {}", e))?;

    Ok(rom)
}

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

const INNER_W: u32 = 256;
const INNER_H: u32 = 240;

pub fn main() {
    let args = RustinesArgs::parse();

    let _logger_handle = if let Some(log_file_name) = args.log_file {
        println!("Logging to file: {}", log_file_name);
        init_logger(Some(log_file_name), args.trace_level)
    } else {
        init_logger(None, args.trace_level)
    };

    let file_path = path::PathBuf::from(&args.file_path);

    info!("Using input file: {}", args.file_path);

    let rom = read_file(&file_path).unwrap();

    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();

    let mut pattern_window = None;

    let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Rustines")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap(),
    );

    let window_size = window.inner_size();
    let surface_texture =
        SurfaceTexture::new(window_size.width, window_size.height, Arc::clone(&window));
    let pixels =
        Pixels::new(INNER_W, INNER_H, surface_texture).expect("Cannot create pixels buffer");

    let renderer = PixelsRenderer::new(pixels, INNER_W as usize, INNER_H as usize);

    let ppu = core::Ppu::new(Box::new(renderer));
    let apu = core::Apu::default();
    let mut bus = core::Bus::new(rom.mapper, ppu, apu);
    let mut cpu = core::Cpu::new();
    if args.trace_boot {
        cpu.enable_tracing(true);
        bus.enable_tracing(true);
    }

    let mut limiter = FpsLimiter::new(60.0);
    let mut counter = FpsCounter::new();
    let mut logpoint = 1;

    let key_map1 = build_keymap_c1();
    let key_map2 = build_keymap_c2();

    let _ = event_loop.run(|event, elwt| {
        if input.update(&event) {
            // Close events
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                // this closes everything, don't really care
                elwt.exit();
                return;
            }

            debug_keys(&input, &mut bus, &mut cpu, &mut logpoint);
            if input.key_pressed(KeyCode::KeyS) && input.held_shift() {
                pattern_window = Some(PatternTableWindow::create(bus.mapper_ref(), elwt));
            }

            map_inputs(&input, bus.controller1_mut(), &key_map1);
            map_inputs(&input, bus.controller2_mut(), &key_map2);

            while !bus.ppu_mut().frame_ready() {
                cpu.tick(&mut bus);
            }
            bus.ppu_mut().clear_frame_ready();

            limiter.update();

            window.request_redraw();
        }

        if let Event::WindowEvent {
            window_id,
            event: WindowEvent::Resized(size),
        } = &event
            && let Some(pattern_window) = pattern_window.as_mut()
            && *window_id == pattern_window.window.id()
        {
            pattern_window.resize(*size);
        }

        // Draw the current frame
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            bus.ppu_mut().renderer().draw();
            if let Some(pattern_window) = pattern_window.as_ref() {
                pattern_window.draw();
            }

            if let Some(fps) = counter.drawn() {
                window.set_title(&format!("Rustines | FPS: {:.1}", fps));
            }
        }
    });
}

#[allow(dead_code)]
struct PatternTableWindow<'a> {
    window: Arc<Window>,
    pixels: Pixels<'a>,
}

impl<'a> PatternTableWindow<'a> {
    fn create(mapper: &dyn Mapper, target: &EventLoopWindowTarget<()>) -> Self {
        let size = LogicalSize::new(512, 256);

        let window = Arc::new(
            WindowBuilder::new()
                .with_title("Pattern tables")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .with_resizable(true)
                .build(target)
                .unwrap(),
        );

        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, Arc::clone(&window));
        let mut pixels =
            Pixels::new(512, 256, surface_texture).expect("Cannot create pixels buffer");
        pixels.set_scaling_mode(ScalingMode::Fill);

        let buf = dump_pattern_tables(mapper);

        pixels.frame_mut().copy_from_slice(&buf);

        PatternTableWindow { window, pixels }
    }

    fn draw(&self) {
        self.pixels.render().expect("Failed to draw");
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.pixels
            .resize_surface(size.width, size.height)
            .expect("Failed to resize pattern table surface");
    }
}

fn debug_keys(
    input: &WinitInputHelper,
    bus: &mut core::Bus,
    cpu: &mut core::Cpu,
    logpoint: &mut u8,
) {
    if input.key_pressed(KeyCode::KeyD) && input.held_shift() {
        core::debug_dump_nametable(bus);
    }

    if input.key_pressed(KeyCode::KeyP) && input.held_shift() {
        core::debug_dump_palette(bus);
    }

    if input.key_pressed(KeyCode::KeyO) && input.held_shift() {
        core::debug_dump_oam(bus);
    }

    if input.key_pressed(KeyCode::KeyT) && input.held_shift() {
        println!("LOGPOINT {}", logpoint);
        info!("LOGPOINT {}", logpoint);
        *logpoint += 1;
        cpu.enable_tracing(true);
        bus.enable_tracing(true);
    }
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

fn build_keymap_c2() -> HashMap<KeyCode, core::NesKey> {
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

fn my_format(
    w: &mut dyn io::Write,
    _now: &mut DeferredNow,
    record: &Record,
) -> Result<(), io::Error> {
    let first = record.level().to_string().chars().next().unwrap_or(' ');
    write!(w, "[{}]{}", first, record.args())
}
