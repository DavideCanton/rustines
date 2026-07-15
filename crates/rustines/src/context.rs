use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    author="Davide C. <davide.canton5@gmail.com>", 
    version="1.0", 
    about="NES emulator written in Rust", 
    long_about = None
)]
pub struct RustinesArgs {
    #[clap(help = "Sets the input rom file to use")]
    pub file_path: String,
    #[clap(short = 'f', long = "log_file", help = "Log to file")]
    pub log_file: bool,
    #[clap(short = 't', long = "trace_cpu", help = "Trace cpu")]
    pub trace_cpu: bool,
}
