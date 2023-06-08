use std::{fs::File, io, io::Read};

use log::info;
use rustines_macro::Named;
use zip::ZipArchive;

use crate::loaders::loader::Loader;
use crate::utils::named::Named;

#[derive(Named)]
pub struct ZipLoader;

impl Loader for ZipLoader {
    fn load_rom(&self, f: &mut File) -> io::Result<Vec<u8>> {
        let mut archive = ZipArchive::new(f)?;

        let mut rom_file = archive.by_index(0)?;

        info!("read file \"{}\"", rom_file.name());

        let mut buf: Vec<u8> = Vec::with_capacity(rom_file.size() as usize);

        rom_file.read_to_end(&mut buf)?;

        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::ZipLoader;
    use crate::loaders::loader::Loader;
    use rand::{rngs::StdRng, RngCore, SeedableRng};
    use std::{
        env::temp_dir,
        fs::{File, OpenOptions},
        io::Write,
    };
    use zip::{write::FileOptions, ZipWriter};

    #[test]
    fn test_load() {
        let mut rng = get_rng();
        let mut buf = vec![0u8; 1 << 16];
        rng.fill_bytes(&mut buf);

        let dir = temp_dir();
        let zip_path = dir.join("test.zip");
        let archive = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&zip_path)
            .unwrap();

        let mut zip = ZipWriter::new(archive);
        zip.start_file("test.rom", FileOptions::default()).unwrap();
        zip.write_all(&buf).unwrap();
        zip.finish().unwrap();

        let loader = ZipLoader;
        let mut file = File::open(&zip_path).unwrap();
        let data = loader.load_rom(&mut file).unwrap();

        assert_eq!(data.len(), buf.len());
        assert_eq!(data, buf);
    }

    fn get_rng() -> StdRng {
        let seed = [0u8; 32];
        SeedableRng::from_seed(seed)
    }
}
