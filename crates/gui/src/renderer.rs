use std::{fs::File, io::Write};

use fontdue::{
    Font, FontSettings,
    layout::{CoordinateSystem, GlyphPosition, Layout, TextStyle},
};
use pixels::Pixels;
use rustines_core::{MessageLocation, Renderer, arch::utils::generate_ppm};
use web_time::{Duration, Instant};

const FONT_PADDING: usize = 1;
const WHITE: u32 = 0xFFFFFFFF;
const BLACK: u32 = 0x000000FF;

const BG_COLOR: u32 = BLACK;
const TEXT_COLOR: u32 = WHITE;

struct RenderedGlyph {
    glyph_pos: GlyphPosition,
    bitmap: Vec<u8>,
}

struct RenderedBox {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

struct RenderCache {
    glyphs: Vec<RenderedGlyph>,
    text_box: RenderedBox,
    until: Instant,
}

pub struct PixelsRenderer {
    pixels: Pixels<'static>,
    width: usize,
    #[allow(unused)]
    height: usize,
    font: Font,
    layout: Layout,
    cache: Option<RenderCache>,
}

impl PixelsRenderer {
    pub fn new(pixels: Pixels<'static>, width: usize, height: usize) -> Self {
        let font_data = include_bytes!("../fonts/PublicPixel.ttf");
        let font =
            Font::from_bytes(&font_data[..], FontSettings::default()).expect("Failed to load font");
        let layout = Layout::new(CoordinateSystem::PositiveYDown);

        Self {
            pixels,
            width,
            height,
            font,
            layout,
            cache: None,
        }
    }

    fn render_text(&mut self) {
        if self.cache.is_none() {
            return;
        }

        let cache = self.cache.as_ref().unwrap();
        let text_box = &cache.text_box;

        for y in text_box.y..text_box.y + text_box.height {
            for x in text_box.x..text_box.x + text_box.width {
                render_pixel_into_buf(self.pixels.frame_mut(), x, y, self.width, BG_COLOR);
            }
        }

        for g in &cache.glyphs {
            let w = g.glyph_pos.width;
            let h = g.glyph_pos.height;
            let x = text_box.x + FONT_PADDING + g.glyph_pos.x.round() as usize;
            let y = text_box.y + FONT_PADDING + g.glyph_pos.y.round() as usize;

            for by in 0..h {
                for bx in 0..w {
                    let alpha = g.bitmap[by * w + bx];
                    if alpha >= 128 {
                        let px = x + bx;
                        let py = y + by;
                        render_pixel_into_buf(
                            self.pixels.frame_mut(),
                            px,
                            py,
                            self.width,
                            TEXT_COLOR,
                        );
                    }
                }
            }
        }
    }

    fn try_rendering_message(&mut self) {
        if let Some(cache) = self.cache.as_ref() {
            if Instant::now() <= cache.until {
                self.render_text();
            } else {
                self.cache = None;
            }
        };
    }
}

impl Renderer for PixelsRenderer {
    #[inline(always)]
    fn render_pixel(&mut self, x: usize, y: usize, rgba: u32) {
        let buf = self.pixels.frame_mut();
        render_pixel_into_buf(buf, x, y, self.width, rgba);
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

    fn display_message(
        &mut self,
        message: &str,
        seconds: u64,
        font_size: f32,
        location: MessageLocation,
    ) {
        self.layout.clear();
        self.layout.append(
            &[&self.font],
            &TextStyle {
                text: &message.to_uppercase(),
                px: font_size,
                font_index: 0,
                user_data: (),
            },
        );

        let glyphs = self.layout.glyphs().clone();
        if glyphs.is_empty() {
            return;
        }

        let last = glyphs.last().unwrap();
        let width = ((last.x.round() as usize + last.width) + FONT_PADDING * 2).min(self.width);
        let height = (glyphs
            .iter()
            .map(|g| g.y.round() as usize + g.height)
            .max()
            .unwrap()
            + FONT_PADDING * 2)
            .min(self.height);

        let x_start = if location.is_left() {
            0
        } else {
            self.width.saturating_sub(width)
        };
        let y_start = if location.is_top() {
            0
        } else {
            self.height.saturating_sub(height)
        };

        self.cache = Some(RenderCache {
            until: Instant::now() + Duration::from_secs(seconds),
            glyphs: glyphs
                .into_iter()
                .map(|g| RenderedGlyph {
                    glyph_pos: g,
                    bitmap: self.font.rasterize_config(g.key).1,
                })
                .collect(),
            text_box: RenderedBox {
                x: x_start,
                y: y_start,
                width,
                height,
            },
        })
    }
}

fn render_pixel_into_buf(buf: &mut [u8], x: usize, y: usize, w: usize, rgba: u32) {
    let i = (y * w + x) * 4;
    buf[i..i + 4].copy_from_slice(&rgba.to_be_bytes());
}
