use loaders::loaders::Loader;
use std::io;
use std::fs::File;
use std::io::Read;

pub struct FlatLoader {}

impl FlatLoader {
    pub fn new() -> Self {
        FlatLoader { }
    }
}

impl Loader for FlatLoader {
    fn load_rom(&self, f: &mut File) -> io::Result<Vec<u8>> {        
        let mut buf: Vec<u8> = vec![];

        f.read_to_end(&mut buf)?;
        
        Ok(buf)
    }

    fn name(&self) -> String {
        String::from("FlatLoader")
    }
}