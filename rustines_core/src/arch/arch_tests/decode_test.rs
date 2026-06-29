#[cfg(test)]
mod tests {
    use crate::arch::{arch_tests::test_common::tests::setup_tests, memory::FetchStore};

    #[test]
    fn test_decode_absolute() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc + 1, 0xcd);
        cpu.memory.store(cpu.registers.pc + 2, 0xab);
        cpu.registers.pc += 3;

        let addr = cpu.decode_absolute();

        assert_eq!(addr, 0xabcd);
    }

    #[test]
    fn test_decode_immediate() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc + 1, 0xcd);
        cpu.registers.pc += 2;

        let addr = cpu.decode_immediate();

        assert_eq!(addr, 0xcd);
    }

    #[test]
    fn test_decode_zeropage() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc + 1, 0xcd);
        cpu.registers.pc += 2;

        let addr = cpu.decode_zeropage();

        assert_eq!(addr, 0xcd);
    }

    #[test]
    fn test_decode_absolute_indexed() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc + 1, 0xcd);
        cpu.memory.store(cpu.registers.pc + 2, 0xab);
        cpu.registers.pc += 3;

        let (addr, _) = cpu.decode_absolute_indexed(0x10);

        assert_eq!(addr, 0xabdd);
    }

    #[test]
    fn test_decode_absolute_indexed_wrapping() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc + 1, 0xfe);
        cpu.memory.store(cpu.registers.pc + 2, 0xff);
        cpu.registers.pc += 3;

        let (addr, _) = cpu.decode_absolute_indexed(0x10);

        assert_eq!(addr, 0x000e);
    }

    #[test]
    fn test_decode_zeropage_indexed() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc + 1, 0xcd);
        cpu.registers.pc += 2;

        let addr = cpu.decode_zeropage_indexed(0x10);

        assert_eq!(addr, 0xdd);
    }

    #[test]
    fn test_decode_zeropage_indexed_wrapping() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc + 1, 0xfe);
        cpu.registers.pc += 2;

        let addr = cpu.decode_zeropage_indexed(0x10);

        assert_eq!(addr, 0x0e);
    }

    #[test]
    fn test_decode_indexed_indirect() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc + 1, 0xcd);
        cpu.registers.pc += 2;

        cpu.memory.store(0xdd, 0xcd);
        cpu.memory.store(0xde, 0xab);

        cpu.registers.x_reg = 0x10;

        let addr = cpu.decode_indexed_indirect();

        assert_eq!(addr, 0xabcd);
    }

    #[test]
    fn test_decode_indexed_indirect_wrapping() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc + 1, 0xff);
        cpu.registers.pc += 2;

        cpu.memory.store(0x0f, 0xcd);
        cpu.memory.store(0x10, 0xab);

        cpu.registers.x_reg = 0x10;

        let addr = cpu.decode_indexed_indirect();

        assert_eq!(addr, 0xabcd);
    }

    #[test]
    fn test_decode_indirect_indexed() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc + 1, 0xcd);
        cpu.registers.pc += 2;

        cpu.memory.store(0xcd, 0xcd);
        cpu.memory.store(0xce, 0xab);

        cpu.registers.y_reg = 0x10;

        let (addr, _) = cpu.decode_indirect_indexed();

        assert_eq!(addr, 0xabdd);
    }

    #[test]
    fn test_decode_indirect_indexed_wrapping() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc + 1, 0xcd);
        cpu.registers.pc += 2;

        cpu.memory.store(0xcd, 0xfe);
        cpu.memory.store(0xce, 0xff);

        cpu.registers.y_reg = 0x10;

        let (addr, _) = cpu.decode_indirect_indexed();

        assert_eq!(addr, 0x000e);
    }
}
