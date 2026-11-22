use std::{fs::File, io};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum MessageLocation {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl MessageLocation {
    pub fn is_top(&self) -> bool {
        matches!(self, MessageLocation::TopLeft | MessageLocation::TopRight)
    }

    pub fn is_left(&self) -> bool {
        matches!(self, MessageLocation::TopLeft | MessageLocation::BottomLeft)
    }
}

pub trait Renderer {
    fn render_pixel(&mut self, x: usize, y: usize, rgba: u32);
    fn draw(&mut self);
    fn dump_frame(&self, file: &mut File) -> io::Result<()>;
    fn display_message(
        &mut self,
        message: &str,
        secs: u64,
        font_size: f32,
        location: MessageLocation,
    );
}

pub struct NoopRenderer;

impl Renderer for NoopRenderer {
    fn render_pixel(&mut self, _x: usize, _y: usize, _rgba: u32) {}
    fn draw(&mut self) {}
    fn dump_frame(&self, _file: &mut File) -> io::Result<()> {
        Ok(())
    }
    fn display_message(
        &mut self,
        _message: &str,
        _secs: u64,
        _font_size: f32,
        _location: MessageLocation,
    ) {
    }
}

pub type RendererBox = Box<dyn Renderer>;
