use std::sync::Arc;

use env_logger::Builder;
use log::LevelFilter;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use rustines_gui_utils::{FpsCounter, FpsLimiter};

fn init_logger() {
    let mut builder = Builder::from_default_env();

    builder.filter(None, LevelFilter::Info).init();
}

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

pub fn main() {
    init_logger();

    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();

    let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Try")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap(),
    );

    let window_size = window.inner_size();
    let surface_texture =
        SurfaceTexture::new(window_size.width, window_size.height, Arc::clone(&window));
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();

    let mut world = World {
        box_size: 100,
        box_x: 1,
        box_y: 1,
    };

    let mut limiter = FpsLimiter::new(60.0);
    let mut counter = FpsCounter::new();

    let _ = event_loop.run(|event, elwt| {
        if input.update(&event) {
            // Close events
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }

            limiter.update();

            world.update();
            window.request_redraw();
        }

        // Draw the current frame
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            world.draw(pixels.frame_mut());
            pixels.render().unwrap();

            if let Some(fps) = counter.drawn() {
                window.set_title(&format!("Try | FPS: {:.1}", fps));
            }
        }
    });
}

struct World {
    box_x: i16,
    box_y: i16,
    box_size: i16,
}

impl World {
    fn update(&mut self) {
        self.box_x += 1;
        self.box_y += 1;
    }

    fn draw(&mut self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let inside_the_box = x >= self.box_x
                && x < self.box_x + self.box_size
                && y >= self.box_y
                && y < self.box_y + self.box_size;

            let rgba = if inside_the_box {
                [255, 0, 0, 255]
            } else {
                [0, 0, 0, 0]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
