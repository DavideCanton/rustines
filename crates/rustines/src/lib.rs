mod args;
mod pattern_window;
mod renderer;
mod utils;

use crate::{pattern_window::PatternTableWindow, renderer::PixelsRenderer};
use log::info;
use pixels::{PixelsBuilder, ScalingMode, SurfaceTexture};

use rustines_core::{self as core, arch::bus::Controller2};
use rustines_gui_utils::FpsCounter;
#[cfg(target_arch = "wasm32")]
use std::{cell::RefCell, rc::Rc};
use std::{collections::HashMap, fs::File, sync::Arc};

use web_time::{Duration, Instant};

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{ElementState, MouseButton, StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow},
    keyboard::KeyCode,
    window::{Window, WindowAttributes, WindowId},
};
use winit_input_helper::WinitInputHelper;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowExtWebSys;
#[cfg(target_arch = "wasm32")]
use winit::{event_loop::EventLoop, platform::web::EventLoopExtWebSys};

const INNER_W: u32 = 256;
const INNER_H: u32 = 240;
const WIDTH: u32 = 1024;
const RATIO: f64 = INNER_H as f64 / INNER_W as f64;
const HEIGHT: u32 = (WIDTH as f64 * RATIO) as u32;
const TARGET_FPS: f64 = 60.0;

type KeyMap = HashMap<KeyCode, rustines_core::NesKey>;

#[derive(Default)]
struct SpeedupState {
    active: bool,
    changed: bool,
}

impl SpeedupState {
    fn update(&mut self, active: bool) {
        self.changed = active != self.active;
        self.active = active;
    }
}

struct AppState {
    rom_name: String,

    bus: core::Bus,
    cpu: core::Cpu,

    render_counter: FpsCounter,
    update_counter: FpsCounter,
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
    next_frame_time: Instant,
    frame_duration: Duration,
    speedup: SpeedupState,
}

struct AppStateInfo {
    mapper: core::MapperBox,
    rom_name: String,
}

pub struct App {
    app_state: Option<AppState>,
    info: Option<AppStateInfo>,
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
        rom_name: String,
        mapper: core::MapperBox,
        trace_boot: bool,
        zapper: bool,
        init_window_hook: Option<fn(Arc<Window>)>,
    ) -> Self {
        App {
            info: Some(AppStateInfo { mapper, rom_name }),
            app_state: None,
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

        let info = self.info.take().unwrap();

        let mut bus = core::Bus::new(info.mapper, ppu, apu, ctrl2);
        let mut cpu = core::Cpu::new();

        if self.trace_boot {
            cpu.enable_tracing(true);
            bus.enable_tracing(true);
            bus.ppu_mut().enable_tracing(true);
        }

        AppState {
            rom_name: info.rom_name,
            bus,
            render_counter: FpsCounter::new(),
            update_counter: FpsCounter::new(),
            cpu,
            key_map1: build_keymap_c1(),
            key_map2: build_keymap_c2(),
            log_point: 1,
            pattern_window: PatternTableWindow::new(),
            main_window,
            pause: false,
            main_window_helper: WinitInputHelper::new(),
            cursor_position: None,
            zapper_trigger_pressed: false,
            zapper_needs_update: false,
            next_frame_time: Instant::now(),
            frame_duration: Duration::from_secs_f64(1.0 / TARGET_FPS),
            speedup: SpeedupState::default(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.app_state.is_some() {
            return;
        }

        let size = PhysicalSize::new(WIDTH as f64, HEIGHT as f64);
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

        if let Some(init_hook) = self.init_window_hook {
            (init_hook)(Arc::clone(&main_window));
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
        _event_loop: &ActiveEventLoop,
        window_id: WindowId,
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
                    app_state.bus.ppu_mut().renderer_mut().draw();

                    app_state.render_counter.update();

                    let r_fps = format_fps(app_state.render_counter.current_fps(), "FPS");
                    let u_fps =
                        format_fps(app_state.update_counter.current_fps(), "Simulation FPS");

                    let title = format!("{} | {r_fps} | {u_fps}", app_state.rom_name);

                    #[cfg(target_arch = "wasm32")]
                    {
                        let window = web_sys::window().unwrap();
                        let document = window.document().unwrap();
                        document
                            .get_element_by_id("status")
                            .unwrap()
                            .set_text_content(Some(&title));
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    app_state
                        .main_window
                        .set_title(&format!("Rustines | {title}"));
                }
            } else if app_state.pattern_window.owns_window_event(window_id) {
                app_state.pattern_window.window_event(&event);
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        #[cfg(target_arch = "wasm32")]
        {
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
            handle_inputs(app_state, event_loop);

            let now = Instant::now();
            let next_frame = now >= app_state.next_frame_time;

            if app_state.speedup.active || next_frame {
                update_logic(app_state);
            }
            if app_state.speedup.active {
                event_loop.set_control_flow(ControlFlow::Poll);
            } else if next_frame {
                if app_state.speedup.changed {
                    app_state.next_frame_time = now + app_state.frame_duration;
                } else {
                    app_state.next_frame_time += app_state.frame_duration;
                }

                event_loop.set_control_flow(ControlFlow::WaitUntil(app_state.next_frame_time));
            }
        }
    }
}

fn update_logic(app_state: &mut AppState) {
    if !app_state.pause {
        let frame_to_run = if app_state.speedup.active { 4 } else { 1 };
        for _ in 0..frame_to_run {
            app_state.update_counter.update();
            while !app_state.bus.ppu().frame_ready() {
                app_state.cpu.tick(&mut app_state.bus);
            }
            app_state.bus.ppu_mut().clear_frame_ready();
        }
    }

    app_state.main_window.request_redraw();
    app_state.pattern_window.render();
}

fn handle_inputs(app_state: &mut AppState, event_loop: &ActiveEventLoop) {
    let input = &mut app_state.main_window_helper;

    input.end_step();

    #[cfg(not(target_arch = "wasm32"))]
    let esc_pressed = input.key_pressed(KeyCode::Escape);
    #[cfg(target_arch = "wasm32")]
    let esc_pressed = false;

    if esc_pressed || input.close_requested() {
        event_loop.exit();
    } else {
        handle_debug_keys(app_state, event_loop);

        // reborrow
        let input = &app_state.main_window_helper;

        handle_input_keys(input, app_state.bus.controller1_mut(), &app_state.key_map1);
        if let Some(ctrl2) = app_state.bus.controller2_mut() {
            handle_input_keys(input, ctrl2, &app_state.key_map2);
        }

        if app_state.bus.zapper_mut().is_some() && app_state.zapper_needs_update {
            update_zapper_input(app_state);
        }
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

    app_state.speedup.update(input.key_held(KeyCode::Backquote));

    if input.held_shift() {
        if input.key_pressed(KeyCode::KeyD) {
            core::debug_utils::debug_dump_nametables(bus);
            core::debug_utils::debug_dump_attribute_tables(bus);
        }

        if input.key_pressed(KeyCode::KeyP) {
            core::debug_utils::debug_dump_palette(bus);
        }

        if input.key_pressed(KeyCode::KeyO) {
            core::debug_utils::debug_dump_oam(bus.ppu());
        }

        if input.key_pressed(KeyCode::KeyX) {
            app_state.pause = !app_state.pause;
        }

        if input.key_pressed(KeyCode::KeyQ) {
            core::debug_utils::debug_dump_state(bus, cpu);
            core::debug_utils::debug_dump_scrolling_state(bus);
        }

        if input.key_pressed(KeyCode::KeyF) {
            let mut file = File::create("frame.ppm").expect("cannot create file");
            bus.ppu()
                .renderer()
                .dump_frame(&mut file)
                .expect("cannot dump frame");
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
    #[cfg(target_arch = "wasm32")]
    let size = PhysicalSize::new(WIDTH, HEIGHT);
    #[cfg(not(target_arch = "wasm32"))]
    let size = window.inner_size();

    if size.width == 0 || size.height == 0 {
        return Err(format!(
            "Window surface has invalid size: {}x{}",
            size.width, size.height
        ));
    }

    let surface_texture = SurfaceTexture::new(size.width, size.height, window);

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

fn handle_input_keys(
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

fn format_fps(fps: f64, msg: &str) -> String {
    format!("{}: {:.1}", msg, fps)
}

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
    canvas.set_id("game_canvas");
    body.append_child(&canvas).expect("Canvas append failed");
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start_wasm(buf: &[u8], name: &str, ext: &str) -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).ok();

    let loader = core::decode_loader(ext);

    let (_, mapper) = loader
        .load_rom_struct(buf)
        .map_err(|err| JsValue::from_str(&err.to_string()))?;

    let event_loop = EventLoop::new().map_err(|err| JsValue::from_str(&err.to_string()))?;
    let app = App::new(name.to_string(), mapper, false, false, Some(attach_canvas));

    event_loop.spawn_app(app);
    Ok(())
}
