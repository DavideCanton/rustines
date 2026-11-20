use std::{collections::HashMap, fs::File, hash::Hash};

use gilrs::Button;
use log::info;
use rustines_core as core;
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct};
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode};
use winit_input_helper::WinitInputHelper;

use crate::{
    AppState, Platform,
    save_game::{load_save_state, save_state},
    types::{GamepadMap, KeyMap},
};

pub(crate) fn handle_special_keys(
    app_state: &mut AppState,
    event_loop: &ActiveEventLoop,
    platform: &impl Platform,
) {
    let input = &app_state.main_window_helper;

    // keep this enabled even when debug keys are disabled
    app_state.speedup.update(input.key_held(KeyCode::Backquote));

    if app_state.enable_debug_keys {
        handle_debug_keys(app_state, event_loop);
    }

    let input = &app_state.main_window_helper;

    if input.key_pressed(KeyCode::F5) {
        save_state(
            &app_state.nes_rom,
            &app_state.bus,
            &app_state.cpu,
            &app_state.proxy,
            platform,
        );
    }
    if input.key_pressed(KeyCode::F9) {
        load_save_state(&app_state.nes_rom, &app_state.proxy, platform);
    }

    if input.held_shift() {
        let bus = &mut app_state.bus;
        let cpu = &mut app_state.cpu;

        let mixer = bus.apu_mut().mixer_mut();

        if input.key_pressed(KeyCode::Digit1) {
            mixer.pulse1().toggle();
        }
        if input.key_pressed(KeyCode::Digit2) {
            mixer.pulse2().toggle();
        }
        if input.key_pressed(KeyCode::Digit3) {
            mixer.triangle().toggle();
        }
        if input.key_pressed(KeyCode::Digit4) {
            mixer.noise().toggle();
        }
        if input.key_pressed(KeyCode::Digit5) {
            mixer.dmc().toggle();
        }
        if input.key_pressed(KeyCode::Digit0) {
            mixer.enable_all();
        }

        if input.key_pressed(KeyCode::Digit6) {
            bus.ppu_mut().toggle_sprite_render();
        }
        if input.key_pressed(KeyCode::Digit7) {
            bus.ppu_mut().toggle_background_render();
        }

        if input.key_pressed(KeyCode::KeyR) {
            bus.reset();
            cpu.reset();
        }
    }
}

fn handle_debug_keys(app_state: &mut AppState, event_loop: &ActiveEventLoop) {
    let input = &app_state.main_window_helper;

    if input.held_shift() {
        if input.key_pressed(KeyCode::KeyX) {
            if app_state.pause {
                app_state.resume();
            } else {
                app_state.pause();
            }
        }

        let input = &app_state.main_window_helper;

        if input.key_pressed(KeyCode::KeyD) {
            core::debug_utils::debug_dump_nametables(&app_state.bus);
            core::debug_utils::debug_dump_attribute_tables(&app_state.bus);
        }

        if input.key_pressed(KeyCode::KeyP) {
            core::debug_utils::debug_dump_palette(&app_state.bus);
        }

        if input.key_pressed(KeyCode::KeyO) {
            core::debug_utils::debug_dump_oam(app_state.bus.ppu());
        }

        if input.key_pressed(KeyCode::KeyQ) {
            core::debug_utils::debug_dump_state(&app_state.bus, &app_state.cpu);
        }

        if input.key_pressed(KeyCode::F12) {
            app_state.auto_ab.toggle();
            if app_state.auto_ab.active {
                info!("Auto-A/B mode enabled");
            } else {
                info!("Auto-A/B mode disabled");
            }
        }

        if input.key_pressed(KeyCode::KeyF) {
            let mut file = File::create("frame.ppm").expect("cannot create file");
            app_state
                .bus
                .ppu()
                .renderer()
                .dump_frame(&mut file)
                .expect("cannot dump frame");
        }

        if input.key_pressed(KeyCode::KeyT) {
            let log_point = app_state.log_point;
            println!("LOG_POINT {}", log_point);
            info!("LOG_POINT {}", log_point);

            app_state.log_point += 1;

            app_state.cpu.enable_tracing(true);
            app_state.bus.enable_tracing(true);
            app_state.bus.ppu_mut().enable_tracing(true);
        }

        if input.key_pressed(KeyCode::KeyS) {
            app_state
                .pattern_window
                .show(app_state.bus.mapper_ref(), event_loop, 4);
        }
    }
}

pub(crate) fn handle_input_keys(
    input: &WinitInputHelper,
    ctrl: &mut core::NesController,
    key_map: &KeyMap,
) {
    for (k, v) in key_map.iter() {
        if input.key_pressed(*k) {
            ctrl.pressed(*v);
        }
        if input.key_released(*k) {
            ctrl.released(*v);
        }
    }
}

#[derive(Eq, PartialEq)]
pub struct InputMap {
    key_map_1: KeyMap,
    key_map_2: KeyMap,
    gamepad_map_1: GamepadMap,
}

impl InputMap {
    pub fn default_map() -> Self {
        Self {
            key_map_1: build_default_key_map_1(),
            key_map_2: build_default_key_map_2(),
            gamepad_map_1: build_default_gamepad_map_1(),
        }
    }

    pub fn gamepad_map_1(&self) -> &GamepadMap {
        &self.gamepad_map_1
    }

    pub fn key_map_1(&self) -> &KeyMap {
        &self.key_map_1
    }

    pub fn key_map_2(&self) -> &KeyMap {
        &self.key_map_2
    }
}

/// Helper struct for deserializing InputMap with reversed key-value pairs
#[derive(Deserialize)]
struct InputMapReversed {
    key_map_1: HashMap<core::NesKey, Vec<KeyCode>>,
    key_map_2: HashMap<core::NesKey, Vec<KeyCode>>,
    gamepad_map_1: HashMap<core::NesKey, Vec<Button>>,
}

fn reverse_and_group_map<K: Eq + Hash + Copy>(
    input: &HashMap<K, core::NesKey>,
) -> HashMap<core::NesKey, Vec<K>> {
    let mut reversed: HashMap<core::NesKey, Vec<K>> = HashMap::new();
    for (key, nes_key) in input {
        reversed.entry(*nes_key).or_default().push(*key);
    }
    reversed
}

fn reverse_and_flatten_map<K: Eq + Hash>(
    reversed: HashMap<core::NesKey, Vec<K>>,
) -> HashMap<K, core::NesKey> {
    reversed
        .into_iter()
        .flat_map(|(nes_key, keys)| keys.into_iter().map(move |key| (key, nes_key)))
        .collect()
}

impl Serialize for InputMap {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_struct("InputMap", 3)?;

        map.serialize_field("key_map_1", &reverse_and_group_map(&self.key_map_1))?;
        map.serialize_field("key_map_2", &reverse_and_group_map(&self.key_map_2))?;
        map.serialize_field("gamepad_map_1", &reverse_and_group_map(&self.gamepad_map_1))?;

        map.end()
    }
}

impl<'de> Deserialize<'de> for InputMap {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let reversed = InputMapReversed::deserialize(deserializer)?;

        Ok(InputMap {
            key_map_1: reverse_and_flatten_map(reversed.key_map_1),
            key_map_2: reverse_and_flatten_map(reversed.key_map_2),
            gamepad_map_1: reverse_and_flatten_map(reversed.gamepad_map_1),
        })
    }
}

fn build_default_key_map_1() -> KeyMap {
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

fn build_default_key_map_2() -> KeyMap {
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

fn build_default_gamepad_map_1() -> GamepadMap {
    use core::NesKey;

    let mut key_map = HashMap::new();
    key_map.insert(Button::East, NesKey::A);
    key_map.insert(Button::West, NesKey::A);
    key_map.insert(Button::North, NesKey::B);
    key_map.insert(Button::South, NesKey::B);
    key_map.insert(Button::Select, NesKey::Select);
    key_map.insert(Button::Start, NesKey::Start);
    key_map.insert(Button::DPadUp, NesKey::Up);
    key_map.insert(Button::DPadDown, NesKey::Down);
    key_map.insert(Button::DPadLeft, NesKey::Left);
    key_map.insert(Button::DPadRight, NesKey::Right);
    key_map
}

#[cfg(test)]
mod tests {
    use super::InputMap;

    #[test]
    fn test_roundtrip() {
        let original = InputMap::default_map();
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: InputMap = serde_json::from_str(&json).unwrap();

        assert_eq!(original.key_map_1(), deserialized.key_map_1());
        assert_eq!(original.key_map_2(), deserialized.key_map_2());
        assert_eq!(original.gamepad_map_1(), deserialized.gamepad_map_1());
    }
}
