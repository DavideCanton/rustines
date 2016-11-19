#[macro_use]
extern crate log;
extern crate env_logger;

mod arch;
mod utils;
//use arch::memory::Memory;
//use arch::cpu::CPU;

use log::{LogLevelFilter, SetLoggerError};
use env_logger::LogBuilder;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

fn load_rom(path: PathBuf) -> std::io::Result<Vec<u8>> {
    let mut f : File = try!(File::open(path));
    let mut buf : Vec<u8> = vec![];

    f.read_to_end(&mut buf)?;

    let header = &buf[0..16];
    let prg_size = header[4] as usize;
    let prg_bank_size = 1 << 14; // 16384
    let has_trainer = (buf[6] & (1 << 2)) != 0;

    info!("Rom has trainer? {}", has_trainer);
    
    let prg_start = 16 + (if has_trainer { 512 } else { 0 }) as usize;
    let prg_end = prg_start + (prg_size * prg_bank_size) as usize;

    info!("PRG ROM from {:#X} to {:#X}", prg_start, prg_end);    

    let rom : &[u8] = &buf[prg_start..prg_end];

    Ok(Vec::from(rom))
}

fn init_logger() -> Result<(), SetLoggerError>{    
    let mut builder = LogBuilder::new();
    builder.filter(None, LogLevelFilter::Info);
    builder.init()
}

pub fn main() {
    let _ = init_logger();

    let file_path = "C:\\Users\\Davide\\Downloads\\Legend of Zelda, The (USA)\\Legend of Zelda, The (USA).nes"; // example path
    let mem = load_rom(PathBuf::from(file_path)).unwrap();
    //let mem = Memory::from_array(mem).expect("Invalid data!");
    

}
