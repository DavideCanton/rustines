use std::{fs::File, io, io::Read};

use log::info;
use rustines_macro::Named;

use crate::loaders::loader::Loader;
use crate::utils::named::Named;

#[derive(Named)]
pub struct ZipLoader;

impl Loader for ZipLoader {
    fn load_rom(&self, f: &mut File) -> io::Result<Vec<u8>> {
        let mut archive = zip::ZipArchive::new(f)?;

        let mut rom_file = archive.by_index(0)?;

        info!("read file \"{}\"", rom_file.name());

        let mut buf: Vec<u8> = Vec::with_capacity(rom_file.size() as usize);

        rom_file.read_to_end(&mut buf)?;

        Ok(buf)
    }
}
