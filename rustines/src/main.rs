mod context;
mod renderer;

use clap::Parser;
// use
use env_logger::Builder;
use log::{LevelFilter, info};
use rustines_core::{
    arch::{bus::Bus, cpu::Cpu, ppu::Ppu, rom_structs},
    loaders::loaders_factory::decode_loader,
};
use std::{fs, path};

use crate::{context::RustinesArgs, renderer::PixelsRenderer};

fn init_logger() {
    let mut builder = Builder::from_default_env();

    builder.filter(None, LevelFilter::Debug).init();
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

pub fn main() {
    let matches = RustinesArgs::parse();
    let context = context::Context::from_args(matches);

    init_logger();

    let file_path = path::PathBuf::from(&context.rom_name);

    info!("Using input file: {}", &context.rom_name);

    let rom = read_file(&file_path).unwrap();
    let ppu = Ppu::new(Box::new(PixelsRenderer));
    let mut bus = Bus::new(rom.mapper, ppu);
    let mut cpu = Cpu::new();

    loop {
        let cycles = cpu.tick(&mut bus);

        if cycles == 0xFF {
            break;
        }

        let ppu_cycles = cycles * 3;

        for _ in 0..ppu_cycles {
            bus.ppu_tick();

            if bus.ppu().nmi_requested() {
                cpu.perform_nmi(&mut bus);
            }
        }
    }
}
