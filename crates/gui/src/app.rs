use crate::{
    Platform,
    states::{InitializingState, State},
    types::AppEventLoopProxy,
};

use rustines_core::{self as core};
use std::path::PathBuf;

use winit::{
    application::ApplicationHandler,
    event::{StartCause, WindowEvent},
    event_loop::ActiveEventLoop,
    window::WindowId,
};

#[derive(PartialEq, Eq, Debug)]
pub enum CustomEvents {
    Pause,
    Resume,
    SaveGameRead(Option<Vec<u8>>),
    SaveGameWritten(PathBuf, bool),
    SaveStateRead(Option<Vec<u8>>),
    SaveStateWritten(PathBuf, bool),
    ConfigRead(Option<String>),
    ConfigWritten(PathBuf, bool),
}

pub const CONFIG_PATH: &str = "config.json";

pub struct App {
    state: Option<Box<dyn State>>,
}

impl App {
    #[allow(clippy::too_many_arguments)]
    pub fn new<P: Platform + 'static>(
        rom_name: String,
        nes_rom: core::NesRom,
        mapper: core::MapperBox,
        trace_boot: bool,
        zapper: bool,
        width: u32,
        platform: P,
        enable_debug_keys: bool,
        proxy: AppEventLoopProxy,
    ) -> Self {
        App {
            state: Some(Box::new(InitializingState::new(
                rom_name,
                nes_rom,
                mapper,
                trace_boot,
                zapper,
                width,
                Box::new(platform),
                enable_debug_keys,
                proxy,
            ))),
        }
    }

    fn take_and_run(&mut self, transition: impl FnOnce(Box<dyn State>) -> Box<dyn State>) {
        self.state = Some(transition(self.state.take().expect("No current state")))
    }
}

impl ApplicationHandler<CustomEvents> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.take_and_run(|s| s.resumed(event_loop));
    }

    fn new_events(&mut self, event_loop: &ActiveEventLoop, _: StartCause) {
        self.take_and_run(|s| s.new_events(event_loop));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        self.take_and_run(|s| s.window_event(event_loop, window_id, event));
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: CustomEvents) {
        self.take_and_run(|s| s.user_event(event_loop, event));
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.take_and_run(|s| s.about_to_wait(event_loop));
    }
}
