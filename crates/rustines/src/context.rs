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
    #[clap(
        short = 't',
        long = "trace_level",
        help = "Trace level (1=CPU, 2=CPU+BUS)",
        default_value = "0"
    )]
    pub trace_level: u8,
    #[clap(short = 'b', long = "trace_boot", help = "Trace boot")]
    pub trace_boot: bool,
}
