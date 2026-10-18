#![cfg(target_arch = "wasm32")]

mod platform;

use rustines_core as core;

use rustines_gui::App;
use std::path::Path;

use wasm_bindgen::prelude::*;
use winit::{event_loop::EventLoop, platform::web::EventLoopExtWebSys};

const WIDTH: u32 = 1024;

#[wasm_bindgen]
pub fn start_wasm(buf: &[u8], name: &str) -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).ok();

    let file_path = Path::new(name);
    let ext = match file_path.extension() {
        Some(ext) => ext.to_str().unwrap_or(""),
        None => "",
    };

    let loader = core::decode_loader(ext);

    let (_, mapper) = loader
        .load_rom_struct(buf)
        .map_err(|err| JsValue::from_str(&err.to_string()))?;

    let event_loop = EventLoop::new().map_err(|err| JsValue::from_str(&err.to_string()))?;
    let app = App::new(
        name.to_string(),
        mapper,
        false,
        false,
        WIDTH,
        Box::new(platform::WasmPlatform),
    );

    event_loop.spawn_app(app);
    Ok(())
}
