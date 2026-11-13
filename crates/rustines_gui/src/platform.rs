use std::{cell::RefCell, path::PathBuf, rc::Rc, sync::Arc};

use rustines_core::Renderer;
use winit::{dpi::PhysicalSize, window::Window};

use crate::{AppEventLoopProxy, AppState, CustomEvents};

pub type PendingRenderer = Rc<RefCell<Option<Box<dyn Renderer>>>>;
pub type ReadEventConstructor = Box<dyn FnOnce(Option<Vec<u8>>) -> CustomEvents>;
pub type WriteEventConstructor = Box<dyn FnOnce(PathBuf, bool) -> CustomEvents>;

pub trait Platform {
    fn set_title(&self, app_state: &AppState);
    fn init_window_hook(&self, window: Arc<Window>, size: PhysicalSize<u32>);
    fn init_pending_renderer(
        &self,
        main_window: Arc<Window>,
        pending_renderer: PendingRenderer,
        size: PhysicalSize<u32>,
    );
    fn esc_pressed_enabled(&self) -> bool;
    fn start_read_file(
        &self,
        path: PathBuf,
        proxy: &AppEventLoopProxy,
        evt_ctor: ReadEventConstructor,
    );
    fn start_write_file(
        &self,
        path: PathBuf,
        proxy: &AppEventLoopProxy,
        content: &[u8],
        evt_ctor: WriteEventConstructor,
    );
    fn should_exit(&self) -> bool;
}
