use clap::Parser;

#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum TraceLevel {
    #[value(name = "1")]
    TraceCpu = 1,
    #[value(name = "2")]
    TraceCpuBus = 2,
}

impl TraceLevel {
    pub fn trace_cpu(&self) -> bool {
        true
    }

    pub fn trace_bus(&self) -> bool {
        use TraceLevel::*;
        matches!(self, TraceCpuBus)
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
        help = "Trace level (1=CPU, 2=CPU+BUS)"
    )]
    pub trace_level: Option<TraceLevel>,
    #[clap(short = 'b', long = "trace_boot", help = "Trace boot")]
    pub trace_boot: bool,
}

#[cfg(test)]
mod tests {
    use crate::args::TraceLevel;

    use super::RustinesArgs;
    use clap::Parser;

    #[test]
    fn defaults() {
        let args = RustinesArgs::try_parse_from(["rustines", "game.nes"]).unwrap();
        assert_eq!(args.log_file, None);
        assert!(!args.trace_boot);
        assert!(args.trace_level.is_none());
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

    #[test]
    fn trace_options() {
        let args = RustinesArgs::try_parse_from(["rustines", "-t", "2", "-b", "game.nes"]).unwrap();
        assert!(args.trace_boot);
        assert_eq!(args.trace_level, Some(TraceLevel::TraceCpuBus));

        let args = RustinesArgs::try_parse_from(["rustines", "-t", "1", "game.nes"]).unwrap();
        assert_eq!(args.trace_level, Some(TraceLevel::TraceCpu));
    }

    #[test]
    fn trace_invalid() {
        let args = RustinesArgs::try_parse_from(["rustines", "-t", "3", "game.nes"]);
        assert!(args.is_err());
    }
}
