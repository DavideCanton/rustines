use log::info;
use rexie::{ObjectStore, Rexie, TransactionMode};
use rustines_core::Renderer;
use rustines_gui::{
    AppState, CustomEvents, INNER_SIZE, Platform, ReadEventConstructor, WriteEventConstructor,
    create_pixels,
};
use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};
use wasm_bindgen::JsValue;
use web_sys::js_sys::Uint8Array;
use winit::event_loop::EventLoopProxy;

use crate::renderer::WasmRenderer;
use pixels::wgpu::TextureFormat;
use winit::{dpi::PhysicalSize, platform::web::WindowExtWebSys, window::Window};

pub struct WasmPlatform;

const DB_NAME: &str = "RustinesDB";
const STORE_NAME: &str = "files";

fn format_fps(fps: f64) -> String {
    format!("{:.1}", fps)
}

impl Platform for WasmPlatform {
    fn set_title(&self, app_state: &AppState) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        if let Some(e) = document.get_element_by_id("filename") {
            e.set_text_content(Some(app_state.rom_name()))
        }

        if let Some(e) = document.get_element_by_id("r_fps") {
            e.set_text_content(Some(&format_fps(app_state.render_fps())))
        }

        if let Some(e) = document.get_element_by_id("u_fps") {
            e.set_text_content(Some(&format_fps(app_state.update_fps())))
        }
    }

    fn init_window_hook(&self, window: Arc<Window>, size: PhysicalSize<u32>) {
        let web_window = web_sys::window().expect("no global `window` exists");
        let document = web_window
            .document()
            .expect("should have a document on window");

        let canvas = window.canvas().expect("Canvas generation failed");
        canvas.set_width(size.width);
        canvas.set_height(size.height);
        canvas.set_id("game_canvas");
        document
            .get_element_by_id("main")
            .unwrap()
            .append_child(&canvas)
            .expect("Canvas append failed");
    }

    fn init_pending_renderer(
        &self,
        main_window: Arc<Window>,
        pending_renderer: Rc<RefCell<Option<Box<dyn Renderer>>>>,
        size: PhysicalSize<u32>,
    ) {
        wasm_bindgen_futures::spawn_local(async move {
            let pixels = create_pixels(
                Arc::clone(&main_window),
                size,
                Some(TextureFormat::Bgra8Unorm),
            )
            .await
            .unwrap();
            let renderer = WasmRenderer::new(
                pixels,
                INNER_SIZE.width as usize,
                INNER_SIZE.height as usize,
            );
            *pending_renderer.borrow_mut() = Some(Box::new(renderer));
            main_window.request_redraw();
        });
    }

    fn esc_pressed_enabled(&self) -> bool {
        false
    }

    fn start_read_file(
        &self,
        path: PathBuf,
        proxy: &EventLoopProxy<CustomEvents>,
        evt_ctor: ReadEventConstructor,
    ) {
        if let Ok(name) = resolve_filename(&path) {
            let proxy = proxy.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let res = read_file(name)
                    .await
                    .inspect_err(|e| {
                        info!("Failed to read saved data: {e:?}");
                    })
                    .ok()
                    .flatten();
                let _ = proxy.send_event(evt_ctor(res));
            })
        }
    }

    fn start_write_file(
        &self,
        path: PathBuf,
        proxy: &EventLoopProxy<CustomEvents>,
        content: &[u8],
        evt_ctor: WriteEventConstructor,
    ) {
        if let Ok(name) = resolve_filename(&path) {
            let content = content.to_vec();
            let proxy = proxy.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Err(e) = write_file(&name, &content).await {
                    info!("Failed to write save data: {e:?}");
                }
                let _ = proxy.send_event(evt_ctor(path));
            });
        }
    }

    fn should_exit(&self) -> bool {
        false
    }
}

fn resolve_filename(path: &Path) -> Result<String, String> {
    let name = path.file_name().ok_or("No file name")?;
    let name = name.to_str().ok_or("Cannot convert path")?;
    Ok(name.to_owned())
}

async fn read_file(name: String) -> rexie::Result<Option<Vec<u8>>> {
    let rexie = Rexie::builder(DB_NAME)
        .add_object_store(ObjectStore::new(STORE_NAME))
        .build()
        .await?;

    let tx = rexie.transaction(&[STORE_NAME], TransactionMode::ReadOnly)?;
    let store = tx.store(STORE_NAME)?;

    let res = store.get(JsValue::from_str(&name)).await?.and_then(|val| {
        if val.is_null() || val.is_undefined() {
            None
        } else {
            let uint8_array = Uint8Array::new(&val);
            Some(uint8_array.to_vec())
        }
    });

    Ok(res)
}

async fn write_file(name: &str, content: &[u8]) -> rexie::Result<()> {
    let rexie = Rexie::builder(DB_NAME)
        .add_object_store(ObjectStore::new(STORE_NAME))
        .build()
        .await?;

    let tx = rexie.transaction(&[STORE_NAME], TransactionMode::ReadWrite)?;
    let store = tx.store(STORE_NAME)?;

    let uint8_array = Uint8Array::from(content);

    store
        .put(&uint8_array, Some(&JsValue::from_str(name)))
        .await?;

    tx.commit().await?;

    Ok(())
}
