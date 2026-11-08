use base64::{Engine, engine::general_purpose::STANDARD};
use log::info;
use rustines_core::Renderer;
use rustines_gui::{AppState, INNER_SIZE, Platform, create_pixels};
use std::{cell::RefCell, path::Path, rc::Rc, sync::Arc};
use web_sys::Storage;

use crate::renderer::WasmRenderer;
use pixels::wgpu::TextureFormat;
use winit::dpi::PhysicalSize;
use winit::platform::web::WindowExtWebSys;
use winit::window::Window;

pub struct WasmPlatform;

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

    fn read_file(&self, path: &Path) -> Option<Vec<u8>> {
        let storage = local_storage()?;

        let name = path.file_name().inspect_none(|| info!("No file name"))?;
        let name = name
            .to_str()
            .inspect_none(|| info!("Cannot convert path"))?;

        let value = storage
            .get_item(name)
            .inspect_err(|e| {
                info!("Cannot write to local storage: {e:?}");
            })
            .ok()
            .flatten()?;

        STANDARD
            .decode(value)
            .inspect_err(|e| {
                info!("Cannot decode value: {e:?}");
            })
            .ok()
    }

    fn write_file(&self, path: &Path, content: &[u8]) -> bool {
        write_file(path, content).is_some()
    }

    fn should_exit(&self) -> bool {
        false
    }
}

fn write_file(path: &Path, content: &[u8]) -> Option<()> {
    let storage = local_storage()?;

    let name = path.file_name().inspect_none(|| info!("No file name"))?;
    let name = name
        .to_str()
        .inspect_none(|| info!("Cannot convert path"))?;

    let encoded = STANDARD.encode(content);

    storage
        .set_item(name, &encoded)
        .inspect_err(|e| {
            info!("Cannot write to local storage: {e:?}");
        })
        .ok()?;

    Some(())
}

trait InspectNone {
    fn inspect_none(self, f: impl FnOnce()) -> Self;
}

impl<T> InspectNone for Option<T> {
    fn inspect_none(self, f: impl FnOnce()) -> Self {
        if self.is_none() {
            f();
        }
        self
    }
}

fn local_storage() -> Option<Storage> {
    web_sys::window()?
        .local_storage()
        .inspect_err(|e| {
            info!("Cannot read localstorage: {e:?}");
        })
        .ok()?
}
