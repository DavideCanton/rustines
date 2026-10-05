mod context;
mod types;

use crate::{context::Args, types::RustinesDebugError};
use clap::Parser;
use flexi_logger::{LogSpecBuilder, Logger, LoggerHandle};
use log::{LevelFilter, info};
use rustines_core::{
    self as core,
    arch::{bus::Controller2, instrs::instr_table::disassemble_instr},
};
use std::{fs, path};

#[must_use]
fn init_logger() -> LoggerHandle {
    let builder = Logger::with(LogSpecBuilder::new().default(LevelFilter::Debug).build());

    builder.start().expect("Failed to start logger")
}

fn disassemble_rom(mapper: core::MapperBox) {
    let data = mapper.prg_rom();
    let mut cnt: usize = 0;

    while cnt < data.len() {
        let (string, cnt_2) = disassemble_instr(data, cnt);
        cnt = cnt_2;
        println!("{}", string);
    }
}

#[allow(unused)]
fn execute_rom(mapper: core::MapperBox, verbose: bool) {
    let ppu = core::Ppu::new(Box::new(core::NoopRenderer));
    let apu = core::Apu::default();
    let mem = core::Bus::new(mapper, ppu, apu, Controller2::nes_controller());
    let mut cpu = core::Cpu::new();
    todo!()
}

fn read_file(
    file_path: &path::Path,
) -> Result<(core::NesRom, core::MapperBox), RustinesDebugError> {
    let ext = match file_path.extension() {
        Some(ext) => ext.to_str().unwrap_or(""),
        None => "",
    };

    let mut file = fs::File::open(file_path).map_err(RustinesDebugError::ReadFileError)?;

    let loader = core::decode_loader(ext);

    let (rom, mapper) = loader
        .load_rom_struct(&mut file)
        .map_err(|e| RustinesDebugError::FileFormatError(e.to_string()))?;

    Ok((rom, mapper))
}

fn process_file(
    mapper: core::MapperBox,
    context: &context::Context,
) -> Result<(), RustinesDebugError> {
    use context::Commands;

    match &context.subcommand {
        Commands::Dis => {
            disassemble_rom(mapper);
        }
        Commands::Ex(args) => {
            execute_rom(mapper, args.verbose);
        }
    };
    Ok(())
}

pub fn main() -> anyhow::Result<()> {
    let matches = Args::parse();
    let context = context::Context::from_args(matches);

    let _logger = init_logger();

    let file_path = path::PathBuf::from(&context.rom_name);

    info!("Subcommand: {:?}", context.subcommand);
    info!("Using input file: {}", context.rom_name);

    let (_, mapper) = read_file(&file_path)?;
    process_file(mapper, &context)?;
    Ok(())
}
