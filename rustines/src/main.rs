// mod declarations
mod arch;
mod context;
mod loaders;
mod utils;

// use
use crate::{
    arch::{cpu::Cpu, instrs::instr_table, memory::Memory, rom_structs},
    context::Args,
    loaders::loaders_factory::decode_loader,
};
use clap::Parser;
use env_logger::Builder;
use log::{info, LevelFilter};
use std::{fs, path};

fn init_logger() {
    let mut builder = Builder::from_default_env();

    builder.filter(None, LevelFilter::Debug).init();
}

fn disassemble_rom(rom: rom_structs::NesRom) {
    for bank in rom.prg_rom_banks.iter() {
        let mut cnt: usize = 0;

        println!("Bank {}", bank.id);
        let data = &bank.data;

        while cnt < data.len() {
            let (string, cnt_2) = instr_table::disassemble_instr(data, cnt);
            cnt = cnt_2;
            println!("{}", string);
        }
    }
}

fn execute_rom(rom: rom_structs::NesRom) {
    let mem = Memory::new(rom);
    let mut cpu = Cpu::new(mem);
    cpu.execute_verbose();
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

fn process_file(buf: rom_structs::NesRom, context: &context::Context) -> Result<(), String> {
    use context::Commands;
    info!("ROM size: {:#x}", buf.size);

    match context.subcommand {
        Commands::Dis => {
            disassemble_rom(buf);
            Ok(())
        }
        Commands::Ex => {
            execute_rom(buf);
            Ok(())
        }
        _ => Ok(()),
    }
}

pub fn main() {
    let matches = Args::parse();
    let context = context::Context::from_args(matches);

    init_logger();

    let file_path = path::PathBuf::from(&context.rom_name);

    info!("Subcommand: {:?}", &context.subcommand);
    info!("Using input file: {}", &context.rom_name);

    let rom = read_file(&file_path).unwrap();
    process_file(rom, &context).unwrap();
}
