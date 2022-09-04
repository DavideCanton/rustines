use std::{fs::File, io, io::Read};

use log::info;

use crate::impl_named;
use crate::loaders::loader::Loader;
use crate::utils::named::Named;

pub struct ZipLoader;

impl_named!(ZipLoader);

impl Loader for ZipLoader {
    fn load_rom(&self, f: &mut File) -> io::Result<Vec<u8>> {
        let mut archive = zip::ZipArchive::new(f)?;

        let mut rom_file = archive.by_index(0)?;

        info!("ZipLoader: read file \"{}\"", rom_file.name());

        let mut buf: Vec<u8> = vec![];

        rom_file.read_to_end(&mut buf)?;

        Ok(buf)
    }
}
