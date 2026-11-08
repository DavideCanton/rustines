use std::{cell::RefCell, path::Path, rc::Rc, sync::Arc};

use rustines_core::Renderer;
use winit::{dpi::PhysicalSize, event_loop::EventLoopProxy, window::Window};

use crate::{CustomEvents, app::AppState};

pub trait Platform {
    fn set_title(&self, app_state: &AppState);
    fn init_window_hook(&self, window: Arc<Window>, size: PhysicalSize<u32>);
    fn init_pending_renderer(
        &self,
        main_window: Arc<Window>,
        pending_renderer: Rc<RefCell<Option<Box<dyn Renderer>>>>,
        size: PhysicalSize<u32>,
    );
    fn esc_pressed_enabled(&self) -> bool;
    fn start_read_file(&self, path: &Path, proxy: &EventLoopProxy<CustomEvents>);
    fn start_write_file(&self, path: &Path, content: &[u8]);
    fn should_exit(&self) -> bool;
}
