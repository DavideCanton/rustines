use std::collections::HashMap;

use gilrs::Button;
use winit::{event_loop::EventLoopProxy, keyboard::KeyCode};

use crate::CustomEvents;

pub type KeyMap = HashMap<KeyCode, rustines_core::NesKey>;
pub type GamepadMap = HashMap<Button, rustines_core::NesKey>;
pub type AppEventLoopProxy = EventLoopProxy<CustomEvents>;
