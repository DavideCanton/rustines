use std::{
    fs::{File, create_dir, remove_dir_all},
    io::Write,
};

use pixels::Pixels;
use rustines_core::{arch::debug_utils::generate_ppm, renderer::Renderer};

const FRAME_INTERVAL: Option<usize> = None;

pub struct PixelsRenderer {
    pixels: Pixels<'static>,
    width: usize,
    #[allow(unused)]
    height: usize,
    frame_cnt: usize,
    frame_interval: Option<usize>,
}

impl PixelsRenderer {
    pub(crate) fn new(pixels: Pixels<'static>, width: usize, height: usize) -> Self {
        if FRAME_INTERVAL.is_some() {
            let _ = remove_dir_all("frames");
            let _ = create_dir("frames");
        }

        Self {
            pixels,
            width,
            height,
            frame_cnt: 0,
            frame_interval: FRAME_INTERVAL,
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
        if let Some(fi) = self.frame_interval
            && self.frame_cnt.is_multiple_of(fi)
        {
            let mut file = File::create(format!("frames/frame_{}.ppm", self.frame_cnt))
                .expect("Failed to create file");

            let data = generate_ppm(256, 240, self.pixels.frame(), true);

            file.write_all(&data).expect("Failed to write file");
        }

        self.pixels.render().unwrap();
        self.frame_cnt += 1;
    }
}
