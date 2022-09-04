use std::{fs::File, io, io::Read};

use crate::impl_named;
use crate::loaders::loader::Loader;
use crate::utils::named::Named;

pub struct FlatLoader;

impl_named!(FlatLoader);

impl Loader for FlatLoader {
    fn load_rom(&self, f: &mut File) -> io::Result<Vec<u8>> {
        let mut buf: Vec<u8> = vec![];

        f.read_to_end(&mut buf)?;

        Ok(buf)
    }
}
