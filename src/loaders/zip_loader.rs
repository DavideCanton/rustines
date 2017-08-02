extern crate zip;

use loaders::loaders::Loader;
use std::io;
use std::fs::File;
use std::io::Read;

pub struct ZipLoader {}

impl ZipLoader {
    pub fn new() -> Self {
        ZipLoader { }
    }
}

impl Loader for ZipLoader {
    fn load_rom(&self, f: &mut File) -> io::Result<Vec<u8>> {
        let mut archive = zip::ZipArchive::new(f).unwrap();

        let mut rom_file = archive.by_index(0).unwrap();

        info!("ZipLoader: read file \"{}\"", rom_file.name());

        let mut buf = vec![];

        rom_file.read_to_end(&mut buf)?;

        Ok(buf)
    }

    fn name(&self) -> String {
        String::from("ZipLoader")
    }
}