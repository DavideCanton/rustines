mod context;
mod renderer;

use clap::Parser;
// use
use env_logger::Builder;
use log::{LevelFilter, info};
use pixels::{Pixels, SurfaceTexture};
use rustines_core::{
    arch::{bus::Bus, cpu::Cpu, ppu::Ppu, rom_structs},
    loaders::loaders_factory::decode_loader,
};
use std::{fs, path, sync::Arc};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::Window,
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
        event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("Rustines")
                    .with_inner_size(size)
                    .with_min_inner_size(size),
            )
            .unwrap(),
    );

    let window_size = window.inner_size();
    let surface_texture =
        SurfaceTexture::new(window_size.width, window_size.height, Arc::clone(&window));
    let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();

    let renderer = PixelsRenderer::new(pixels, WIDTH as usize, HEIGHT as usize);
    let render_box = Box::new(renderer);
    let ppu = Ppu::new(render_box);
    let mut bus = Bus::new(rom.mapper, ppu);
    let mut cpu = Cpu::new();

    let _ = event_loop.run(|event, elwt| {
        match event {
            Event::Resumed => {}
            Event::NewEvents(_) => input.step(),
            Event::AboutToWait => input.end_step(),
            Event::DeviceEvent { event, .. } => {
                input.process_device_event(&event);
            }
            Event::WindowEvent { event, .. } => {
                // Handle input events
                if input.process_window_event(&event) {
                    // Close events
                    if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                        elwt.exit();
                        return;
                    }
                }

                // Draw the current frame
                if event == WindowEvent::RedrawRequested {
                    if cpu_tick(&mut bus, &mut cpu) {
                        elwt.exit();
                        return;
                    }
                    window.request_redraw();
                }
            }
            _ => {}
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

        if bus.ppu().nmi_requested() {
            cpu.perform_nmi(bus);
        }
    }
    false
}
