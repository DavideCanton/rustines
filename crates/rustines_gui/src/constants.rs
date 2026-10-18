use winit::dpi::PhysicalSize;

const INNER_W: u32 = 256;
const INNER_H: u32 = 240;
pub const RATIO: f64 = INNER_H as f64 / INNER_W as f64;

pub const INNER_SIZE: PhysicalSize<u32> = PhysicalSize::new(INNER_W, INNER_H);

pub const fn width_to_size(width: u32) -> PhysicalSize<u32> {
    let height = ((width as f64) * RATIO) as u32;
    PhysicalSize::new(width, height)
}
