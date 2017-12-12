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
mod context;
// use arch::memory::Memory;
// use arch::cpu::CPU;

use log::{LogLevelFilter, SetLoggerError};
use env_logger::LogBuilder;
use std::fs::File;
use std::path::PathBuf;
use loaders::loaders_factory::LoadersFactory;
use arch::rom_structs::Header;
use clap::{App, Arg, ArgMatches};
use arch::instrs::instr_table::disassemble_instr;
use context::Context;

fn init_logger() -> Result<(), SetLoggerError> {
    let mut builder = LogBuilder::new();
    builder.filter(None, LogLevelFilter::Info);
    builder.init()
}

fn load_rom(buf: &[u8]) -> Result<(Header, &[u8]), String> {
    let header = Header::from_bytes(array_ref![buf[0..16], 0, 16]);

    if &header.header != "NES\x1A".as_bytes() {
        error!("Found unexpected {:?} header", header.header);
        return Err("Invalid header!".to_owned());
    }

    info!("Rom has trainer? {}", header.has_trainer());

    let trainer_size = if header.has_trainer() { 512 } else { 0 };
    let prg_start = 16 + trainer_size as usize;
    let prg_end = prg_start + header.prg_rom_size();

    info!("PRG ROM from {:#x} to {:#x}", prg_start, prg_end);
    info!("Number of PRG banks: {}", header.prg_rom_banks());

    info!("Mapper: {}", header.mapping_number());

    let rom: &[u8] = &buf[prg_start..prg_end];

    Ok((header, rom))
}

fn disassemble_rom(_: &Header, rom: &[u8]) {
    let mut cnt: usize = 0;
    let mut last = 0;

    while cnt < rom.len() {
        let rem = cnt >> 14;
        if cnt == 0 || rem != last {
            last = rem;
            println!("Bank {}", last + 1);
        }
        let (string, cnt_2) = disassemble_instr(&rom, cnt);
        cnt = cnt_2;
        println!("{}", string);
    }
}

fn get_args() -> ArgMatches<'static> {
    App::new("rustines")
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
        .get_matches()
}

fn read_file(file_path: PathBuf) -> Result<Vec<u8>, String> {
    let ext = match file_path.extension() {
        Some(ext) => ext.to_str().unwrap_or(""),
        None => "",
    };

    let mut file = File::open(&file_path)
        .map_err(|e| format!("Failed to open file: {}", e))?;

    let loader = LoadersFactory::decode(ext);

    let buf = loader.load_rom(&mut file)
        .map_err(|e| format!("Failed to load ROM: {}", e))?;

    Ok(buf)
}

fn process_file(buf: &[u8], context: &Context) -> Result<(), String> {
    let (header, rom) = load_rom(buf)?;

    info!("ROM size: {:#x}", rom.len());

    if context.disassemble {
        Ok(disassemble_rom(&header, &rom))
    } else {
        // TODO execute ROM
        Ok(())
    }
}

fn build_context(matches: ArgMatches) -> Context {
    Context {
        disassemble: matches.occurrences_of("disassemble") > 0,
        rom_name: matches.value_of("INPUT").unwrap().to_owned()
    }
}

pub fn main() {
    let matches = get_args();
    let context = build_context(matches);

    if init_logger().is_err() {
        eprintln!("Failed to initialize logger.");
        return;
    }

    let file_path = PathBuf::from(&context.rom_name);

    info!("Disassemble: {}", &context.disassemble);
    info!("Using input file: {}", &context.rom_name);

    let buf = read_file(file_path).unwrap();
    process_file(&buf, &context).unwrap();
}
