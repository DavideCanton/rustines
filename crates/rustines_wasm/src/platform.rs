use std::{cell::RefCell, rc::Rc, sync::Arc};

use rustines_core::Renderer;
use rustines_gui::{AppState, INNER_SIZE, Platform, create_pixels};

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

        document
            .get_element_by_id("filename")
            .map(|e| e.set_text_content(Some(app_state.rom_name())));

        document
            .get_element_by_id("r_fps")
            .map(|e| e.set_text_content(Some(&format_fps(app_state.render_fps()))));

        document
            .get_element_by_id("u_fps")
            .map(|e| e.set_text_content(Some(&format_fps(app_state.update_fps()))));
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
}
