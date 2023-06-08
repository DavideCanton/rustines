use std::{fs::File, io, io::Read};

use log::debug;
use rustines_macro::Named;

use crate::loaders::loader::Loader;
use crate::utils::named::Named;

#[derive(Named)]
pub struct FlatLoader;

impl Loader for FlatLoader {
    fn load_rom(&self, f: &mut File) -> io::Result<Vec<u8>> {
        let mut buf: Vec<u8> = vec![];

        let read = f.read_to_end(&mut buf)?;
        debug!("Read {} bytes", read);

        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use crate::loaders::loader::Loader;

    use super::FlatLoader;
    use rand::{rngs::StdRng, RngCore, SeedableRng};
    use std::env::temp_dir;
    use std::fs::{write, File};

    #[test]
    fn test_load() {
        let mut rng = get_rng();
        let mut buf = vec![0u8; 1 << 16];
        rng.fill_bytes(&mut buf);

        let dir = temp_dir();
        let path = dir.join("test.rom");
        write(&path, &buf).unwrap();

        let loader = FlatLoader;
        let mut file = File::open(&path).unwrap();
        let data = loader.load_rom(&mut file).unwrap();

        assert_eq!(data.len(), buf.len());
        assert_eq!(data, buf);
    }

    fn get_rng() -> StdRng {
        let seed = [0u8; 32];
        SeedableRng::from_seed(seed)
    }
}
