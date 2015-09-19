#[cfg(test)]
pub mod tests
{
    use arch::cpu::CPU;
    use arch::memory::Memory;
    use std::rc::Rc;
    use std::cell::RefCell;

    pub fn setup_tests() -> (CPU, Rc<RefCell<Memory>>)
    {
        let mem = Rc::new(RefCell::new(Memory::new()));
        let mut cpu = CPU::new(mem.clone());

        cpu.registers.PC = 0x100;

        (cpu, mem.clone())
    }
}
