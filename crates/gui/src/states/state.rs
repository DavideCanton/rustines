use winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::WindowId};

use crate::{CustomEvents, Platform};

pub type PlatformBox = Box<dyn Platform>;

pub trait State {
    fn resumed(self: Box<Self>, event_loop: &ActiveEventLoop) -> Box<dyn State>;
    fn new_events(self: Box<Self>, event_loop: &ActiveEventLoop) -> Box<dyn State>;
    fn window_event(
        self: Box<Self>,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) -> Box<dyn State>;
    fn user_event(
        self: Box<Self>,
        event_loop: &ActiveEventLoop,
        event: CustomEvents,
    ) -> Box<dyn State>;
    fn about_to_wait(self: Box<Self>, event_loop: &ActiveEventLoop) -> Box<dyn State>;
}
