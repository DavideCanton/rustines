mod arch;
mod utils;
use arch::memory::Memory;
use arch::cpu::CPU;

use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

fn load_ram(path: PathBuf) -> std::io::Result<Vec<u8>>
{
    let mut f = try!(File::open(path));
    let mut buf = vec![];
    try!(f.read_to_end(&mut buf));
    println!("Len: {}", buf.len());
    Ok(buf)
}

pub fn main()
{
    let mem = load_ram(PathBuf::from("D:\\prova.nes")).unwrap();
    let mem = Memory::from_array(mem).expect("Invalid data!");
    let mut cpu = CPU::new(Rc::new(RefCell::new(mem)));

    cpu.execute();

    {
        let mem = cpu.memory.borrow();
        println!("{}", mem.fetch(0x100));
        println!("{}", mem.fetch(0x101));
        println!("{}", mem.fetch(0x102));
        println!("{}", mem.fetch(0x103));
    }

    println!("End!");
}
