#[cfg(test)]
mod tests {
    use crate::arch::arch_tests::test_common::tests::setup_tests;
    use crate::arch::bus::FetchStore;
    use crate::arch::instrs::lda;

    #[test]
    fn test_lda_immediate() {
        let (mut cpu, mut bus) = setup_tests();

        bus.store(cpu.registers.pc, 0xa9);
        bus.store(cpu.registers.pc + 1, 0xDE);

        cpu.registers.pc += 2;

        let cycles = lda::immediate(&mut cpu, &mut bus);

        assert_eq!(2, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xDE);
    }

    #[test]
    fn test_lda_zeropage() {
        let (mut cpu, mut bus) = setup_tests();

        bus.store(cpu.registers.pc, 0xa5);
        bus.store(cpu.registers.pc + 1, 0xDE);
        bus.store(0xDE, 0xAB);

        cpu.registers.pc += 2;

        let cycles = lda::zeropage(&mut cpu, &mut bus);

        assert_eq!(3, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_zeropage_x() {
        let (mut cpu, mut bus) = setup_tests();

        bus.store(cpu.registers.pc, 0xb5);
        bus.store(cpu.registers.pc + 1, 0xDE);
        bus.store(0xEE, 0xAB);

        cpu.registers.pc += 2;

        cpu.registers.x_reg = 0x10;

        let cycles = lda::zeropage_x(&mut cpu, &mut bus);

        assert_eq!(4, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_zeropage_x_flipping() {
        let (mut cpu, mut bus) = setup_tests();

        bus.store(cpu.registers.pc, 0xb5);
        bus.store(cpu.registers.pc + 1, 0xFF);
        bus.store(0x0, 0xAB);

        cpu.registers.pc += 2;

        cpu.registers.x_reg = 0x1;

        let cycles = lda::zeropage_x(&mut cpu, &mut bus);

        assert_eq!(4, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_absolute() {
        let (mut cpu, mut bus) = setup_tests();

        bus.store(cpu.registers.pc, 0xad);
        bus.store(cpu.registers.pc + 1, 0x34);
        bus.store(cpu.registers.pc + 2, 0x12);
        bus.store(0x1234, 0xAB);

        cpu.registers.pc += 3;

        let cycles = lda::absolute(&mut cpu, &mut bus);

        assert_eq!(4, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_absolute_x() {
        let (mut cpu, mut bus) = setup_tests();

        bus.store(cpu.registers.pc, 0xbd);
        bus.store(cpu.registers.pc + 1, 0x34);
        bus.store(cpu.registers.pc + 2, 0x12);
        bus.store(0x1244, 0xAB);

        cpu.registers.pc += 3;

        cpu.registers.x_reg = 0x10;

        let cycles = lda::absolute_x(&mut cpu, &mut bus);

        assert_eq!(4, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_absolute_x_flipping() {
        let (mut cpu, mut bus) = setup_tests();

        bus.store(cpu.registers.pc, 0xbd);
        bus.store(cpu.registers.pc + 1, 0xFE);
        bus.store(cpu.registers.pc + 2, 0xFF);

        cpu.registers.pc += 3;

        bus.store(0x0001, 0xAB);

        cpu.registers.x_reg = 0x3;

        let cycles = lda::absolute_x(&mut cpu, &mut bus);

        assert_eq!(5, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_absolute_y() {
        let (mut cpu, mut bus) = setup_tests();

        bus.store(cpu.registers.pc, 0xb9);
        bus.store(cpu.registers.pc + 1, 0x34);
        bus.store(cpu.registers.pc + 2, 0x12);

        cpu.registers.pc += 3;

        bus.store(0x1244, 0xAB);

        cpu.registers.y_reg = 0x10;

        let cycles = lda::absolute_y(&mut cpu, &mut bus);

        assert_eq!(4, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_absolute_y_flipping() {
        let (mut cpu, mut bus) = setup_tests();

        bus.store(cpu.registers.pc, 0xb9);
        bus.store(cpu.registers.pc + 1, 0xFE);
        bus.store(cpu.registers.pc + 2, 0xFF);

        cpu.registers.pc += 3;

        bus.store(0x0001, 0xAB);

        cpu.registers.y_reg = 0x3;

        let cycles = lda::absolute_y(&mut cpu, &mut bus);

        assert_eq!(5, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_indirect_x() {
        let (mut cpu, mut bus) = setup_tests();

        bus.store(cpu.registers.pc, 0xa1);
        bus.store(cpu.registers.pc + 1, 0x34);
        cpu.registers.pc += 2;

        bus.store(0x44, 0x10);
        bus.store(0x45, 0x11);

        bus.store(0x1110, 0xAB);

        cpu.registers.x_reg = 0x10;

        let cycles = lda::indirect_x(&mut cpu, &mut bus);

        assert_eq!(6, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_indirect_x_flipping() {
        let (mut cpu, mut bus) = setup_tests();

        bus.store(cpu.registers.pc, 0xa1);
        bus.store(cpu.registers.pc + 1, 0xFE);
        cpu.registers.pc += 2;

        bus.store(0x0E, 0x10);
        bus.store(0x0F, 0x11);

        bus.store(0x1110, 0xAB);

        cpu.registers.x_reg = 0x10;

        let cycles = lda::indirect_x(&mut cpu, &mut bus);

        assert_eq!(6, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_indirect_y() {
        let (mut cpu, mut bus) = setup_tests();

        bus.store(cpu.registers.pc, 0xb1);
        bus.store(cpu.registers.pc + 1, 0x34);
        cpu.registers.pc += 2;

        bus.store(0x34, 0x10);
        bus.store(0x35, 0x11);

        bus.store(0x1120, 0xAB);

        cpu.registers.y_reg = 0x10;

        let cycles = lda::indirect_y(&mut cpu, &mut bus);

        assert_eq!(5, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_indirect_y_flipping() {
        let (mut cpu, mut bus) = setup_tests();

        bus.store(cpu.registers.pc, 0xb1);
        bus.store(cpu.registers.pc + 1, 0x0E);
        cpu.registers.pc += 2;

        bus.store(0x0E, 0xFE);
        bus.store(0x0F, 0xFF);

        bus.store(0x001E, 0xAB);

        cpu.registers.y_reg = 0x20;

        let cycles = lda::indirect_y(&mut cpu, &mut bus);

        assert_eq!(6, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }
}
