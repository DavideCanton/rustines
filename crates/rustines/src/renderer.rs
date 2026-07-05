use rustines_core::renderer::Renderer;

pub struct PixelsRenderer<'a> {
    pixels: pixels::Pixels<'a>,
    width: usize,
    #[allow(unused)]
    height: usize,
}

impl<'a> PixelsRenderer<'a> {
    pub(crate) fn new(pixels: pixels::Pixels<'a>, width: usize, height: usize) -> Self {
        Self {
            pixels,
            width,
            height,
        }
    }
}

impl<'a> Renderer for PixelsRenderer<'a> {
    fn render_pixel(&mut self, x: usize, y: usize, rgba: u32) {
        let i = (y * self.width + x) * 4;
        let buf = self.pixels.frame_mut();
        buf[i..i + 4].copy_from_slice(&rgba.to_be_bytes());
    }

    fn draw(&mut self) {
        self.pixels.render().unwrap();
        self.pixels.frame_mut().fill(0);
    }
}
