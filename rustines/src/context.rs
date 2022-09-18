use clap::{Parser, Subcommand};

pub struct Context {
    pub subcommand: Commands,
    pub rom_name: String,
}

impl Context {
    pub fn from_args(matches: Args) -> Context {
        Context {
            subcommand: matches.subcommand,
            rom_name: matches.file_path,
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Disassemble ROM
    Dis,
    /// Execute ROM instructions
    Ex,
    /// Play ROM
    Play,
}

#[derive(Parser, Debug)]
#[clap(
    author="Davide C. <davide.canton5@gmail.com>", 
    version="1.0", 
    about="NES emulator written in Rust", 
    long_about = None
)]
pub struct Args {
    #[clap(subcommand)]
    subcommand: Commands,
    #[clap(help = "Sets the input rom file to use")]
    file_path: String,
}
