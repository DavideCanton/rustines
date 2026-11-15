use std::{collections::HashMap, fs::File, hash::Hash};

use gilrs::Button;
use log::info;
use rustines_core::{self as core, MementoWriteable};
use winit::{
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    platform::scancode::PhysicalKeyExtScancode,
};
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

    fn _write_key_map<K>(
        map: &HashMap<K, core::NesKey>,
        key_writer: impl Fn(&K, &mut core::MementoWriter),
        writer: &mut core::MementoWriter,
    ) {
        map.len().write_memento(writer);
        for (k, v) in map.iter() {
            key_writer(k, writer);
            let v_num = *v as u16;
            v_num.write_memento(writer);
        }
    }

    fn _read_key_map<K: Eq + Hash>(
        key_reader: impl Fn(&mut core::MementoReader) -> core::MementoResult<K>,
        reader: &mut core::MementoReader,
    ) -> core::MementoResult<HashMap<K, core::NesKey>> {
        let len: usize = reader.read_memento()?;
        let mut map = HashMap::new();

        for _ in 0..len {
            let key = key_reader(reader)?;
            let value: u16 = reader.read_memento()?;

            // TODO improve
            let nes_key: core::NesKey = unsafe { std::mem::transmute(value) };

            map.insert(key, nes_key);
        }

        Ok(map)
    }
}

impl core::MementoReadable for InputMap {
    fn read_memento(reader: &mut core::MementoReader) -> core::MementoResult<Self>
    where
        Self: Sized,
    {
        let keycode_reader = |r: &mut core::MementoReader| -> core::MementoResult<KeyCode> {
            match KeyCode::from_scancode(r.read_memento::<u32>()?) {
                PhysicalKey::Code(code) => Ok(code),
                // TODO improve
                PhysicalKey::Unidentified(_) => Err(core::MementoError::InvalidValue(42, 42)),
            }
        };

        let key_map_1 = Self::_read_key_map(keycode_reader, reader)?;
        let key_map_2 = Self::_read_key_map(keycode_reader, reader)?;

        let gamepad_map_1 = Self::_read_key_map(
            |r| -> core::MementoResult<Button> {
                let code: u16 = r.read_memento()?;

                // TODO improve
                let btn: Button = unsafe { std::mem::transmute(code) };

                Ok(btn)
            },
            reader,
        )?;

        Ok(Self {
            key_map_1,
            key_map_2,
            gamepad_map_1,
        })
    }
}

impl core::MementoWriteable for InputMap {
    fn write_memento(&self, writer: &mut core::MementoWriter) {
        let keycode_writer = |k: &KeyCode, w: &mut core::MementoWriter| {
            k.to_scancode().unwrap().write_memento(w);
        };

        Self::_write_key_map(&self.key_map_1, keycode_writer, writer);
        Self::_write_key_map(&self.key_map_2, keycode_writer, writer);

        Self::_write_key_map(
            &self.gamepad_map_1,
            |btn, w| {
                let v = *btn as u16;
                v.write_memento(w);
            },
            writer,
        );
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
