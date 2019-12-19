use crate::loaders::loader::Loader;
use log::info;
use std::{any::Any, fs::File, io, io::Read};

pub struct ZipLoader;

impl Loader for ZipLoader {
    fn load_rom(&self, f: &mut File) -> io::Result<Vec<u8>> {
        let mut archive = zip::ZipArchive::new(f)?;

        let mut rom_file = archive.by_index(0)?;

        info!("ZipLoader: read file \"{}\"", rom_file.name());

        let mut buf: Vec<u8> = vec![];

        rom_file.read_to_end(&mut buf)?;

        Ok(buf)
    }

    impl_loader!("ZipLoader");
}
