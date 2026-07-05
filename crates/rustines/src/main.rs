mod context;
mod renderer;

use clap::Parser;
// use
use env_logger::Builder;
use log::{LevelFilter, info};
use pixels::{Pixels, SurfaceTexture};
use rustines_core::{
    arch::{bus::Bus, cpu::Cpu, debug_utils::debug_dump_nametable, ppu::Ppu, rom_structs},
    loaders::loaders_factory::decode_loader,
};
use rustines_gui_utils::{FpsCounter, FpsLimiter};
use std::{fs, path, sync::Arc};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::{context::RustinesArgs, renderer::PixelsRenderer};

fn init_logger() {
    let mut builder = Builder::from_default_env();

    builder.filter(None, LevelFilter::Info).init();
}

fn read_file(file_path: &path::Path) -> Result<rom_structs::NesRom, String> {
    let ext = match file_path.extension() {
        Some(ext) => ext.to_str().unwrap_or(""),
        None => "",
    };

    let mut file = fs::File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;

    let loader = decode_loader(ext);

    let rom = loader
        .load_rom_struct(&mut file)
        .map_err(|e| format!("Failed to load ROM: {}", e))?;

    Ok(rom)
}

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

const INNER_W: u32 = 256;
const INNER_H: u32 = 240;

#[allow(deprecated)]
pub fn main() {
    let matches = RustinesArgs::parse();
    let context = context::Context::from_args(matches);

    init_logger();

    let file_path = path::PathBuf::from(&context.rom_name);

    info!("Using input file: {}", &context.rom_name);

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
    let render_box = Box::new(renderer);
    let ppu = Ppu::new(render_box);
    let mut bus = Bus::new(rom.mapper, ppu);
    let mut cpu = Cpu::new();

    let mut limiter = FpsLimiter::new(60.0);
    let mut counter = FpsCounter::new();

    let _ = event_loop.run(|event, elwt| {
        if input.update(&event) {
            // Close events
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }

            // uncomment to dump nametable
            if input.key_pressed(KeyCode::KeyD) {
                debug_dump_nametable(bus.ppu_mut());
            }

            // map input to nes controller registers

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

fn cpu_tick(bus: &mut Bus, cpu: &mut Cpu) -> bool {
    let cycles = cpu.tick(bus, false);
    if cycles == 0xFF {
        return true;
    }
    let ppu_cycles = cycles * 3;

    for _ in 0..ppu_cycles {
        bus.ppu_tick();

        if bus.ppu_mut().nmi_requested() {
            bus.ppu_mut().clear_nmi();
            cpu.perform_nmi(bus);
        }
    }
    false
}
