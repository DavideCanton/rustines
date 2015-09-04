mod arch;
use arch::memory::Memory;
use arch::cpu::CPU;

use std::rc::Rc;
use std::cell::RefCell;

pub fn main()
{
    let mem = Rc::new(RefCell::new(Memory::new()));

    let mut cpu = CPU::new(mem.clone());
    cpu.execute();
}
