use pixels::{Pixels, ScalingMode, SurfaceTexture};
use rustines_core::{Mapper, arch::debug_utils::dump_pattern_tables};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event_loop::EventLoopWindowTarget,
    window::{WindowBuilder, WindowId},
};

struct PatternTableWindowState {
    pixels: Pixels<'static>,
    window_id: WindowId,
}

pub struct PatternTableWindow {
    state: Option<PatternTableWindowState>,
}

impl PatternTableWindow {
    pub fn new() -> Self {
        PatternTableWindow { state: None }
    }

    pub fn show(&mut self, mapper: &dyn Mapper, target: &EventLoopWindowTarget<()>, scale: usize) {
        let pattern_width = 256 * scale;
        let pattern_height = 128 * scale;
        let buf = dump_pattern_tables(mapper, scale);

        let size = LogicalSize::new(pattern_width as f64, pattern_height as f64);
        let window = WindowBuilder::new()
            .with_title("Pattern tables")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(true)
            .build(target)
            .unwrap();

        let window_id = window.id();

        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);
        let mut pixels = Pixels::new(pattern_width as u32, pattern_height as u32, surface_texture)
            .expect("Cannot create pixels buffer");
        pixels.set_scaling_mode(ScalingMode::Fill);

        pixels.frame_mut().copy_from_slice(&buf);

        self.state = Some(PatternTableWindowState { pixels, window_id });
    }

    pub(crate) fn is_event(&self, target_window_id: WindowId) -> bool {
        match &self.state {
            Some(PatternTableWindowState { window_id, .. }) => *window_id == target_window_id,
            None => false,
        }
    }

    pub(crate) fn draw(&self) {
        if let Some(PatternTableWindowState { pixels, .. }) = &self.state {
            pixels.render().expect("Failed to draw");
        }
    }

    pub(crate) fn resize(&mut self, size: PhysicalSize<u32>) {
        if let Some(PatternTableWindowState { pixels, .. }) = &mut self.state {
            pixels
                .resize_surface(size.width, size.height)
                .expect("Failed to resize pattern table surface");
        }
    }

    pub(crate) fn is_displayed(&self) -> bool {
        matches!(&self.state, Some(PatternTableWindowState { .. }))
    }
}
