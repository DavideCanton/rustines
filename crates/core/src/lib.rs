// define alias name to allow using memento proc macro inside the crate
extern crate self as rustines_core;

pub mod arch;
pub mod audio;
pub mod loaders;
pub mod renderer;
pub mod utils;

pub use arch::{
    apu::Apu,
    bus::{Bus, Controller2},
    controller::{NesController, NesKey},
    cpu::Cpu,
    debug_utils,
    mappers::mapper::{Mapper, MapperBox},
    memento::{
        MementoError, MementoReadAndFill, MementoReadable, MementoReader, MementoResult,
        MementoWriteable, MementoWriter,
    },
    ppu::Ppu,
    rom_structs::{INesHeader, MirroringType, NesRom},
};
pub use audio::{AudioOutput, AudioOutputBox, NoAudio};
pub use loaders::{loader::Loader, loaders_factory::decode_loader};
pub use renderer::{MessageLocation, NoopRenderer, Renderer, RendererBox};
