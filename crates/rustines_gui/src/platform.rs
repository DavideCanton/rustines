use std::{cell::RefCell, rc::Rc, sync::Arc};

use rustines_core::Renderer;
use winit::{dpi::PhysicalSize, window::Window};

use crate::app::AppState;

pub trait Platform {
    fn set_title(&self, app_state: &AppState, title: &str);
    fn init_window_hook(&self, window: Arc<Window>, size: PhysicalSize<u32>);
    fn init_pending_renderer(
        &self,
        main_window: Arc<Window>,
        pending_renderer: Rc<RefCell<Option<Box<dyn Renderer>>>>,
        size: PhysicalSize<u32>,
    );
    fn esc_pressed_enabled(&self) -> bool;
}
