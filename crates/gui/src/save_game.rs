use std::path::Path;

use log::info;
use rustines_core::{self as core, arch::memento::MementoWriteable};

use crate::{
    AppEventLoopProxy, CustomEvents, Platform,
    constants::{SAVE_COMPRESSION_LEVEL, SAVE_GAME_EXTENSION, SAVE_STATE_EXTENSION},
};

pub fn load_save_state(
    nes_rom: &core::NesRom,
    proxy: &AppEventLoopProxy,
    platform: &impl Platform,
) {
    if let Some(path) = nes_rom.file_path.as_ref() {
        platform.start_read_file(
            path.with_extension(SAVE_STATE_EXTENSION),
            proxy,
            Box::new(CustomEvents::LoadedState),
        );
    }
}

pub fn load_save_game(
    nes_rom: &core::NesRom,
    mapper: &dyn core::Mapper,
    platform: &impl Platform,
    proxy: &AppEventLoopProxy,
) {
    if let Some((path, _)) = supports_save(nes_rom, mapper) {
        platform.start_read_file(
            path.with_extension(SAVE_GAME_EXTENSION),
            proxy,
            Box::new(CustomEvents::LoadedPrgRam),
        );
    }
}

pub fn save_state(
    nes_rom: &core::NesRom,
    bus: &core::Bus,
    cpu: &core::Cpu,
    proxy: &AppEventLoopProxy,
    platform: &impl Platform,
) {
    if let Some(path) = nes_rom.file_path.as_ref() {
        let mut writer = vec![];

        bus.write_memento(&mut writer)
            .expect("Failed to write saved state");
        cpu.write_memento(&mut writer)
            .expect("Failed to write saved state");

        match zstd::encode_all(writer.as_slice(), SAVE_COMPRESSION_LEVEL) {
            Ok(compressed) => platform.start_write_file(
                path.with_extension(SAVE_STATE_EXTENSION),
                proxy,
                &compressed,
                Box::new(CustomEvents::SavedPrgRam),
            ),
            Err(e) => info!("Compress save state failed: {e}"),
        }
    }
}

fn supports_save<'n, 'm>(
    nes_rom: &'n core::NesRom,
    mapper: &'m dyn core::Mapper,
) -> Option<(&'n Path, &'m [u8])> {
    if !nes_rom.header.battery_backed_prg_ram() {
        return None;
    }

    let path = nes_rom.file_path.as_ref()?;
    let memory = mapper.get_prg_ram()?;

    Some((path, memory))
}

pub fn save_game(
    nes_rom: &core::NesRom,
    mapper: &dyn core::Mapper,
    platform: &impl Platform,
    proxy: &AppEventLoopProxy,
) {
    if let Some((path, memory)) = supports_save(nes_rom, mapper) {
        info!("Writing save file");

        match zstd::encode_all(memory, SAVE_COMPRESSION_LEVEL) {
            Ok(compressed) => platform.start_write_file(
                path.with_extension(SAVE_GAME_EXTENSION),
                proxy,
                &compressed,
                Box::new(CustomEvents::SavedState),
            ),
            Err(e) => info!("Compress save state failed: {e}"),
        }
    }
}
