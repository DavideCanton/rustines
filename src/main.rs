#[macro_use]
extern crate log;
extern crate env_logger;

mod arch;
mod utils;
mod loaders;
// use arch::memory::Memory;
// use arch::cpu::CPU;

use log::{LogLevelFilter, SetLoggerError};
use env_logger::LogBuilder;
use std::fs::File;
use std::path::PathBuf;
use loaders::loaders_factory::LoadersFactory;


fn init_logger() -> Result<(), SetLoggerError> {
    let mut builder = LogBuilder::new();
    builder.filter(None, LogLevelFilter::Info);
    builder.init()
}

fn load_rom(buf: &[u8]) -> &[u8] {
    let header = &buf[0..16];
    let prg_size = header[4] as usize;
    let prg_bank_size = 1 << 14; // 16384
    let has_trainer = (buf[6] & (1 << 2)) != 0;

    info!("Rom has trainer? {}", has_trainer);

    let prg_start = 16 + (if has_trainer { 512 } else { 0 }) as usize;
    let prg_end = prg_start + (prg_size * prg_bank_size) as usize;

    info!("PRG ROM from {:#X} to {:#X}", prg_start, prg_end);

    let rom: &[u8] = &buf[prg_start..prg_end];

    rom
}

pub fn main() {
    if let Err(_) = init_logger() {
        eprintln!("Failed to initialize logger.");
    }

    let file_path = PathBuf::from("C:\\Users\\Davide\\Downloads\\nes\\NES Test (USA).zip"); // example path
    let ext = file_path.extension().unwrap().to_str().unwrap();

    let mut file = File::open(&file_path).expect("Failed to open file");

    let loader = LoadersFactory::decode(ext);

    info!("Selected loader {}", loader.name());

    let buf = loader.load_rom(&mut file).expect("Failed to load ROM");
    let rom = load_rom(&buf);

    info!("ROM size: {}", rom.len());
}
