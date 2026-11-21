use std::{fs::File, io::Write};

use pixels::Pixels;
use rustines_core::{MessageLocation, Renderer, arch::utils::generate_ppm};
use web_time::{Duration, Instant};

struct MessageState {
    until: Instant,
    _current_message: String,
    location: MessageLocation,
}

pub struct PixelsRenderer {
    pixels: Pixels<'static>,
    width: usize,
    #[allow(unused)]
    height: usize,
    message: Option<MessageState>,
}

impl PixelsRenderer {
    pub fn new(pixels: Pixels<'static>, width: usize, height: usize) -> Self {
        Self {
            pixels,
            width,
            height,
            message: None,
        }
    }

    #[inline(always)]
    fn to_index(&self, x: usize, y: usize) -> usize {
        (y * self.width + x) * 4
    }

    fn try_rendering_message(&mut self) {
        self.message = match self.message.take() {
            Some(msg) => {
                if Instant::now() <= msg.until {
                    let w = 20;
                    let h = 16;
                    let x_start = if msg.location.is_left() {
                        0
                    } else {
                        self.width - w
                    };
                    let y_start = if msg.location.is_top() {
                        0
                    } else {
                        self.height - h
                    };

                    for x in x_start..x_start + w {
                        for y in y_start..y_start + h {
                            let i = self.to_index(x, y);
                            let buf = self.pixels.frame_mut();
                            // TODO render message instead
                            buf[i..i + 4].copy_from_slice(&0xFF000000u32.to_be_bytes())
                        }
                    }
                    Some(msg)
                } else {
                    None
                }
            }
            None => None,
        };
    }
}

impl Renderer for PixelsRenderer {
    fn render_pixel(&mut self, x: usize, y: usize, rgba: u32) {
        let i = self.to_index(x, y);
        let buf = self.pixels.frame_mut();
        buf[i..i + 4].copy_from_slice(&rgba.to_be_bytes());
    }

    fn draw(&mut self) {
        self.try_rendering_message();
        self.pixels.render().unwrap();
    }

    fn dump_frame(&self, file: &mut File) -> std::io::Result<()> {
        let buf = self.pixels.frame();
        let data = generate_ppm(256, 240, buf, true);
        file.write_all(&data)?;
        Ok(())
    }

    fn display_message(&mut self, message: &str, seconds: u64, location: MessageLocation) {
        self.message = Some(MessageState {
            until: Instant::now() + Duration::from_secs(seconds),
            _current_message: message.to_string(),
            location,
        })
    }
}
