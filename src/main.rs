#[macro_use]
extern crate log;
#[macro_use]
extern crate arrayref;
extern crate env_logger;

mod arch;
mod utils;
mod loaders;
// use arch::memory::Memory;
// use arch::cpu::CPU;

use log::{LogLevelFilter, SetLoggerError};
use std::env;
use env_logger::LogBuilder;
use std::fs::File;
use std::path::PathBuf;
use loaders::loaders_factory::LoadersFactory;
use arch::rom_structs::Header;

fn init_logger() -> Result<(), SetLoggerError> {
    let mut builder = LogBuilder::new();
    builder.filter(None, LogLevelFilter::Info);
    builder.init()
}

fn load_rom(buf: &[u8]) -> Result<&[u8], String> {
    let header = Header::from_bytes(array_ref![buf[0..16], 0, 16]);

    if &header.header != "NES\x1A".as_bytes() {
        return Err("Invalid header!".to_owned());
    }

    info!("Rom has trainer? {}", header.has_trainer());
    
    let prg_start = 16 + (if header.has_trainer() { 512 } else { 0 }) as usize;
    let prg_end = prg_start + header.prg_rom_size();

    info!("PRG ROM from {:#X} to {:#X}", prg_start, prg_end);

    info!("Mapper: {}", header.mapping_number());

    let rom: &[u8] = &buf[prg_start..prg_end];

    Ok(rom)
}

pub fn main() {
    if init_logger().is_err() {
        eprintln!("Failed to initialize logger.");
        return;
    }

    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Not enough arguments!");
        return;
    }

    let file_path = PathBuf::from(&args[1]);

    let ext = match file_path.extension() {
        Some(ext) => ext.to_str().unwrap_or(""),
        None => "",
    };

    let mut file = File::open(&file_path).expect("Failed to open file");

    let loader = LoadersFactory::decode(ext);

    info!("Selected loader {}", loader.name());

    let buf = loader.load_rom(&mut file).expect("Failed to load ROM");
    match load_rom(&buf) {
        Ok(rom) => {
            info!("ROM size: {}", rom.len());
        }
        Err(e) => {
            eprintln!("Errore {}", e);
        }
    }
}
