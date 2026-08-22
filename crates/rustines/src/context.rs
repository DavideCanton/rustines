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
    #[clap(
        short = 'f',
        long = "log_file",
        help = "Log to file",
        num_args = 0..=1,
        require_equals = true,
        default_missing_value = "rustines.log"
    )]
    pub log_file: Option<String>,
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

#[cfg(test)]
mod tests {
    use super::RustinesArgs;
    use clap::Parser;

    #[test]
    fn log_file_is_none_when_omitted() {
        let args = RustinesArgs::try_parse_from(["rustines", "game.nes"]).unwrap();

        assert_eq!(args.log_file, None);
    }

    #[test]
    fn log_file_uses_default_when_value_is_omitted() {
        let args = RustinesArgs::try_parse_from(["rustines", "-f", "game.nes"]).unwrap();

        assert_eq!(args.log_file.as_deref(), Some("rustines.log"));
    }

    #[test]
    fn log_file_uses_supplied_value() {
        let args = RustinesArgs::try_parse_from(["rustines", "-f=custom.log", "game.nes"]).unwrap();

        assert_eq!(args.log_file.as_deref(), Some("custom.log"));
    }
}
