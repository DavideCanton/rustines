#[cfg(test)]
mod tests {
    use crate::arch::arch_tests::test_common::tests::setup_tests;

    #[test]
    fn test_decode_absolute() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc + 1, 0xcd);
        bus.write(cpu.registers.pc + 2, 0xab);
        cpu.registers.pc += 1;

        let addr = cpu.decode_absolute(&mut bus);

        assert_eq!(addr, 0xabcd);
    }

    #[test]
    fn test_decode_immediate() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc + 1, 0xcd);
        cpu.registers.pc += 1;

        let addr = cpu.decode_immediate(&mut bus);

        assert_eq!(addr, 0xcd);
    }

    #[test]
    fn test_decode_zeropage() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc + 1, 0xcd);
        cpu.registers.pc += 1;

        let addr = cpu.decode_zeropage(&mut bus);

        assert_eq!(addr, 0xcd);
    }

    #[test]
    fn test_decode_absolute_indexed() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc + 1, 0xcd);
        bus.write(cpu.registers.pc + 2, 0xab);
        cpu.registers.pc += 1;

        let result = cpu.decode_absolute_indexed(&mut bus, 0x10, false);

        assert_eq!(result.address(), 0xabdd);
    }

    #[test]
    fn test_decode_absolute_indexed_wrapping() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc + 1, 0xfe);
        bus.write(cpu.registers.pc + 2, 0xff);
        cpu.registers.pc += 1;

        let result = cpu.decode_absolute_indexed(&mut bus, 0x10, false);

        assert_eq!(result.address(), 0x000e);
    }

    #[test]
    fn test_decode_zeropage_indexed() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc + 1, 0xcd);
        cpu.registers.pc += 1;

        let addr = cpu.decode_zeropage_indexed(&mut bus, 0x10);

        assert_eq!(addr, 0xdd);
    }

    #[test]
    fn test_decode_zeropage_indexed_wrapping() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc + 1, 0xfe);
        cpu.registers.pc += 1;

        let addr = cpu.decode_zeropage_indexed(&mut bus, 0x10);

        assert_eq!(addr, 0x0e);
    }

    #[test]
    fn test_decode_indexed_indirect() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc + 1, 0xcd);
        cpu.registers.pc += 1;

        bus.write(0xdd, 0xcd);
        bus.write(0xde, 0xab);

        cpu.registers.x_reg = 0x10;

        let addr = cpu.decode_indexed_indirect(&mut bus);

        assert_eq!(addr, 0xabcd);
    }

    #[test]
    fn test_decode_indexed_indirect_wrapping() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc + 1, 0xff);
        cpu.registers.pc += 1;

        bus.write(0x0f, 0xcd);
        bus.write(0x10, 0xab);

        cpu.registers.x_reg = 0x10;

        let addr = cpu.decode_indexed_indirect(&mut bus);

        assert_eq!(addr, 0xabcd);
    }

    #[test]
    fn test_decode_indirect_indexed() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc + 1, 0xcd);
        cpu.registers.pc += 1;

        bus.write(0xcd, 0xcd);
        bus.write(0xce, 0xab);

        cpu.registers.y_reg = 0x10;

        let result = cpu.decode_indirect_indexed(&mut bus, false);

        assert_eq!(result.address(), 0xabdd);
    }

    #[test]
    fn test_decode_indirect_indexed_wrapping() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc + 1, 0xcd);
        cpu.registers.pc += 1;

        bus.write(0xcd, 0xfe);
        bus.write(0xce, 0xff);

        cpu.registers.y_reg = 0x10;

        let result = cpu.decode_indirect_indexed(&mut bus, false);

        assert_eq!(result.address(), 0x000e);
    }
}
