use std::fs::File;
use std;

// TODO define return type
pub trait Loader {
    fn load_rom(&self, file: &mut File) -> std::io::Result<Vec<u8>>;
    fn name(&self) -> String;
}

