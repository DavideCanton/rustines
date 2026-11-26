use log::info;
use rexie::{ObjectStore, Rexie, TransactionMode};
use rustines_core::Renderer;
use rustines_gui::{
    AppEventLoopProxy, INNER_SIZE, Platform, ReadEventConstructor, WriteEventConstructor,
    create_pixels,
};
use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};
use wasm_bindgen::JsValue;
use web_sys::{Element, js_sys::Uint8Array};

use crate::constants::{
    CANVAS_ID, CANVAS_PARENT_ID, DB_NAME, RENDER_FPS_ID, STORE_NAME, UPDATE_FPS_ID,
};
use crate::renderer::WasmRenderer;
use pixels::wgpu::TextureFormat;
use winit::{dpi::PhysicalSize, platform::web::WindowExtWebSys, window::Window};

pub struct WasmPlatform {
    render_fps_element: RefCell<Option<Element>>,
    update_fps_element: RefCell<Option<Element>>,
}

impl WasmPlatform {
    pub fn new() -> Self {
        Self {
            render_fps_element: RefCell::new(None),
            update_fps_element: RefCell::new(None),
        }
    }

    fn cache_elements(&self) {
        let web_window = web_sys::window().expect("no global window exists");
        let document = web_window
            .document()
            .expect("should have a document on window");

        let render_fps_element = document
            .get_element_by_id(RENDER_FPS_ID)
            .expect("Cannot find render fps element");

        let update_fps_element = document
            .get_element_by_id(UPDATE_FPS_ID)
            .expect("Cannot find update fps element");

        *self.render_fps_element.borrow_mut() = Some(render_fps_element);
        *self.update_fps_element.borrow_mut() = Some(update_fps_element);
    }
}

fn format_fps(fps: f64) -> String {
    format!("{:.1}", fps)
}

impl Platform for WasmPlatform {
    fn set_title(&self, _window: &Window, _rom_name: &str, render_fps: f64, update_fps: f64) {
        let fps_str = format_fps(render_fps);
        let update_fps_str = format_fps(update_fps);

        if let Some(e) = self.render_fps_element.borrow().as_ref() {
            e.set_text_content(Some(&fps_str));
        }

        if let Some(e) = self.update_fps_element.borrow().as_ref() {
            e.set_text_content(Some(&update_fps_str));
        }
    }

    fn init_window_hook(&self, window: Arc<Window>, size: PhysicalSize<u32>) {
        let web_window = web_sys::window().expect("no global window exists");
        let document = web_window
            .document()
            .expect("should have a document on window");

        let canvas = window.canvas().expect("Canvas generation failed");
        canvas.set_width(size.width);
        canvas.set_height(size.height);
        canvas.set_id(CANVAS_ID);
        document
            .get_element_by_id(CANVAS_PARENT_ID)
            .unwrap()
            .append_child(&canvas)
            .expect("Canvas append failed");

        self.cache_elements();
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

    fn start_read_binary_file(
        &self,
        path: PathBuf,
        proxy: &AppEventLoopProxy,
        evt_ctor: ReadEventConstructor<Vec<u8>>,
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
        } else {
            let _ = proxy.send_event(evt_ctor(None));
        }
    }

    fn start_read_text_file(
        &self,
        path: PathBuf,
        proxy: &AppEventLoopProxy,
        evt_ctor: ReadEventConstructor<String>,
    ) {
        if let Ok(name) = resolve_filename(&path) {
            let proxy = proxy.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let res = read_text_file(name)
                    .await
                    .inspect_err(|e| {
                        info!("Failed to read text file: {e:?}");
                    })
                    .ok()
                    .flatten();
                let _ = proxy.send_event(evt_ctor(res));
            })
        } else {
            let _ = proxy.send_event(evt_ctor(None));
        }
    }

    fn start_write_binary_file(
        &self,
        path: PathBuf,
        proxy: &AppEventLoopProxy,
        content: &[u8],
        evt_ctor: WriteEventConstructor,
    ) {
        if let Ok(name) = resolve_filename(&path) {
            let content = content.to_vec();
            let proxy = proxy.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let res = write_file(&name, &content).await;
                let ok = res.is_ok();
                if let Err(e) = res {
                    info!("Failed to write save data: {e:?}");
                }
                let _ = proxy.send_event(evt_ctor(path, ok));
            });
        } else {
            let _ = proxy.send_event(evt_ctor(path, false));
        }
    }

    fn start_write_text_file(
        &self,
        path: PathBuf,
        proxy: &AppEventLoopProxy,
        content: &str,
        evt_ctor: WriteEventConstructor,
    ) {
        if let Ok(name) = resolve_filename(&path) {
            let content = content.to_owned();
            let proxy = proxy.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let res = write_text_file(&name, &content).await;
                let ok = res.is_ok();
                if let Err(e) = res {
                    info!("Failed to write text file: {e:?}");
                }
                let _ = proxy.send_event(evt_ctor(path, ok));
            });
        } else {
            let _ = proxy.send_event(evt_ctor(path, false));
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

async fn read_text_file(name: String) -> Result<Option<String>, JsValue> {
    let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

    let value = storage.get_item(&name)?;
    Ok(value)
}

async fn write_text_file(name: &str, content: &str) -> Result<(), JsValue> {
    let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

    storage.set_item(name, content)?;
    Ok(())
}
