use std::{cell::RefCell, rc::Rc, sync::Arc};

use rustines_core::Renderer;
use rustines_gui::{AppState, Platform, create_renderer};
use winit::{dpi::PhysicalSize, window::Window};

pub struct DesktopPlatform;

impl Platform for DesktopPlatform {
    fn set_title(&self, app_state: &AppState, title: &str) {
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
        let renderer =
            pollster::block_on(create_renderer(Arc::clone(&main_window), size, None)).unwrap();
        *pending_renderer.borrow_mut() = Some(Box::new(renderer));
    }

    fn esc_pressed_enabled(&self) -> bool {
        true
    }
}
