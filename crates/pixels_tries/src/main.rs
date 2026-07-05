use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

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

const INNER_WIDTH: u32 = 256;
const INNER_HEIGHT: u32 = 192;

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
    let mut pixels = Pixels::new(INNER_WIDTH, INNER_HEIGHT, surface_texture).unwrap();

    let mut world = World::new(1, 1, 20, 3, INNER_WIDTH, INNER_HEIGHT);

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
    speed: i16,
    w: u32,
    h: u32,
    x_sp: bool,
    y_sp: bool,
    color: [u8; 4],
}

impl World {
    fn new(box_x: i16, box_y: i16, box_size: i16, speed: i16, w: u32, h: u32) -> Self {
        Self {
            box_x,
            box_y,
            box_size,
            speed,
            w,
            h,
            x_sp: true,
            y_sp: true,
            color: [255, 0, 0, 255],
        }
    }

    fn left_x(&self) -> i16 {
        self.box_x
    }

    fn right_x(&self) -> i16 {
        self.box_x + self.box_size
    }

    fn top_y(&self) -> i16 {
        self.box_y
    }

    fn bottom_y(&self) -> i16 {
        self.box_y + self.box_size
    }

    fn update(&mut self) {
        let mut hit = false;
        if self.bottom_y() >= self.h as i16 || self.top_y() <= 0 {
            self.y_sp = !self.y_sp;
            hit = true;
        } else if self.right_x() >= self.w as i16 || self.left_x() <= 0 {
            self.x_sp = !self.x_sp;
            hit = true;
        }

        if hit {
            let inst = (SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                & 0xFFFFFF) as u32;
            self.color = [
                ((inst & 0xFF0000) >> 16) as u8,
                ((inst & 0xFF00) >> 8) as u8,
                (inst & 0xFF) as u8,
                255,
            ];
        }

        self.box_x += (if self.x_sp { 1 } else { -1 }) * self.speed;
        self.box_y += (if self.y_sp { 1 } else { -1 }) * self.speed;
    }

    fn draw(&mut self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % self.w as usize) as i16;
            let y = (i / self.w as usize) as i16;

            let inside_the_box = x >= self.left_x()
                && x < self.right_x()
                && y >= self.top_y()
                && y < self.bottom_y();

            let rgba = if inside_the_box {
                self.color
            } else {
                [0, 0, 0, 0]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
