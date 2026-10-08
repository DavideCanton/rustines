use std::io;

use rustines_macro::Named;

use crate::loaders::loader::Loader;
use crate::utils::named::Named;

#[derive(Named)]
pub struct FlatLoader;

impl Loader for FlatLoader {
    fn load_rom(&self, buf: &[u8]) -> io::Result<Vec<u8>> {
        Ok(buf.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use crate::loaders::loader::Loader;

    use super::FlatLoader;
    use rand::{Rng, SeedableRng, rngs::StdRng};

    #[test]
    fn test_load() {
        let mut rng = get_rng();
        let mut buf = vec![0u8; 1 << 16];
        rng.fill_bytes(&mut buf);

        let loader = FlatLoader;
        let data = loader.load_rom(&buf).unwrap();

        assert_eq!(data.len(), buf.len());
        assert_eq!(data, buf);
    }

    fn get_rng() -> StdRng {
        let seed = [0u8; 32];
        SeedableRng::from_seed(seed)
    }
}
