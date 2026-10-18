#![cfg(target_arch = "wasm32")]

mod platform;
mod renderer;

use log::warn;
use rustines_core as core;

use console_error_panic_hook::hook;
use rustines_gui::App;
use std::{
    panic::{PanicHookInfo, set_hook},
    path::Path,
};

use wasm_bindgen::prelude::*;
use winit::{event_loop::EventLoop, platform::web::EventLoopExtWebSys};

const WIDTH: u32 = 1024;

fn panic_hook(info: &PanicHookInfo) {
    if web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("error"))
        .and_then(|e| e.class_list().add_1("show").ok())
        .is_none()
    {
        warn!("Can't display fancy error!");
    }

    hook(info);
}

#[wasm_bindgen]
pub fn start_wasm(buf: &[u8], name: &str, zapper: bool) -> Result<(), JsValue> {
    set_hook(Box::new(panic_hook));
    console_log::init_with_level(log::Level::Debug)
        .map_err(|err| JsValue::from_str(&err.to_string()))?;

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
        zapper,
        WIDTH,
        Box::new(platform::WasmPlatform),
    );

    event_loop.spawn_app(app);
    Ok(())
}
