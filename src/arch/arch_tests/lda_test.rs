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
    use arch::instrs::lda;
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

        let (cycles, ilen) = lda::immediate(&mut cpu);

        assert_eq!(2, cycles);
        assert_eq!(2, ilen);

        let val = cpu.registers.A;
        assert_eq!(val, 0xDE);
        
    }

    #[test]
    pub fn test_lda_zeropage()
    {
        let (mut cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC, 0xa5);
            mem.store(cpu.registers.PC + 1, 0xDE);

            mem.store(0xDE, 0xAB);
        }

        let (cycles, ilen) = lda::zeropage(&mut cpu);

        assert_eq!(3, cycles);
        assert_eq!(2, ilen);

        let val = cpu.registers.A;
        assert_eq!(val, 0xAB);
    }

    #[test]
    pub fn test_lda_zeropage_x()
    {
        let (mut cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC, 0xb5);
            mem.store(cpu.registers.PC + 1, 0xDE);

            mem.store(0xEE, 0xAB);
        }

        cpu.registers.X = 0x10;

        let (cycles, ilen) = lda::zeropage_x(&mut cpu);

        assert_eq!(4, cycles);
        assert_eq!(2, ilen);

        let val = cpu.registers.A;
        assert_eq!(val, 0xAB);
    }

    #[test]
    pub fn test_lda_zeropage_x_flipping()
    {
        let (mut cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC, 0xb5);
            mem.store(cpu.registers.PC + 1, 0xFF);

            mem.store(0x0, 0xAB);
        }

        cpu.registers.X = 0x1;

        let (cycles, ilen) = lda::zeropage_x(&mut cpu);

        assert_eq!(4, cycles);
        assert_eq!(2, ilen);

        let val = cpu.registers.A;
        assert_eq!(val, 0xAB);
    }
}
