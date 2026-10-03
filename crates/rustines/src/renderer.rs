use pixels::Pixels;
use rustines_core::renderer::Renderer;

pub struct PixelsRenderer {
    pixels: Pixels<'static>,
    width: usize,
    #[allow(unused)]
    height: usize,
}

impl PixelsRenderer {
    pub(crate) fn new(pixels: Pixels<'static>, width: usize, height: usize) -> Self {
        Self {
            pixels,
            width,
            height,
        }
    }
}

impl Renderer for PixelsRenderer {
    fn render_pixel(&mut self, x: usize, y: usize, rgba: u32) {
        let i = (y * self.width + x) * 4;
        let buf = self.pixels.frame_mut();
        buf[i..i + 4].copy_from_slice(&rgba.to_be_bytes());
    }

    fn draw(&mut self) {
        self.pixels.render().unwrap();
    }
}
