#![cfg(target_arch = "wasm32")]

mod platform;
mod renderer;

use log::warn;
use rustines_core as core;

use rustines_gui::{App, CustomEvents};
use std::{
    panic::{PanicHookInfo, set_hook},
    path::Path,
};
use web_sys::{Event, HtmlElement, VisibilityState, js_sys::Function};

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

    console_error_panic_hook::hook(info);
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

    let (nes_rom, mapper) = loader
        .load_rom_struct(Some(file_path), buf)
        .map_err(|err| JsValue::from_str(&err.to_string()))?;

    let event_loop = EventLoop::with_user_event()
        .build()
        .map_err(|err| JsValue::from_str(&err.to_string()))?;

    let proxy = event_loop.create_proxy();

    let app = App::new(
        name.to_string(),
        nes_rom,
        mapper,
        false,
        zapper,
        WIDTH,
        platform::WasmPlatform,
        false,
        proxy.clone(),
    );

    event_loop.spawn_app(app);

    let closure = move |e: JsValue| {
        let e: Event = e.into();
        e.prevent_default();

        let doc = web_sys::window().unwrap().document().unwrap();
        match doc.visibility_state() {
            VisibilityState::Hidden => {
                let _ = proxy.send_event(CustomEvents::Pause);
            }
            VisibilityState::Visible => {
                if let Some(canvas) = doc.get_element_by_id("game_canvas") {
                    let _ = canvas.dyn_into::<HtmlElement>().unwrap().focus();
                }
                let _ = proxy.send_event(CustomEvents::Resume);
            }
            _ => {}
        }
        false
    };
    let func: ScopedClosure<'static, dyn FnMut(JsValue) -> bool> = ScopedClosure::own(closure);

    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .add_event_listener_with_callback("visibilitychange", &Function::from_closure(func))
        .unwrap();

    Ok(())
}
