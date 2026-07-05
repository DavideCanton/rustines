use std::{fs::File, io::Write};

use rustines_core::renderer::Renderer;

pub struct PixelsRenderer<'a> {
    pixels: pixels::Pixels<'a>,
    width: usize,
    #[allow(unused)]
    height: usize,
    frame_cnt: usize,
    log_frames: bool,
}

impl<'a> PixelsRenderer<'a> {
    pub(crate) fn new(pixels: pixels::Pixels<'a>, width: usize, height: usize) -> Self {
        Self {
            pixels,
            width,
            height,
            frame_cnt: 0,
            log_frames: false,
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
        if self.log_frames {
            File::create(format!("frame_{}.bin", self.frame_cnt))
                .expect("Failed to create file")
                .write_all(self.pixels.frame())
                .expect("Failed to write file");
        }

        self.pixels.render().unwrap();
        self.pixels.frame_mut().fill(0);
        self.frame_cnt += 1;
    }
}
