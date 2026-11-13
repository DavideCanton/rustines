use flexi_logger::{DeferredNow, FileSpec, LogSpecBuilder, Logger, LoggerHandle, WriteMode};
use log::{LevelFilter, Record, debug};
use pixels::{Pixels, PixelsBuilder, ScalingMode, SurfaceTexture, wgpu::TextureFormat};
use rustines_core::{self as core, arch::mappers::mapper::MapperBox};
use std::{
    collections::HashSet,
    fs,
    io::{self, Read},
    path,
    str::FromStr,
    sync::Arc,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::INNER_SIZE;

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

#[must_use]
pub fn init_logger(file: Option<String>, trace: HashSet<TraceTargets>) -> LoggerHandle {
    let mut log_spec_builder = LogSpecBuilder::new();

    log_spec_builder
        .default(LevelFilter::Debug)
        .module("wgpu", LevelFilter::Warn)
        .module("winit", LevelFilter::Warn)
        .module("naga", LevelFilter::Warn)
        .module("gilrs", LevelFilter::Warn);

    if trace.contains(&TraceTargets::Cpu) {
        log_spec_builder.module("rustines_core::arch::instr_tracer", LevelFilter::Trace);
    }
    if trace.contains(&TraceTargets::Bus) {
        log_spec_builder.module("rustines_core::arch::bus", LevelFilter::Trace);
    }
    if trace.contains(&TraceTargets::Ppu) {
        log_spec_builder.module("rustines_core::arch::ppu", LevelFilter::Trace);
    }
    if !trace.is_empty() {
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
        .format(log_format)
        .start()
        .expect("Failed to start logger")
}

pub fn read_file(file_path: &path::Path) -> Result<(core::NesRom, MapperBox), String> {
    let ext = match file_path.extension() {
        Some(ext) => ext.to_str().unwrap_or(""),
        None => "",
    };

    let mut file = fs::File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;

    let loader = core::decode_loader(ext);

    let mut buf: Vec<u8> = vec![];
    let read = file.read_to_end(&mut buf).map_err(|e| format!("{}", e))?;

    debug!("Read {} bytes", read);

    let loaded = loader
        .load_rom_struct(Some(file_path), &buf)
        .map_err(|e| format!("Failed to load ROM: {}", e))?;

    Ok(loaded)
}

pub async fn create_pixels(
    window: Arc<Window>,
    size: PhysicalSize<u32>,
    texture_format: Option<TextureFormat>,
) -> Result<Pixels<'static>, String> {
    if size.width == 0 || size.height == 0 {
        return Err(format!(
            "Window surface has invalid size: {}x{}",
            size.width, size.height
        ));
    }

    let surface_texture = SurfaceTexture::new(size.width, size.height, window);

    let mut builder = PixelsBuilder::new(INNER_SIZE.width, INNER_SIZE.height, surface_texture);
    if let Some(texture_format) = texture_format {
        builder = builder.surface_texture_format(texture_format);
    }

    let mut pixels = builder.build_async().await.map_err(|e| format!("{}", e))?;
    pixels.set_scaling_mode(ScalingMode::Fill);

    Ok(pixels)
}

fn log_format(
    w: &mut dyn io::Write,
    _now: &mut DeferredNow,
    record: &Record,
) -> Result<(), io::Error> {
    let first = record.level().to_string().chars().next().unwrap_or(' ');
    write!(w, "[{}]{}", first, record.args())
}
