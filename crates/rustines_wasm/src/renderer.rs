use pixels::Pixels;
use rustines_core::{Renderer, compose_rgba, decompose_rgba};
use rustines_gui::PixelsRenderer;
use std::fs::File;
use std::io;

pub struct WasmRenderer {
    renderer: PixelsRenderer,
    cache_color: Vec<u8>,
}

impl WasmRenderer {
    pub fn new(pixels: Pixels<'static>, width: usize, height: usize) -> Self {
        Self {
            renderer: PixelsRenderer::new(pixels, width, height),
            // cache color correction
            cache_color: (0u8..=255)
                .into_iter()
                .map(|c| ((c as f32 / 255.0).powf(1.0 / 2.2) * 255.0) as u8)
                .collect::<Vec<_>>(),
        }
    }

    fn fix_color(&mut self, c: u8) -> u8 {
        self.cache_color[c as usize]
    }
}

impl Renderer for WasmRenderer {
    fn render_pixel(&mut self, x: usize, y: usize, rgba: u32) {
        // apply color correction since colors on canvas are not rendered linearly
        let [r, g, b, a] = decompose_rgba(rgba);
        let rgba = compose_rgba(self.fix_color(r), self.fix_color(g), self.fix_color(b), a);
        self.renderer.render_pixel(x, y, rgba)
    }

    fn draw(&mut self) {
        self.renderer.draw()
    }

    fn dump_frame(&self, _file: &mut File) -> io::Result<()> {
        // don't implement this for web
        Ok(())
    }
}
