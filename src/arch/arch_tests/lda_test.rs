#[cfg(test)]
mod tests
{
    use arch::arch_tests::common::tests::setup_tests;
    use arch::instrs::lda;

    #[test]
    fn test_lda_immediate()
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
    fn test_lda_zeropage()
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
    fn test_lda_zeropage_x()
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
    fn test_lda_zeropage_x_flipping()
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

    #[test]
    fn test_lda_absolute()
    {
        let (mut cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC, 0xad);
            mem.store(cpu.registers.PC + 1, 0x34);
            mem.store(cpu.registers.PC + 2, 0x12);

            mem.store(0x1234, 0xAB);
        }

        let (cycles, ilen) = lda::absolute(&mut cpu);

        assert_eq!(4, cycles);
        assert_eq!(3, ilen);

        let val = cpu.registers.A;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_absolute_x()
    {
        let (mut cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC, 0xbd);
            mem.store(cpu.registers.PC + 1, 0x34);
            mem.store(cpu.registers.PC + 2, 0x12);

            mem.store(0x1244, 0xAB);
        }
        cpu.registers.X = 0x10;

        let (cycles, ilen) = lda::absolute_x(&mut cpu);

        assert_eq!(4, cycles);
        assert_eq!(3, ilen);

        let val = cpu.registers.A;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_absolute_x_flipping()
    {
        let (mut cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC, 0xbd);
            mem.store(cpu.registers.PC + 1, 0xFE);
            mem.store(cpu.registers.PC + 2, 0xFF);

            mem.store(0x0001, 0xAB);
        }
        cpu.registers.X = 0x3;

        let (cycles, ilen) = lda::absolute_x(&mut cpu);

        assert_eq!(4, cycles);
        assert_eq!(3, ilen);

        let val = cpu.registers.A;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_absolute_y()
    {
        let (mut cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC, 0xb9);
            mem.store(cpu.registers.PC + 1, 0x34);
            mem.store(cpu.registers.PC + 2, 0x12);

            mem.store(0x1244, 0xAB);
        }
        cpu.registers.Y = 0x10;

        let (cycles, ilen) = lda::absolute_y(&mut cpu);

        assert_eq!(4, cycles);
        assert_eq!(3, ilen);

        let val = cpu.registers.A;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_absolute_y_flipping()
    {
        let (mut cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC, 0xb9);
            mem.store(cpu.registers.PC + 1, 0xFE);
            mem.store(cpu.registers.PC + 2, 0xFF);

            mem.store(0x0001, 0xAB);
        }
        cpu.registers.Y = 0x3;

        let (cycles, ilen) = lda::absolute_y(&mut cpu);

        assert_eq!(4, cycles);
        assert_eq!(3, ilen);

        let val = cpu.registers.A;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_indirect_x()
    {
        let (mut cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC, 0xa1);
            mem.store(cpu.registers.PC + 1, 0x34);

            mem.store(0x44, 0x10);
            mem.store(0x45, 0x11);

            mem.store(0x1110, 0xAB);
        }
        cpu.registers.X = 0x10;

        let (cycles, ilen) = lda::indirect_x(&mut cpu);

        assert_eq!(6, cycles);
        assert_eq!(2, ilen);

        let val = cpu.registers.A;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_indirect_x_flipping()
    {
        let (mut cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC, 0xa1);
            mem.store(cpu.registers.PC + 1, 0xFE);

            mem.store(0x0E, 0x10);
            mem.store(0x0F, 0x11);

            mem.store(0x1110, 0xAB);
        }
        cpu.registers.X = 0x10;

        let (cycles, ilen) = lda::indirect_x(&mut cpu);

        assert_eq!(6, cycles);
        assert_eq!(2, ilen);

        let val = cpu.registers.A;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_indirect_y()
    {
        let (mut cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC, 0xb1);
            mem.store(cpu.registers.PC + 1, 0x34);

            mem.store(0x34, 0x10);
            mem.store(0x35, 0x11);

            mem.store(0x1120, 0xAB);
        }
        cpu.registers.Y = 0x10;

        let (cycles, ilen) = lda::indirect_y(&mut cpu);

        assert_eq!(5, cycles);
        assert_eq!(2, ilen);

        let val = cpu.registers.A;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_indirect_y_flipping()
    {
        let (mut cpu, mem) = setup_tests();

        {
            let mut mem = mem.borrow_mut();
            mem.store(cpu.registers.PC, 0xb1);
            mem.store(cpu.registers.PC + 1, 0x0E);

            mem.store(0x0E, 0xFE);
            mem.store(0x0F, 0xFF);

            mem.store(0x001E, 0xAB);
        }
        cpu.registers.Y = 0x20;

        let (cycles, ilen) = lda::indirect_y(&mut cpu);

        assert_eq!(5, cycles);
        assert_eq!(2, ilen);

        let val = cpu.registers.A;
        assert_eq!(val, 0xAB);
    }
}
