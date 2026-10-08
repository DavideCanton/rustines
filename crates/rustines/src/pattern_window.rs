use std::sync::Arc;

use pixels::{Pixels, ScalingMode, SurfaceTexture};
use rustines_core::{Mapper, arch::debug_utils::dump_pattern_tables};
use winit::{
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    keyboard::KeyCode,
    window::{Window, WindowAttributes, WindowId},
};
use winit_input_helper::WinitInputHelper;

struct PatternTableWindowState {
    pixels: Pixels<'static>,
    window: Arc<Window>,
    helper: WinitInputHelper,
}

pub struct PatternTableWindow {
    state: Option<PatternTableWindowState>,
}

impl PatternTableWindow {
    pub fn new() -> Self {
        PatternTableWindow { state: None }
    }

    pub fn show(&mut self, mapper: &dyn Mapper, event_loop: &ActiveEventLoop, scale: usize) {
        if self.state.is_some() {
            return;
        }

        let pattern_width = 256 * scale;
        let pattern_height = 128 * scale;
        let buf = dump_pattern_tables(mapper, scale);

        let size = LogicalSize::new(pattern_width as f64, pattern_height as f64);
        let window = Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title("Pattern tables")
                        .with_inner_size(size)
                        .with_min_inner_size(size)
                        .with_resizable(true),
                )
                .unwrap(),
        );

        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, Arc::clone(&window));
        let mut pixels = Pixels::new(pattern_width as u32, pattern_height as u32, surface_texture)
            .expect("Cannot create pixels buffer");
        pixels.set_scaling_mode(ScalingMode::Fill);

        pixels.frame_mut().copy_from_slice(&buf);

        self.state = Some(PatternTableWindowState {
            pixels,
            window,
            helper: WinitInputHelper::new(),
        });
    }

    pub(crate) fn owns_window_event(&self, window_id: WindowId) -> bool {
        self.state
            .as_ref()
            .map(|s| s.window.id() == window_id)
            .unwrap_or(false)
    }

    pub(crate) fn window_event(&mut self, event: &WindowEvent) {
        if let Some(state) = self.state.as_mut()
            && state.helper.process_window_event(event)
        {
            state.pixels.render().expect("Failed to draw");
        }
    }

    pub(crate) fn update(&mut self) {
        let mut close = false;

        if let Some(state) = self.state.as_mut() {
            let helper = &mut state.helper;

            helper.end_step();

            if let Some(size) = helper.window_resized() {
                state
                    .pixels
                    .resize_surface(size.width, size.height)
                    .expect("Failed to resize pattern table surface");
            }

            if helper.key_released(KeyCode::Escape)
                || helper.close_requested()
                || helper.destroyed()
            {
                close = true;
            }
        }

        if close {
            self.state = None;
        }
    }

    pub(crate) fn render(&mut self) {
        if let Some(state) = self.state.as_mut() {
            state.window.request_redraw();
        }
    }

    pub(crate) fn step(&mut self) {
        if let Some(state) = self.state.as_mut() {
            state.helper.step();
        }
    }
}
