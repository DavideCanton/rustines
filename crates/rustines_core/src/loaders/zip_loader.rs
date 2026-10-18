use std::io::Cursor;
use std::{io, io::Read};

use log::info;
use rustines_macro::Named;
use zip::ZipArchive;

use crate::loaders::loader::Loader;
use crate::utils::named::Named;

#[derive(Named)]
pub struct ZipLoader;

impl Loader for ZipLoader {
    fn load_rom(&self, buf: &[u8]) -> io::Result<Vec<u8>> {
        let cursor = Cursor::new(buf);
        let mut archive = ZipArchive::new(cursor)?;

        let mut rom_file = archive.by_index(0)?;

        info!("Read file \"{}\"", rom_file.name());

        let mut buf: Vec<u8> = Vec::with_capacity(rom_file.size() as usize);

        rom_file.read_to_end(&mut buf)?;

        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::ZipLoader;
    use crate::loaders::loader::Loader;
    use rand::{Rng, SeedableRng, rngs::StdRng};
    use std::io::{Cursor, Write};
    use zip::{ZipWriter, write::SimpleFileOptions};

    #[test]
    fn test_load() {
        let mut rng = get_rng();
        let mut rom_data = vec![0u8; 1 << 16];
        rng.fill_bytes(&mut rom_data);

        let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
        zip.start_file("test.rom", SimpleFileOptions::default())
            .unwrap();
        zip.write_all(&rom_data).unwrap();
        let archive = zip.finish().unwrap().into_inner();

        let loader = ZipLoader;
        let data = loader.load_rom(&archive).unwrap();

        assert_eq!(data.len(), rom_data.len());
        assert_eq!(data, rom_data);
    }

    fn get_rng() -> StdRng {
        let seed = [0u8; 32];
        SeedableRng::from_seed(seed)
    }
}
