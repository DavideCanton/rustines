pub trait Renderer {
    fn render_pixel(&mut self, x: usize, y: usize, rgb: u32);
    fn draw(&mut self);
    fn clear(&mut self);
}

pub struct NoopRenderer;

impl Renderer for NoopRenderer {
    fn render_pixel(&mut self, _x: usize, _y: usize, _rgb: u32) {}
    fn draw(&mut self) {}
    fn clear(&mut self) {}
}
