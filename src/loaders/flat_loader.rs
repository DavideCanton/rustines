use crate::loaders::loader::Loader;
use std::{fs::File, io, io::Read, any::Any};

pub struct FlatLoader;

impl Loader for FlatLoader {
    fn load_rom(&self, f: &mut File) -> io::Result<Vec<u8>> {
        let mut buf: Vec<u8> = vec![];

        f.read_to_end(&mut buf)?;

        Ok(buf)
    }

    impl_loader!("FlatLoader");
}
