use clap::Parser;

pub struct Context {
    pub rom_name: String,
}

impl Context {
    pub fn from_args(matches: RustinesArgs) -> Context {
        Context {
            rom_name: matches.file_path,
        }
    }
}

#[derive(Parser, Debug)]
#[clap(
    author="Davide C. <davide.canton5@gmail.com>", 
    version="1.0", 
    about="NES emulator written in Rust", 
    long_about = None
)]
pub struct RustinesArgs {
    #[clap(help = "Sets the input rom file to use")]
    file_path: String,
}
