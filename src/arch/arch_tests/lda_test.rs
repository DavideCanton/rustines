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
}
