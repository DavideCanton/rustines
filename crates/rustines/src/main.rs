mod args;
mod pattern_window;
mod renderer;
mod utils;

use crate::{
    args::RustinesArgs,
    utils::{init_logger, read_file},
};
use clap::Parser;
use log::info;
use rustines_lib::App;
use std::path;
use winit::event_loop::EventLoop;

pub fn main() {
    let args = RustinesArgs::parse();

    let _logger_handle = init_logger(args.log_file, args.trace_level);

    let file_path = path::PathBuf::from(&args.file_path);

    info!("Using input file: {}", args.file_path);

    let (_, mapper) = read_file(&file_path).unwrap();

    let event_loop = EventLoop::new().unwrap();

    let mut app = App::new(mapper, args.trace_boot, args.zapper, None);

    let _ = event_loop.run_app(&mut app);
}
