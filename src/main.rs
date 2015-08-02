mod arch;
use arch::memory::Memory;
use arch::cpu::CPU;

pub fn main()
{
    let mut mem = Memory::new();
    let mut cpu = CPU::new(&mut mem);
    cpu.execute();
}
