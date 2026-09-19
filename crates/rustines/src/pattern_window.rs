use std::sync::Arc;

use pixels::{Pixels, ScalingMode, SurfaceTexture};
use rustines_core::{Mapper, arch::debug_utils::dump_pattern_tables};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowBuilder},
};

pub struct PatternTableWindow {
    pub window: Arc<Window>,
    pixels: Pixels<'static>,
}

impl PatternTableWindow {
    pub fn create(mapper: &dyn Mapper, target: &EventLoopWindowTarget<()>, scale: usize) -> Self {
        let pattern_width = 256 * scale;
        let pattern_height = 128 * scale;
        let buf = dump_pattern_tables(mapper, scale);

        let size = LogicalSize::new(pattern_width as f64, pattern_height as f64);
        let window = Arc::new(
            WindowBuilder::new()
                .with_title("Pattern tables")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .with_resizable(true)
                .build(target)
                .unwrap(),
        );

        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, Arc::clone(&window));
        let mut pixels = Pixels::new(pattern_width as u32, pattern_height as u32, surface_texture)
            .expect("Cannot create pixels buffer");
        pixels.set_scaling_mode(ScalingMode::Fill);

        pixels.frame_mut().copy_from_slice(&buf);

        PatternTableWindow { window, pixels }
    }

    pub fn draw(&self) {
        self.pixels.render().expect("Failed to draw");
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.pixels
            .resize_surface(size.width, size.height)
            .expect("Failed to resize pattern table surface");
    }
}
