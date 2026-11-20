use winit::dpi::PhysicalSize;

/// The width of the inner viewport in pixels.
const INNER_W: u32 = 256;
/// The height of the inner viewport in pixels.
const INNER_H: u32 = 240;
/// The aspect ratio of the inner viewport, computed as `INNER_H` / `INNER_W`.
pub const RATIO: f64 = INNER_H as f64 / INNER_W as f64;

/// The default inner viewport size (256x240) for the NES emulator.
pub const INNER_SIZE: PhysicalSize<u32> = PhysicalSize::new(INNER_W, INNER_H);

/// Returns a PhysicalSize with the given width and a height calculated based on the aspect ratio.
pub const fn width_to_size(width: u32) -> PhysicalSize<u32> {
    let height = ((width as f64) * RATIO) as u32;
    PhysicalSize::new(width, height)
}

/// Compression level used when saving game state (0 = zstd default).
pub const SAVE_COMPRESSION_LEVEL: i32 = 0;

/// File extension for game save files.
pub const SAVE_GAME_EXTENSION: &str = "sav";

/// File extension for state save files.
pub const SAVE_STATE_EXTENSION: &str = "sst";

/// Target frames per second for the application loop.
pub const TARGET_FPS: u64 = 60;
/// Interval in seconds between automatic game saves.
pub const SAVE_INTERVAL: u64 = 10;
/// Number of extra frames to process per update when speedup is active.
pub const FRAMES_MULTIPLIER_SPEEDUP: u32 = 4;

// Number of ticks between press and release of A+B when AB mode is enabled
pub const AUTO_AB_TICK_INTERVAL: u32 = 100;
