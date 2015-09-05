use std::rc::Rc;
use std::cell::RefCell;

use arch::cpu::CPU;
use arch::memory::Memory;

fn setup_tests() -> (CPU, Rc<RefCell<Memory>>)
{
    let mem = Rc::new(RefCell::new(Memory::new()));
    let mut cpu = CPU::new(mem.clone());

    cpu.registers.PC = 0x100;

    (cpu, mem.clone())
}

#[cfg(test)]
mod tests
{
    use super::setup_tests;

    #[test]
    pub fn test_lda_immediate()
    {
        let (mut cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC, 0xa9);
            mem.store(cpu.registers.PC + 1, 0xDE);
        }

        cpu.execute();

        let val = cpu.registers.A;
        assert_eq!(val, 0xDE);
    }

    #[test]
    pub fn test_lda_zeropage()
    {
        let (mut cpu, mem) = setup_tests();

        cpu.registers.PC = 0x100 as u16;

        {
            let mut mem = mem.borrow_mut();
            mem.store(0x100, 0xa5);
            mem.store(0x101, 0xDE);

            mem.store(0xDE, 0xAB);
        }

        cpu.execute();

        let val = cpu.registers.A;
        assert_eq!(val, 0xAB);
    }
}
