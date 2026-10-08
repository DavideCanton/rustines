mod args;
mod pattern_window;
mod renderer;
mod utils;

use crate::{
    args::RustinesArgs,
    pattern_window::PatternTableWindow,
    renderer::PixelsRenderer,
    utils::{init_logger, read_file},
};
use clap::Parser;
use log::info;
use pixels::{PixelsBuilder, ScalingMode, SurfaceTexture};

use rustines_core::{self as core, arch::bus::Controller2};
use rustines_gui_utils::FpsCounter;
#[cfg(not(target_arch = "wasm32"))]
use rustines_gui_utils::FpsLimiter;
#[cfg(target_arch = "wasm32")]
use std::{cell::RefCell, rc::Rc};
use std::{collections::HashMap, path, sync::Arc};
#[cfg(target_arch = "wasm32")]
use winit::event_loop::ControlFlow;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{ElementState, MouseButton, StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::KeyCode,
    window::{Window, WindowAttributes},
};
use winit_input_helper::WinitInputHelper;

const INNER_W: u32 = 256;
const INNER_H: u32 = 240;
const WIDTH: u32 = 1024;
const RATIO: f64 = INNER_H as f64 / INNER_W as f64;
const HEIGHT: u32 = (WIDTH as f64 * RATIO) as u32;

type KeyMap = HashMap<KeyCode, rustines_core::NesKey>;

struct AppState {
    bus: core::Bus,
    cpu: core::Cpu,

    #[cfg(not(target_arch = "wasm32"))]
    limiter: FpsLimiter,
    counter: FpsCounter,
    log_point: u32,

    key_map1: KeyMap,
    key_map2: KeyMap,

    pattern_window: PatternTableWindow,

    main_window: Arc<Window>,
    main_window_helper: WinitInputHelper,

    cursor_position: Option<(f64, f64)>,
    zapper_trigger_pressed: bool,
    zapper_needs_update: bool,

    pause: bool,
}

pub struct App {
    app_state: Option<AppState>,
    mapper: Option<core::MapperBox>,
    trace_boot: bool,
    zapper: bool,
    init_window_hook: Option<fn(Arc<Window>)>,
    #[cfg(target_arch = "wasm32")]
    pending_window: Option<Arc<Window>>,
    #[cfg(target_arch = "wasm32")]
    pending_renderer: Rc<RefCell<Option<Result<PixelsRenderer, String>>>>,
}

impl App {
    pub fn new(
        mapper: core::MapperBox,
        trace_boot: bool,
        zapper: bool,
        init_window_hook: Option<fn(Arc<Window>)>,
    ) -> Self {
        App {
            app_state: None,
            mapper: Some(mapper),
            trace_boot,
            zapper,
            init_window_hook,
            #[cfg(target_arch = "wasm32")]
            pending_window: None,
            #[cfg(target_arch = "wasm32")]
            pending_renderer: Rc::new(RefCell::new(None)),
        }
    }

    fn init_app_state(&mut self, main_window: Arc<Window>, renderer: PixelsRenderer) -> AppState {
        let ppu = core::Ppu::new(Box::new(renderer));
        let apu = core::Apu::default();

        let ctrl2 = if self.zapper {
            Controller2::zapper()
        } else {
            Controller2::nes_controller()
        };

        let mut bus = core::Bus::new(self.mapper.take().unwrap(), ppu, apu, ctrl2);
        let mut cpu = core::Cpu::new();

        if self.trace_boot {
            cpu.enable_tracing(true);
            bus.enable_tracing(true);
            bus.ppu_mut().enable_tracing(true);
        }

        AppState {
            bus,
            counter: FpsCounter::new(),
            cpu,
            key_map1: build_keymap_c1(),
            key_map2: build_keymap_c2(),
            #[cfg(not(target_arch = "wasm32"))]
            limiter: FpsLimiter::new(60.0),
            log_point: 1,
            pattern_window: PatternTableWindow::new(),
            main_window,
            pause: false,
            main_window_helper: WinitInputHelper::new(),
            cursor_position: None,
            zapper_trigger_pressed: false,
            zapper_needs_update: false,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let main_window = Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title("Rustines")
                        .with_inner_size(size)
                        .with_min_inner_size(size),
                )
                .unwrap(),
        );

        if let Some(f) = self.init_window_hook {
            (f)(Arc::clone(&main_window));
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let renderer = pollster::block_on(create_renderer(Arc::clone(&main_window))).unwrap();
            self.app_state = Some(self.init_app_state(main_window, renderer));
        }

        #[cfg(target_arch = "wasm32")]
        {
            self.pending_window = Some(Arc::clone(&main_window));
            let pending_renderer = Rc::clone(&self.pending_renderer);
            wasm_bindgen_futures::spawn_local(async move {
                let renderer = create_renderer(Arc::clone(&main_window)).await;
                *pending_renderer.borrow_mut() = Some(renderer);
                main_window.request_redraw();
            });
        }
    }

    fn new_events(&mut self, _: &ActiveEventLoop, _: StartCause) {
        if let Some(app_state) = self.app_state.as_mut() {
            app_state.main_window_helper.step();
            app_state.pattern_window.step();
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let Some(app_state) = self.app_state.as_mut() {
            if window_id == app_state.main_window.id() {
                if app_state.bus.zapper_mut().is_some() {
                    match &event {
                        WindowEvent::CursorMoved { position, .. } => {
                            app_state.cursor_position = Some((position.x, position.y));
                            app_state.zapper_needs_update = true;
                        }
                        WindowEvent::CursorLeft { .. } => {
                            app_state.cursor_position = None;
                            app_state.zapper_needs_update = true;
                        }
                        WindowEvent::MouseInput {
                            state,
                            button: MouseButton::Left,
                            ..
                        } => {
                            app_state.zapper_trigger_pressed = *state == ElementState::Pressed;
                            app_state.zapper_needs_update = true;
                        }
                        _ => {}
                    }
                }

                if app_state.main_window_helper.process_window_event(&event) {
                    // Draw the current frame
                    app_state.bus.ppu_mut().renderer().draw();

                    if let Some(fps) = app_state.counter.drawn() {
                        app_state
                            .main_window
                            .set_title(&format!("Rustines | FPS: {:.1}", fps));
                    }
                }
            } else if app_state.pattern_window.owns_window_event(window_id) {
                app_state.pattern_window.window_event(&event);
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        #[cfg(target_arch = "wasm32")]
        {
            event_loop.set_control_flow(ControlFlow::wait_duration(
                web_time::Duration::from_millis(16),
            ));

            if self.app_state.is_none() {
                let renderer = self.pending_renderer.borrow_mut().take();
                if let Some(renderer) = renderer {
                    let window = self.pending_window.take().unwrap();
                    let renderer = renderer.expect("Failed to initialize renderer");
                    self.app_state = Some(self.init_app_state(window, renderer));
                }
            }
        }

        if let Some(app_state) = self.app_state.as_mut() {
            update_logic(app_state, event_loop);
        }
    }
}

pub fn main() {
    let args = RustinesArgs::parse();

    let _logger_handle = init_logger(args.log_file, args.trace_level);

    let file_path = path::PathBuf::from(&args.file_path);

    info!("Using input file: {}", args.file_path);

    let (_, mapper) = read_file(&file_path).unwrap();

    let event_loop = EventLoop::new().unwrap();

    let mut app = App::new(mapper, args.trace_boot, args.zapper, None);

    let _ = event_loop.run_app(&mut app);
}

fn update_logic(app_state: &mut AppState, event_loop: &ActiveEventLoop) {
    let input = &mut app_state.main_window_helper;

    input.end_step();

    if input.key_pressed(KeyCode::Escape) || input.close_requested() {
        event_loop.exit();
    } else {
        handle_debug_keys(app_state, event_loop);

        // reborrow
        let input = &app_state.main_window_helper;

        handle_inputs(input, app_state.bus.controller1_mut(), &app_state.key_map1);
        if let Some(ctrl2) = app_state.bus.controller2_mut() {
            handle_inputs(input, ctrl2, &app_state.key_map2);
        }

        if app_state.bus.zapper_mut().is_some() && app_state.zapper_needs_update {
            update_zapper_input(app_state);
        }

        if !app_state.pause {
            while !app_state.bus.ppu().frame_ready() {
                app_state.cpu.tick(&mut app_state.bus);
            }
            app_state.bus.ppu_mut().clear_frame_ready();
            #[cfg(not(target_arch = "wasm32"))]
            app_state.limiter.update();
        }

        app_state.main_window.request_redraw();
    }

    app_state.pattern_window.update();
}

fn update_zapper_input(app_state: &mut AppState) {
    let size = app_state.main_window.inner_size();
    let position = app_state.cursor_position.and_then(|(x, y)| {
        (size.width > 0 && size.height > 0).then(|| {
            (
                (x * f64::from(INNER_W) / f64::from(size.width)) as usize,
                (y * f64::from(INNER_H) / f64::from(size.height)) as usize,
            )
        })
    });

    app_state
        .bus
        .set_zapper_input(position, app_state.zapper_trigger_pressed);
}

fn handle_debug_keys(app_state: &mut AppState, event_loop: &ActiveEventLoop) {
    let input = &app_state.main_window_helper;

    let bus = &mut app_state.bus;
    let cpu = &mut app_state.cpu;

    if input.held_shift() {
        if input.key_pressed(KeyCode::KeyD) {
            core::debug_utils::debug_dump_nametables(bus);
        }

        if input.key_pressed(KeyCode::KeyP) {
            core::debug_utils::debug_dump_palette(bus);
        }

        if input.key_pressed(KeyCode::KeyO) {
            core::debug_utils::debug_dump_oam(bus);
        }

        if input.key_pressed(KeyCode::KeyX) {
            app_state.pause = !app_state.pause;
        }

        if input.key_pressed(KeyCode::KeyQ) {
            core::debug_utils::debug_dump_state(bus, cpu);
        }

        if input.key_pressed(KeyCode::KeyT) {
            let log_point = app_state.log_point;
            println!("LOG_POINT {}", log_point);
            info!("LOG_POINT {}", log_point);

            app_state.log_point += 1;

            cpu.enable_tracing(true);
            bus.enable_tracing(true);
            bus.ppu_mut().enable_tracing(true);
        }

        if input.key_pressed(KeyCode::KeyS) {
            app_state
                .pattern_window
                .show(bus.mapper_ref(), event_loop, 4);
        }
    }
}

async fn create_renderer(window: Arc<Window>) -> Result<PixelsRenderer, String> {
    let (surface_width, surface_height) = (WIDTH, HEIGHT);

    if surface_width == 0 || surface_height == 0 {
        return Err(format!(
            "Window surface has invalid size: {surface_width}x{surface_height}"
        ));
    }

    let surface_texture = SurfaceTexture::new(surface_width, surface_height, window);
    let builder = PixelsBuilder::new(INNER_W, INNER_H, surface_texture);
    #[cfg(target_arch = "wasm32")]
    let builder = builder.surface_texture_format(pixels::wgpu::TextureFormat::Bgra8Unorm);
    let mut pixels = builder.build_async().await.map_err(|e| format!("{}", e))?;
    pixels.set_scaling_mode(ScalingMode::Fill);

    Ok(PixelsRenderer::new(
        pixels,
        INNER_W as usize,
        INNER_H as usize,
    ))
}

fn build_keymap_c1() -> HashMap<KeyCode, core::NesKey> {
    // TODO read from config file
    use core::NesKey;

    let mut key_map = HashMap::new();
    key_map.insert(KeyCode::KeyZ, NesKey::A);
    key_map.insert(KeyCode::KeyX, NesKey::B);
    key_map.insert(KeyCode::Space, NesKey::Select);
    key_map.insert(KeyCode::Enter, NesKey::Start);
    key_map.insert(KeyCode::ArrowUp, NesKey::Up);
    key_map.insert(KeyCode::ArrowDown, NesKey::Down);
    key_map.insert(KeyCode::ArrowLeft, NesKey::Left);
    key_map.insert(KeyCode::ArrowRight, NesKey::Right);
    key_map
}

fn build_keymap_c2() -> KeyMap {
    // TODO read from config file
    use core::NesKey;

    let mut key_map = HashMap::new();
    key_map.insert(KeyCode::KeyC, NesKey::A);
    key_map.insert(KeyCode::KeyV, NesKey::B);
    key_map.insert(KeyCode::KeyB, NesKey::Select);
    key_map.insert(KeyCode::KeyN, NesKey::Start);
    key_map.insert(KeyCode::KeyW, NesKey::Up);
    key_map.insert(KeyCode::KeyS, NesKey::Down);
    key_map.insert(KeyCode::KeyA, NesKey::Left);
    key_map.insert(KeyCode::KeyD, NesKey::Right);
    key_map
}

fn handle_inputs(
    input: &WinitInputHelper,
    ctrl: &mut core::NesController,
    key_map: &HashMap<KeyCode, core::NesKey>,
) {
    for k in key_map.keys() {
        if input.key_pressed(*k) {
            ctrl.pressed(*key_map.get(k).unwrap());
        }
        if input.key_released(*k) {
            ctrl.released(*key_map.get(k).unwrap());
        }
    }
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowExtWebSys;

#[cfg(target_arch = "wasm32")]
fn attach_canvas(window: Arc<Window>) {
    let web_window = web_sys::window().expect("no global `window` exists");
    let document = web_window
        .document()
        .expect("should have a document on window");
    let body = document.body().expect("HTML body missing");

    let canvas = window.canvas().expect("Canvas generation failed");
    canvas.set_width(WIDTH);
    canvas.set_height(HEIGHT);
    body.append_child(&canvas).expect("Canvas append failed");
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start_wasm(buf: &[u8], ext: &str) -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).ok();

    let loader = core::decode_loader(ext);

    let (_, mapper) = loader
        .load_rom_struct(buf)
        .map_err(|err| JsValue::from_str(&err.to_string()))?;

    let event_loop = EventLoop::new().map_err(|err| JsValue::from_str(&err.to_string()))?;
    let app = App::new(mapper, false, false, Some(attach_canvas));

    event_loop.spawn_app(app);
    Ok(())
}
