mod arch;
mod utils;
use arch::memory::Memory;
use arch::cpu::CPU;

use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

fn load_ram(path: PathBuf) -> std::io::Result<Vec<u8>> {
    let mut f = try!(File::open(path));
    let mut buf = vec![];
    try!(f.read_to_end(&mut buf));
    println!("Len: {}", buf.len());
    Ok(buf)
}

pub fn main() {
    let file_path = "D:\\prova.nes"; // example path
    let mem = load_ram(PathBuf::from(file_path)).unwrap();
    let mem = Memory::from_array(mem).expect("Invalid data!");
    let mut cpu = CPU::new(mem);

    cpu.execute();

    println!("{}", cpu.memory.fetch(0x100));
    println!("{}", cpu.memory.fetch(0x101));
    println!("{}", cpu.memory.fetch(0x102));
    println!("{}", cpu.memory.fetch(0x103));

    println!("End!");
}
