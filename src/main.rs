#[macro_use]
extern crate log;
#[macro_use]
extern crate arrayref;
#[macro_use]
extern crate lazy_static;
extern crate env_logger;
extern crate clap;

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
use arch::rom_structs::Header;
use clap::{App, Arg};
use arch::instrs::instr_table::disassemble_instr;

fn init_logger() -> Result<(), SetLoggerError> {
    let mut builder = LogBuilder::new();
    builder.filter(None, LogLevelFilter::Info);
    builder.init()
}

fn load_rom(buf: &[u8]) -> Result<(Header, &[u8]), String> {
    let header = Header::from_bytes(array_ref![buf[0..16], 0, 16]);

    if &header.header != "NES\x1A".as_bytes() {
        return Err("Invalid header!".to_owned());
    }

    info!("Rom has trainer? {}", header.has_trainer());

    let prg_start = 16 + (if header.has_trainer() { 512 } else { 0 }) as usize;
    let prg_end = prg_start + header.prg_rom_size();

    info!("PRG ROM from {:#X} to {:#X}", prg_start, prg_end);
    info!("Number of PRG banks: {}", header.prg_rom_banks());

    info!("Mapper: {}", header.mapping_number());

    let rom: &[u8] = &buf[prg_start..prg_end];

    Ok((header, rom))
}

fn disassemble_rom(_: &Header, rom: &[u8]) {
    let mut cnt : usize = 0;
    let mut last = 0;

    while cnt < rom.len() {
        let rem = cnt >> 14;
        if cnt == 0 || rem != last {
            last = rem;
            println!("Bank {}", last + 1);
        }
        let (_, cnt_2) = disassemble_instr(&rom, cnt);
        cnt = cnt_2;
        //println!("{}", string);
    }
}

pub fn main() {
    let matches = App::new("rustines")
        .version("1.0")
        .author("Davide C. <davide.canton5@gmail.com>")
        .about("NES emulator written in Rust")
        .arg(
            Arg::with_name("disassemble")
                .short("d")
                .help("Disassemble ROM"),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input rom file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    if init_logger().is_err() {
        eprintln!("Failed to initialize logger.");
        return;
    }

    let disassemble = matches.occurrences_of("disassemble") > 0;
    let rom_name = matches.value_of("INPUT").unwrap();

    let file_path = PathBuf::from(rom_name);

    info!("Disassemble: {}", disassemble);
    info!("Using input file: {}", rom_name);    

    let ext = match file_path.extension() {
        Some(ext) => ext.to_str().unwrap_or(""),
        None => "",
    };

    let mut file = File::open(&file_path).expect("Failed to open file");

    let loader = LoadersFactory::decode(ext);

    info!("Selected loader {}", loader.name());

    let buf = loader.load_rom(&mut file).expect("Failed to load ROM");
    match load_rom(&buf) {
        Ok((header, rom)) => {
            info!("ROM size: {:#X}", rom.len());

            if disassemble {
                disassemble_rom(&header, &rom);
            } else {
                // TODO execute ROM
            }
        }
        Err(e) => {
            eprintln!("Errore {}", e);
        }
    }
}
