#![cfg(target_arch = "wasm32")]

use rustines_core as core;

use rustines_gui::App;

use std::sync::Arc;

use wasm_bindgen::prelude::*;

use winit::{
    event_loop::EventLoop,
    platform::web::{EventLoopExtWebSys, WindowExtWebSys},
    window::Window,
};

const INNER_W: u32 = 256;
const INNER_H: u32 = 240;
const WIDTH: u32 = 1024;
const RATIO: f64 = INNER_H as f64 / INNER_W as f64;
const HEIGHT: u32 = (WIDTH as f64 * RATIO) as u32;

fn attach_canvas(window: Arc<Window>) {
    let web_window = web_sys::window().expect("no global `window` exists");
    let document = web_window
        .document()
        .expect("should have a document on window");
    let body = document.body().expect("HTML body missing");

    let canvas = window.canvas().expect("Canvas generation failed");
    canvas.set_width(WIDTH);
    canvas.set_height(HEIGHT);
    canvas.set_id("game_canvas");
    body.append_child(&canvas).expect("Canvas append failed");
}

#[wasm_bindgen]
pub fn start_wasm(buf: &[u8], name: &str, ext: &str) -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).ok();

    let loader = core::decode_loader(ext);

    let (_, mapper) = loader
        .load_rom_struct(buf)
        .map_err(|err| JsValue::from_str(&err.to_string()))?;

    let event_loop = EventLoop::new().map_err(|err| JsValue::from_str(&err.to_string()))?;
    let app = App::new(name.to_string(), mapper, false, false, Some(attach_canvas));

    event_loop.spawn_app(app);
    Ok(())
}
