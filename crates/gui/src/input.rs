use std::{collections::HashMap, hash::Hash};

use gilrs::Button;
use rustines_core as core;
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct};
use winit::keyboard::KeyCode;

use crate::types::{GamepadMap, KeyMap};

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
