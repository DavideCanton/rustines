use std::{fs::File, io};

pub trait Renderer {
    fn render_pixel(&mut self, x: usize, y: usize, rgb: u32);
    fn draw(&mut self);
    fn dump_frame(&self, file: &mut File) -> io::Result<()>;
}

pub struct NoopRenderer;

impl Renderer for NoopRenderer {
    fn render_pixel(&mut self, _x: usize, _y: usize, _rgb: u32) {}
    fn draw(&mut self) {}
    fn dump_frame(&self, _file: &mut File) -> io::Result<()> {
        Ok(())
    }
}
