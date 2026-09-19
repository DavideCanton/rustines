use flexi_logger::{DeferredNow, FileSpec, LogSpecBuilder, Logger, LoggerHandle, WriteMode};
use log::{LevelFilter, Record};
use rustines_core as core;
use std::{fs, io, path};

#[must_use]
pub fn init_logger(file: Option<String>, trace: u8) -> LoggerHandle {
    let mut log_spec_builder = LogSpecBuilder::new();

    log_spec_builder
        .default(LevelFilter::Debug)
        .module("wgpu", LevelFilter::Warn)
        .module("winit", LevelFilter::Warn)
        .module("naga", LevelFilter::Warn);

    if trace > 0 {
        log_spec_builder.module("rustines_core::arch::instr_tracer", LevelFilter::Trace);
    }
    if trace > 1 {
        log_spec_builder.module("rustines_core::arch::bus", LevelFilter::Trace);
        log_spec_builder.module("rustines_core::arch::controller", LevelFilter::Trace);
    }

    let log_spec = log_spec_builder.build();

    let mut logger_builder = Logger::with(log_spec);

    if let Some(file) = file {
        println!("Logging to file: {}", file);

        logger_builder = logger_builder.log_to_file(
            FileSpec::try_from(file)
                .expect("Cannot create filespec")
                .suppress_timestamp(),
        );
    }

    logger_builder
        .write_mode(WriteMode::Async)
        .format(my_format)
        .start()
        .expect("Failed to start logger")
}

pub fn read_file(file_path: &path::Path) -> Result<core::NesRom, String> {
    let ext = match file_path.extension() {
        Some(ext) => ext.to_str().unwrap_or(""),
        None => "",
    };

    let mut file = fs::File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;

    let loader = core::decode_loader(ext);

    let rom = loader
        .load_rom_struct(&mut file)
        .map_err(|e| format!("Failed to load ROM: {}", e))?;

    Ok(rom)
}

fn my_format(
    w: &mut dyn io::Write,
    _now: &mut DeferredNow,
    record: &Record,
) -> Result<(), io::Error> {
    let first = record.level().to_string().chars().next().unwrap_or(' ');
    write!(w, "[{}]{}", first, record.args())
}
