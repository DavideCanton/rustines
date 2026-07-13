mod context;
mod renderer;

use crate::{context::RustinesArgs, renderer::PixelsRenderer};
use clap::Parser;
use env_logger::{Builder, Target};
use log::{LevelFilter, info};
use pixels::{Pixels, SurfaceTexture};
use rustines_core as core;
use rustines_gui_utils::{FpsCounter, FpsLimiter};
use std::{collections::HashMap, fs, path, sync::Arc};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

fn init_logger(file: Option<fs::File>) {
    let mut builder = Builder::from_default_env();

    let mut builder = builder
        .filter(None, LevelFilter::Debug)
        .filter(Some("wgpu"), LevelFilter::Warn)
        // .filter(Some("rustines_core::arch::cpu"), LevelFilter::Trace)
        .filter(Some("naga"), LevelFilter::Warn);

    if let Some(file) = file {
        builder = builder.target(Target::Pipe(Box::new(file)));
    }

    builder.init();
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
    let matches = RustinesArgs::parse();
    let context = context::Context::from_args(matches);

    init_logger(None);

    // let log_file = fs::File::create("log.log").expect("Cannot create log file");
    // init_logger(Some(log_file));

    let file_path = path::PathBuf::from(&context.rom_name);

    info!("Using input file: {}", context.rom_name);

    let rom = read_file(&file_path).unwrap();

    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();

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
    let pixels = Pixels::new(INNER_W, INNER_H, surface_texture).unwrap();

    let renderer = PixelsRenderer::new(pixels, INNER_W as usize, INNER_H as usize);

    let ppu = core::Ppu::new(Box::new(renderer));
    let mut bus = core::Bus::new(rom.mapper, ppu);
    let mut cpu = core::Cpu::new();

    let mut limiter = FpsLimiter::new(60.0);
    let mut counter = FpsCounter::new();

    let key_map1 = build_keymap_c1();
    let key_map2 = build_keymap_c2();

    let _ = event_loop.run(|event, elwt| {
        if input.update(&event) {
            // Close events
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }

            debug_keys(&input, &mut bus);

            map_inputs(&input, bus.controller1_mut(), &key_map1);
            map_inputs(&input, bus.controller2_mut(), &key_map2);

            while !bus.ppu_mut().frame_ready() {
                if cpu_tick(&mut bus, &mut cpu) {
                    // elwt.exit();
                    // return;
                }
            }
            bus.ppu_mut().clear_frame_ready();

            limiter.update();

            window.request_redraw();
        }

        // Draw the current frame
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            bus.ppu_mut().renderer().draw();

            if let Some(fps) = counter.drawn() {
                window.set_title(&format!("Rustines | FPS: {:.1}", fps));
            }
        }
    });
}

static mut LOGPOINT: usize = 1;

fn debug_keys(input: &WinitInputHelper, bus: &mut core::Bus) {
    if input.key_pressed(KeyCode::KeyD) && input.held_shift() {
        core::debug_dump_nametable(bus);
    }

    if input.key_pressed(KeyCode::KeyP) && input.held_shift() {
        core::debug_dump_palette(bus);
    }

    if input.key_pressed(KeyCode::KeyT) && input.held_shift() {
        unsafe {
            let t = LOGPOINT;
            info!("LOGPOINT {}", t);
            LOGPOINT += 1;
        }
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

fn cpu_tick(bus: &mut core::Bus, cpu: &mut core::Cpu) -> bool {
    let cycles = cpu.tick(bus);
    if cycles == 0xFF {
        return true;
    }
    let ppu_cycles = cycles * 3;

    for _ in 0..ppu_cycles {
        bus.ppu_tick();
    }

    if bus.ppu_mut().nmi_requested() {
        bus.ppu_mut().clear_nmi();
        cpu.perform_nmi(bus);
    }

    false
}
