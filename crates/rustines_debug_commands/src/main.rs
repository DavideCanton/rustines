mod context;
mod types;

use crate::{context::Args, types::RustinesDebugError};
use clap::Parser;
use env_logger::Builder;
use log::{LevelFilter, info};
use rustines_core::{self as core, arch::instrs::instr_table::disassemble_instr};
use std::{fs, path};

fn init_logger() {
    let mut builder = Builder::from_default_env();

    builder.filter(None, LevelFilter::Debug).init();
}

fn disassemble_rom(rom: core::NesRom) {
    let data = rom.mapper.prg_rom();
    let mut cnt: usize = 0;

    while cnt < data.len() {
        let (string, cnt_2) = disassemble_instr(data, cnt);
        cnt = cnt_2;
        println!("{}", string);
    }
}

#[allow(unused)]
fn execute_rom(rom: core::NesRom, verbose: bool) {
    let ppu = core::Ppu::new(Box::new(core::NoopRenderer));
    let mem = core::Bus::new(rom.mapper, ppu);
    let mut cpu = core::Cpu::new();
    todo!()
}

fn read_file(file_path: &path::Path) -> Result<core::NesRom, RustinesDebugError> {
    let ext = match file_path.extension() {
        Some(ext) => ext.to_str().unwrap_or(""),
        None => "",
    };

    let mut file = fs::File::open(file_path).map_err(RustinesDebugError::ReadFileError)?;

    let loader = core::decode_loader(ext);

    let rom = loader
        .load_rom_struct(&mut file)
        .map_err(|e| RustinesDebugError::FileFormatError(e.to_string()))?;

    Ok(rom)
}

fn process_file(buf: core::NesRom, context: &context::Context) -> Result<(), RustinesDebugError> {
    use context::Commands;

    match &context.subcommand {
        Commands::Dis => {
            disassemble_rom(buf);
        }
        Commands::Ex(args) => {
            execute_rom(buf, args.verbose);
        }
    };
    Ok(())
}

pub fn main() -> anyhow::Result<()> {
    let matches = Args::parse();
    let context = context::Context::from_args(matches);

    init_logger();

    let file_path = path::PathBuf::from(&context.rom_name);

    info!("Subcommand: {:?}", context.subcommand);
    info!("Using input file: {}", context.rom_name);

    let rom = read_file(&file_path)?;
    process_file(rom, &context)?;
    Ok(())
}
