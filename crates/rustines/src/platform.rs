use std::{cell::RefCell, rc::Rc, sync::Arc};

use rustines_core::Renderer;
use rustines_gui::{AppState, INNER_SIZE, PixelsRenderer, Platform, create_pixels};
use winit::{dpi::PhysicalSize, window::Window};

pub struct DesktopPlatform;

impl Platform for DesktopPlatform {
    fn set_title(&self, app_state: &AppState) {
        let r_fps = format_fps(app_state.render_fps(), "FPS");
        let u_fps = format_fps(app_state.update_fps(), "Simulation FPS");

        let title = format!("{} | {r_fps} | {u_fps}", app_state.rom_name());

        app_state
            .main_window()
            .set_title(&format!("Rustines | {title}"));
    }

    fn init_window_hook(&self, _window: Arc<Window>, _size: PhysicalSize<u32>) {}

    fn init_pending_renderer(
        &self,
        main_window: Arc<Window>,
        pending_renderer: Rc<RefCell<Option<Box<dyn Renderer>>>>,
        size: PhysicalSize<u32>,
    ) {
        let pixels =
            pollster::block_on(create_pixels(Arc::clone(&main_window), size, None)).unwrap();

        let renderer = PixelsRenderer::new(
            pixels,
            INNER_SIZE.width as usize,
            INNER_SIZE.height as usize,
        );

        *pending_renderer.borrow_mut() = Some(Box::new(renderer));
    }

    fn esc_pressed_enabled(&self) -> bool {
        true
    }
}

fn format_fps(fps: f64, msg: &str) -> String {
    format!("{}: {:.1}", msg, fps)
}
