#[cfg(test)]
mod tests
{
    use arch::arch_tests::common::tests::setup_tests;
    use arch::instrs::instr_table::*;

    #[test]
    fn test_decode_absolute()
    {
        let (cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC + 1, 0xcd);
            mem.store(cpu.registers.PC + 2, 0xab);
        }

        let (addr, ilen) = decode_absolute(&cpu);

        assert_eq!(ilen, 3);
        assert_eq!(addr, 0xabcd);
    }

    #[test]
    fn test_decode_immediate()
    {
        let (cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC + 1, 0xcd);
        }

        let (addr, ilen) = decode_immediate(&cpu);

        assert_eq!(ilen, 2);
        assert_eq!(addr, 0xcd);
    }

    #[test]
    fn test_decode_zeropage()
    {
        let (cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC + 1, 0xcd);
        }

        let (addr, ilen) = decode_zeropage(&cpu);

        assert_eq!(ilen, 2);
        assert_eq!(addr, 0xcd);
    }

    #[test]
    fn test_decode_absolute_indexed()
    {
        let (cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC + 1, 0xcd);
            mem.store(cpu.registers.PC + 2, 0xab);
        }

        let (addr, ilen) = decode_absolute_indexed(&cpu, 0x10);

        assert_eq!(ilen, 3);
        assert_eq!(addr, 0xabdd);
    }

    #[test]
    fn test_decode_absolute_indexed_wrapping()
    {
        let (cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC + 1, 0xfe);
            mem.store(cpu.registers.PC + 2, 0xff);
        }

        let (addr, ilen) = decode_absolute_indexed(&cpu, 0x10);

        assert_eq!(ilen, 3);
        assert_eq!(addr, 0x000e);
    }

    #[test]
    fn test_decode_zeropage_indexed()
    {
        let (cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC + 1, 0xcd);
        }

        let (addr, ilen) = decode_zeropage_indexed(&cpu, 0x10);

        assert_eq!(ilen, 2);
        assert_eq!(addr, 0xdd);
    }

    #[test]
    fn test_decode_zeropage_indexed_wrapping()
    {
        let (cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC + 1, 0xfe);
        }

        let (addr, ilen) = decode_zeropage_indexed(&cpu, 0x10);

        assert_eq!(ilen, 2);
        assert_eq!(addr, 0x0e);
    }

    #[test]
    fn test_decode_indexed_indirect()
    {
        let (mut cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC + 1, 0xcd);

            mem.store(0xdd, 0xcd);
            mem.store(0xde, 0xab);
        }

        cpu.registers.X = 0x10;

        let (addr, ilen) = decode_indexed_indirect(&cpu);

        assert_eq!(ilen, 2);
        assert_eq!(addr, 0xabcd);
    }

    #[test]
    fn test_decode_indexed_indirect_wrapping()
    {
        let (mut cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC + 1, 0xff);

            mem.store(0x0f, 0xcd);
            mem.store(0x10, 0xab);
        }

        cpu.registers.X = 0x10;

        let (addr, ilen) = decode_indexed_indirect(&cpu);

        assert_eq!(ilen, 2);
        assert_eq!(addr, 0xabcd);
    }

    #[test]
    fn test_decode_indirect_indexed()
    {
        let (mut cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC + 1, 0xcd);

            mem.store(0xcd, 0xcd);
            mem.store(0xce, 0xab);
        }

        cpu.registers.Y = 0x10;

        let (addr, ilen) = decode_indirect_indexed(&cpu);

        assert_eq!(ilen, 2);
        assert_eq!(addr, 0xabdd);
    }

    #[test]
    fn test_decode_indirect_indexed_wrapping()
    {
        let (mut cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC + 1, 0xcd);

            mem.store(0xcd, 0xfe);
            mem.store(0xce, 0xff);
        }

        cpu.registers.Y = 0x10;

        let (addr, ilen) = decode_indirect_indexed(&cpu);

        assert_eq!(ilen, 2);
        assert_eq!(addr, 0x000e);
    }
}
