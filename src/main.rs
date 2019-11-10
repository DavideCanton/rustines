// mod declarations
mod arch;
mod context;
mod loaders;
mod utils;

// use
use crate::{
    arch::{instrs::instr_table, rom_structs},
    loaders::loaders_factory::decode_loader,
};
use env_logger::Builder;
use log::{info, LevelFilter};
use std::{fs, path};

fn init_logger() {
    let mut builder = Builder::from_default_env();

    builder.filter(None, LevelFilter::Info).init();
}

fn disassemble_rom(rom: &rom_structs::NesRom) {
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

fn get_args() -> clap::ArgMatches<'static> {
    clap::App::new("rustines")
        .version("1.0")
        .author("Davide C. <davide.canton5@gmail.com>")
        .about("NES emulator written in Rust")
        .arg(
            clap::Arg::with_name("disassemble")
                .short("d")
                .help("Disassemble ROM"),
        )
        .arg(
            clap::Arg::with_name("INPUT")
                .help("Sets the input rom file to use")
                .required(true)
                .index(1),
        )
        .get_matches()
}

fn read_file(file_path: &path::PathBuf) -> Result<rom_structs::NesRom, String> {
    let ext = match file_path.extension() {
        Some(ext) => ext.to_str().unwrap_or(""),
        None => "",
    };

    let mut file = fs::File::open(&file_path).map_err(|e| format!("Failed to open file: {}", e))?;

    let loader = decode_loader(ext);

    let rom = loader
        .load_rom_struct(&mut file)
        .map_err(|e| format!("Failed to load ROM: {}", e))?;

    Ok(rom)
}

fn process_file(buf: &rom_structs::NesRom, context: &context::Context) -> Result<(), String> {
    info!("ROM size: {:#x}", buf.size);

    if context.disassemble {
        disassemble_rom(buf);
        Ok(())
    } else {
        // TODO execute ROM
        Ok(())
    }
}

pub fn main() {
    let matches = get_args();
    let context = context::Context::build_context(&matches);

    init_logger();

    let file_path = path::PathBuf::from(&context.rom_name);

    info!("Disassemble: {}", &context.disassemble);
    info!("Using input file: {}", &context.rom_name);

    let rom = read_file(&file_path).unwrap();
    process_file(&rom, &context).unwrap();
}
