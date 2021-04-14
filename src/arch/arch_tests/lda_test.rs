#[cfg(test)]
mod tests {
    use crate::arch::arch_tests::common::tests::setup_tests;
    use crate::arch::instrs::lda;

    #[test]
    fn test_lda_immediate() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc, 0xa9);
        cpu.memory.store(cpu.registers.pc + 1, 0xDE);

        let (cycles, ilen) = lda::immediate(&mut cpu);

        assert_eq!(2, cycles);
        assert_eq!(2, ilen);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xDE);
    }

    #[test]
    fn test_lda_zeropage() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc, 0xa5);
        cpu.memory.store(cpu.registers.pc + 1, 0xDE);
        cpu.memory.store(0xDE, 0xAB);

        let (cycles, ilen) = lda::zeropage(&mut cpu);

        assert_eq!(3, cycles);
        assert_eq!(2, ilen);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_zeropage_x() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc, 0xb5);
        cpu.memory.store(cpu.registers.pc + 1, 0xDE);
        cpu.memory.store(0xEE, 0xAB);

        cpu.registers.x_reg = 0x10;

        let (cycles, ilen) = lda::zeropage_x(&mut cpu);

        assert_eq!(4, cycles);
        assert_eq!(2, ilen);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_zeropage_x_flipping() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc, 0xb5);
        cpu.memory.store(cpu.registers.pc + 1, 0xFF);
        cpu.memory.store(0x0, 0xAB);

        cpu.registers.x_reg = 0x1;

        let (cycles, ilen) = lda::zeropage_x(&mut cpu);

        assert_eq!(4, cycles);
        assert_eq!(2, ilen);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_absolute() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc, 0xad);
        cpu.memory.store(cpu.registers.pc + 1, 0x34);
        cpu.memory.store(cpu.registers.pc + 2, 0x12);
        cpu.memory.store(0x1234, 0xAB);

        let (cycles, ilen) = lda::absolute(&mut cpu);

        assert_eq!(4, cycles);
        assert_eq!(3, ilen);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_absolute_x() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc, 0xbd);
        cpu.memory.store(cpu.registers.pc + 1, 0x34);
        cpu.memory.store(cpu.registers.pc + 2, 0x12);
        cpu.memory.store(0x1244, 0xAB);

        cpu.registers.x_reg = 0x10;

        let (cycles, ilen) = lda::absolute_x(&mut cpu);

        assert_eq!(4, cycles);
        assert_eq!(3, ilen);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_absolute_x_flipping() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc, 0xbd);
        cpu.memory.store(cpu.registers.pc + 1, 0xFE);
        cpu.memory.store(cpu.registers.pc + 2, 0xFF);

        cpu.memory.store(0x0001, 0xAB);

        cpu.registers.x_reg = 0x3;

        let (cycles, ilen) = lda::absolute_x(&mut cpu);

        assert_eq!(4, cycles);
        assert_eq!(3, ilen);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_absolute_y() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc, 0xb9);
        cpu.memory.store(cpu.registers.pc + 1, 0x34);
        cpu.memory.store(cpu.registers.pc + 2, 0x12);

        cpu.memory.store(0x1244, 0xAB);

        cpu.registers.y_reg = 0x10;

        let (cycles, ilen) = lda::absolute_y(&mut cpu);

        assert_eq!(4, cycles);
        assert_eq!(3, ilen);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_absolute_y_flipping() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc, 0xb9);
        cpu.memory.store(cpu.registers.pc + 1, 0xFE);
        cpu.memory.store(cpu.registers.pc + 2, 0xFF);

        cpu.memory.store(0x0001, 0xAB);

        cpu.registers.y_reg = 0x3;

        let (cycles, ilen) = lda::absolute_y(&mut cpu);

        assert_eq!(4, cycles);
        assert_eq!(3, ilen);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_indirect_x() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc, 0xa1);
        cpu.memory.store(cpu.registers.pc + 1, 0x34);

        cpu.memory.store(0x44, 0x10);
        cpu.memory.store(0x45, 0x11);

        cpu.memory.store(0x1110, 0xAB);

        cpu.registers.x_reg = 0x10;

        let (cycles, ilen) = lda::indirect_x(&mut cpu);

        assert_eq!(6, cycles);
        assert_eq!(2, ilen);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_indirect_x_flipping() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc, 0xa1);
        cpu.memory.store(cpu.registers.pc + 1, 0xFE);

        cpu.memory.store(0x0E, 0x10);
        cpu.memory.store(0x0F, 0x11);

        cpu.memory.store(0x1110, 0xAB);

        cpu.registers.x_reg = 0x10;

        let (cycles, ilen) = lda::indirect_x(&mut cpu);

        assert_eq!(6, cycles);
        assert_eq!(2, ilen);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_indirect_y() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc, 0xb1);
        cpu.memory.store(cpu.registers.pc + 1, 0x34);

        cpu.memory.store(0x34, 0x10);
        cpu.memory.store(0x35, 0x11);

        cpu.memory.store(0x1120, 0xAB);

        cpu.registers.y_reg = 0x10;

        let (cycles, ilen) = lda::indirect_y(&mut cpu);

        assert_eq!(5, cycles);
        assert_eq!(2, ilen);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_indirect_y_flipping() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc, 0xb1);
        cpu.memory.store(cpu.registers.pc + 1, 0x0E);

        cpu.memory.store(0x0E, 0xFE);
        cpu.memory.store(0x0F, 0xFF);

        cpu.memory.store(0x001E, 0xAB);

        cpu.registers.y_reg = 0x20;

        let (cycles, ilen) = lda::indirect_y(&mut cpu);

        assert_eq!(5, cycles);
        assert_eq!(2, ilen);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }
}
