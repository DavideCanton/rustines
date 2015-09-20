mod arch;
mod utils;
use arch::memory::Memory;
use arch::cpu::CPU;

use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

fn load_ram(cpu: &mut CPU, path: PathBuf) -> std::io::Result<()>
{
    let mut mem = cpu.memory.borrow_mut();
    let mut f = try!(File::open(path));
    let mut buf = vec![0u8; 1 << 16];
    try!(f.read_to_end(&mut buf));
    for (i, el) in buf.into_iter().enumerate()
    {
        mem.store(i as u16, el);
    }
    Ok(())
}

pub fn main()
{
    let mem = Rc::new(RefCell::new(Memory::new()));

    let mut cpu = CPU::new(mem.clone());

    if let Err(e) = load_ram(&mut cpu, PathBuf::from("D:\\prova.nes"))
    {
        println!("{:?}", e);
        return;
    }

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
