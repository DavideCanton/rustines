use rustines_core::renderer::Renderer;

pub struct PixelsRenderer;

impl Renderer for PixelsRenderer {
    fn render_pixel(&mut self, _x: usize, _y: usize, _rgb: u32) {}
}
