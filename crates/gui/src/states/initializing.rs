use std::{cell::RefCell, path::PathBuf, rc::Rc, sync::Arc};

use gilrs::Gilrs;
use log::{error, info, warn};
use rustines_core as core;
use winit::{
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId},
};
use winit_input_helper::WinitInputHelper;

use crate::{
    AppEventLoopProxy, CpalOutput, CustomEvents,
    app::CONFIG_PATH,
    app_structs::{AutoABState, SpeedupState},
    constants as C,
    fps_counter::FpsCounter,
    input::InputMap,
    pattern_window::PatternTableWindow,
    platform::{PendingInputMap, PendingRenderer},
    save_game::load_save_game,
    states::{RunningState, State, state::PlatformBox},
};
use web_time::{Duration, Instant};

pub struct InitializingState {
    rom_name: String,
    nes_rom: core::NesRom,
    mapper: core::MapperBox,
    trace_boot: bool,
    zapper: bool,
    enable_debug_keys: bool,
    proxy: AppEventLoopProxy,
    size: PhysicalSize<u32>,
    pending_window: Option<Arc<Window>>,
    pending_renderer: PendingRenderer,
    pending_input_map: PendingInputMap,
    platform: PlatformBox,
    gilrs: Option<Gilrs>,
}

impl InitializingState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        rom_name: String,
        nes_rom: core::NesRom,
        mapper: core::MapperBox,
        trace_boot: bool,
        zapper: bool,
        width: u32,
        platform: PlatformBox,
        enable_debug_keys: bool,
        proxy: AppEventLoopProxy,
    ) -> Self {
        Self {
            rom_name,
            nes_rom,
            mapper,
            trace_boot,
            zapper,
            platform,
            enable_debug_keys,
            proxy,
            size: C::width_to_size(width),
            pending_window: None,
            pending_renderer: Rc::new(RefCell::new(None)),
            pending_input_map: Rc::new(RefCell::new(None)),
            gilrs: Gilrs::new()
                .inspect_err(|e| info!("Cannot initialize gilrs: {e:?}"))
                .ok(),
        }
    }

    fn write_default_config(&mut self) -> InputMap {
        let default = InputMap::default_map();
        let serialized = serde_json::to_string(&default);
        match serialized {
            Ok(text) => {
                self.platform.start_write_text_file(
                    PathBuf::from(CONFIG_PATH),
                    &self.proxy,
                    &text,
                    Box::new(CustomEvents::ConfigWritten),
                );
            }
            Err(e) => warn!("Cannot write default config: {e}"),
        }
        default
    }

    fn init_running(
        self,
        renderer: core::RendererBox,
        input_map: InputMap,
        main_window: Arc<Window>,
    ) -> Box<dyn State> {
        let ppu = core::Ppu::new(renderer);

        let audio: core::AudioOutputBox = match CpalOutput::new() {
            Ok(mut cpal) => {
                cpal.play().expect("Can't play");
                Box::new(cpal)
            }
            Err(e) => {
                warn!("Cannot initialize audio: {}", e);
                Box::new(core::NoAudio)
            }
        };

        let apu = core::Apu::new(audio);

        let ctrl2 = if self.zapper {
            core::Controller2::zapper()
        } else {
            core::Controller2::nes_controller()
        };

        load_save_game(
            &self.nes_rom,
            self.mapper.as_ref(),
            self.platform.as_ref(),
            &self.proxy,
        );

        let mut bus = core::Bus::new(self.mapper, ppu, apu, ctrl2);
        let mut cpu = core::Cpu::new();

        if self.trace_boot {
            cpu.enable_tracing(true);
            bus.enable_tracing(true);
            bus.ppu_mut().enable_tracing(true);
        }

        let new_state = RunningState {
            rom_name: self.rom_name,
            nes_rom: self.nes_rom,
            bus,
            render_counter: FpsCounter::new(),
            update_counter: FpsCounter::new(),
            cpu,
            input_map,
            log_point: 1,
            pattern_window: PatternTableWindow::new(),
            main_window,
            pause: false,
            main_window_helper: WinitInputHelper::new(),
            cursor_position: None,
            zapper_trigger_pressed: false,
            zapper_needs_update: false,
            frame_duration: Duration::from_nanos(1_000_000_000 / C::TARGET_FPS),
            speedup: SpeedupState::default(),
            auto_ab: AutoABState::new(C::AUTO_AB_TICK_INTERVAL),
            enable_debug_keys: self.enable_debug_keys,
            accumulator: Duration::ZERO,
            last_frame_time: Instant::now(),
            last_save: 0,
            proxy: self.proxy.clone(),
            platform: self.platform,
            gilrs: self.gilrs,
        };

        Box::new(new_state)
    }
}

impl State for InitializingState {
    fn resumed(mut self: Box<Self>, event_loop: &ActiveEventLoop) -> Box<dyn State> {
        let main_window = match event_loop.create_window(
            WindowAttributes::default()
                .with_title("Rustines")
                .with_inner_size(self.size)
                .with_min_inner_size(self.size),
        ) {
            Ok(window) => Arc::new(window),
            Err(e) => {
                error!("Failed to create window: {}", e);
                event_loop.exit();
                return self;
            }
        };

        self.platform
            .init_window_hook(Arc::clone(&main_window), self.size);

        self.pending_window = Some(Arc::clone(&main_window));

        self.platform.init_pending_renderer(
            Arc::clone(&main_window),
            Rc::clone(&self.pending_renderer),
            self.size,
        );

        self.platform.start_read_text_file(
            PathBuf::from(CONFIG_PATH),
            &self.proxy,
            Box::new(CustomEvents::ConfigRead),
        );

        self
    }

    fn user_event(
        mut self: Box<Self>,
        _event_loop: &ActiveEventLoop,
        event: CustomEvents,
    ) -> Box<dyn State> {
        match event {
            CustomEvents::ConfigWritten(..) => {
                info!("Wrote config");
            }
            CustomEvents::ConfigRead(content) => match content {
                Some(c) => {
                    *self.pending_input_map.borrow_mut() = match serde_json::from_str(&c) {
                        Ok(input_map) => input_map,
                        Err(e) => {
                            warn!("Failed to parse config, defaulting: {e}");
                            // use default, don't overwrite the file, so the user can fix it
                            Some(InputMap::default_map())
                        }
                    }
                }
                None => {
                    let default = self.write_default_config();
                    *self.pending_input_map.borrow_mut() = Some(default);
                }
            },
            e => {
                warn!("Unexpected event during init: {:?}", e);
            }
        }
        self
    }

    fn about_to_wait(mut self: Box<Self>, _event_loop: &ActiveEventLoop) -> Box<dyn State> {
        if self.pending_renderer.borrow().is_some() && self.pending_input_map.borrow().is_some() {
            let renderer = self.pending_renderer.borrow_mut().take().unwrap();
            let input_map = self.pending_input_map.borrow_mut().take().unwrap();
            let window = self.pending_window.take().unwrap();

            self.init_running(renderer, input_map, window)
        } else {
            self
        }
    }

    fn new_events(self: Box<Self>, _: &ActiveEventLoop) -> Box<dyn State> {
        self
    }

    fn window_event(
        self: Box<Self>,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        _event: WindowEvent,
    ) -> Box<dyn State> {
        self
    }
}
