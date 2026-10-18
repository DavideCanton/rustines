use std::{fs::File, io::Write};

use pixels::Pixels;
use rustines_core::{arch::utils::generate_ppm, renderer::Renderer};

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

    fn dump_frame(&self, file: &mut File) -> std::io::Result<()> {
        let buf = self.pixels.frame();
        let data = generate_ppm(256, 240, buf, true);
        file.write_all(&data)?;
        Ok(())
    }
}
