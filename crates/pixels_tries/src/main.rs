use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use flexi_logger::{LogSpecBuilder, Logger, LoggerHandle};
use log::LevelFilter;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Window, WindowAttributes, WindowId},
};

use rustines_gui_utils::{FpsCounter, FpsLimiter};

#[must_use]
fn init_logger() -> LoggerHandle {
    let builder = Logger::with(LogSpecBuilder::new().default(LevelFilter::Info).build());

    builder.start().expect("Failed to start logger")
}

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

const INNER_WIDTH: u32 = 256;
const INNER_HEIGHT: u32 = 192;

struct AppState {
    window: Arc<Window>,
    pixels: Pixels<'static>,
    world: World,
    limiter: FpsLimiter,
    counter: FpsCounter,
}

#[derive(Default)]
struct App {
    state: Option<AppState>,
}

impl ApplicationHandler for App {
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(state) = self.state.as_mut() {
            state.world.update();
            state.limiter.update();
            state.window.request_redraw();
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);

        let window = Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title("Try")
                        .with_inner_size(size)
                        .with_min_inner_size(size),
                )
                .unwrap(),
        );

        let window_size = window.inner_size();

        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, Arc::clone(&window));

        let pixels = Pixels::new(INNER_WIDTH, INNER_HEIGHT, surface_texture).unwrap();

        let world = World::new(1, 1, 20, 3, INNER_WIDTH, INNER_HEIGHT);

        let limiter = FpsLimiter::new(60.0);
        let counter = FpsCounter::new();

        let state = AppState {
            counter,
            limiter,
            pixels,
            window,
            world,
        };
        self.state = Some(state);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(state) = self.state.as_mut() {
            match event {
                WindowEvent::RedrawRequested => {
                    state.world.draw(state.pixels.frame_mut());
                    state.pixels.render().unwrap();

                    if let Some(fps) = state.counter.drawn() {
                        state.window.set_title(&format!("Try | FPS: {:.1}", fps));
                    }
                }
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            logical_key: Key::Named(NamedKey::Escape),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    event_loop.exit();
                }
                _ => {}
            }
        }
    }
}

pub fn main() {
    let _logger = init_logger();

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
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
                & 0b1111_1111_1111_1111_1111_1111) as u32;
            self.color = [
                ((inst & 0b1111_1111_0000_0000_0000_0000) >> 16) as u8,
                ((inst & 0b1111_1111_0000_0000) >> 8) as u8,
                (inst & 0b1111_1111) as u8,
                255,
            ];
        }

        self.box_x += (if self.x_sp { 1 } else { -1 }) * self.speed;
        self.box_y += (if self.y_sp { 1 } else { -1 }) * self.speed;
    }

    fn draw(&mut self, frame: &mut [u8]) {
        let chunks = frame.as_chunks_mut::<4>().0;

        for (i, pixel) in chunks.iter_mut().enumerate() {
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
