use winit::dpi::PhysicalSize;

/// The width of the inner viewport in pixels.
const INNER_W: u32 = 256;
/// The height of the inner viewport in pixels.
const INNER_H: u32 = 240;
/// The aspect ratio of the inner viewport, computed as `INNER_H` / `INNER_W`.
pub const RATIO: f64 = INNER_H as f64 / INNER_W as f64;

pub const INNER_SIZE: PhysicalSize<u32> = PhysicalSize::new(INNER_W, INNER_H);

/// Returns a PhysicalSize with the given width and a height calculated based on the aspect ratio.
pub const fn width_to_size(width: u32) -> PhysicalSize<u32> {
    let height = ((width as f64) * RATIO) as u32;
    PhysicalSize::new(width, height)
}
