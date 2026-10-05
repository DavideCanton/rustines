use std::{collections::HashSet, str::FromStr};

use clap::Parser;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TraceTargets {
    Cpu,
    Bus,
    Ppu,
}

impl FromStr for TraceTargets {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TraceTargets::*;
        match &*s.to_lowercase() {
            "c" => Ok(Cpu),
            "b" => Ok(Bus),
            "p" => Ok(Ppu),
            _ => Err(format!("Invalid value {}", s)),
        }
    }
}

fn parse_trace_targets(value: &str) -> Result<HashSet<TraceTargets>, String> {
    let mut targets = HashSet::new();
    if value.is_empty() {
        return Ok(targets);
    }
    for target in value.split(',') {
        targets.insert(target.trim().parse()?);
    }
    Ok(targets)
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
        help = "Trace level c=CPU, b=BUS, p=PPU",
        value_parser = parse_trace_targets,
        default_value = ""
    )]
    pub trace_level: HashSet<TraceTargets>,
    #[clap(short = 'b', long = "trace_boot", help = "Trace boot")]
    pub trace_boot: bool,
    #[clap(short = 'z', long = "zapper", help = "Zapper in port 2")]
    pub zapper: bool,
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::args::TraceTargets;

    use super::RustinesArgs;
    use clap::Parser;

    #[test]
    fn defaults() {
        let args = RustinesArgs::try_parse_from(["rustines", "game.nes"]).unwrap();
        assert_eq!(args.log_file, None);
        assert!(!args.trace_boot);
        assert!(!args.zapper);
        assert!(args.trace_level.is_empty());
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
        let args =
            RustinesArgs::try_parse_from(["rustines", "-t", "c, b", "-b", "game.nes"]).unwrap();
        assert!(args.trace_boot);
        assert_eq!(
            args.trace_level,
            HashSet::from_iter([TraceTargets::Bus, TraceTargets::Cpu])
        );

        let args = RustinesArgs::try_parse_from(["rustines", "-t", "c,b,p", "game.nes"]).unwrap();
        assert_eq!(
            args.trace_level,
            HashSet::from_iter([TraceTargets::Bus, TraceTargets::Cpu, TraceTargets::Ppu])
        );
        assert_eq!(
            RustinesArgs::try_parse_from(["rustines", "-t", "c,c", "game.nes"])
                .unwrap()
                .trace_level,
            HashSet::from_iter([TraceTargets::Cpu])
        );
    }

    #[test]
    fn trace_invalid() {
        use clap::error::ErrorKind;

        for value in ["3", "x", "c,x"] {
            let error =
                RustinesArgs::try_parse_from(["rustines", "-t", value, "game.nes"]).unwrap_err();
            assert_eq!(error.kind(), ErrorKind::ValueValidation);
        }
    }
}
