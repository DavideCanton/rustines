mod args;
mod platform;

use std::path::Path;

use crate::args::RustinesArgs;
use clap::Parser;
use log::info;
use rustines_gui::{App, init_logger, read_file};
use winit::event_loop::EventLoop;

const WIDTH: u32 = 1024;

pub fn main() {
    let args = RustinesArgs::parse();

    let _logger_handle = init_logger(args.log_file, args.trace_level);

    info!("Using input file: {}", args.file_path);

    let file_path = Path::new(&args.file_path);
    let (_, mapper) = read_file(file_path).unwrap();

    info!(
        "Zapper {}",
        if args.zapper { "enabled" } else { "disabled" }
    );

    let event_loop = EventLoop::new().unwrap();

    let mut app = App::new(
        file_path.file_name().unwrap().to_str().unwrap().to_string(),
        mapper,
        args.trace_boot,
        args.zapper,
        WIDTH,
        Box::new(platform::DesktopPlatform),
    );

    let _ = event_loop.run_app(&mut app);
}
