#[cfg(test)]
mod tests {
    use crate::arch::arch_tests::common::tests::setup_tests;
    use crate::utils::bit_utils::*;

    #[test]
    fn test_decode_absolute() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc + 1, 0xcd);
        cpu.memory.store(cpu.registers.pc + 2, 0xab);

        let (addr, ilen) = decode_absolute!(&cpu);

        assert_eq!(ilen, 3);
        assert_eq!(addr, 0xabcd);
    }

    #[test]
    fn test_decode_immediate() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc + 1, 0xcd);

        let (addr, ilen) = decode_immediate!(&cpu);

        assert_eq!(ilen, 2);
        assert_eq!(addr, 0xcd);
    }

    #[test]
    fn test_decode_zeropage() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc + 1, 0xcd);

        let (addr, ilen) = decode_zeropage!(&cpu);

        assert_eq!(ilen, 2);
        assert_eq!(addr, 0xcd);
    }

    #[test]
    fn test_decode_absolute_indexed() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc + 1, 0xcd);
        cpu.memory.store(cpu.registers.pc + 2, 0xab);

        let (addr, ilen) = decode_absolute_indexed!(&cpu, 0x10);

        assert_eq!(ilen, 3);
        assert_eq!(addr, 0xabdd);
    }

    #[test]
    fn test_decode_absolute_indexed_wrapping() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc + 1, 0xfe);
        cpu.memory.store(cpu.registers.pc + 2, 0xff);

        let (addr, ilen) = decode_absolute_indexed!(&cpu, 0x10);

        assert_eq!(ilen, 3);
        assert_eq!(addr, 0x000e);
    }

    #[test]
    fn test_decode_zeropage_indexed() {
        let mut cpu = setup_tests();



        cpu.memory.store(cpu.registers.pc + 1, 0xcd);


        let (addr, ilen) = decode_zeropage_indexed!(&cpu, 0x10);

        assert_eq!(ilen, 2);
        assert_eq!(addr, 0xdd);
    }

    #[test]
    fn test_decode_zeropage_indexed_wrapping() {
        let mut cpu = setup_tests();



        cpu.memory.store(cpu.registers.pc + 1, 0xfe);


        let (addr, ilen) = decode_zeropage_indexed!(&cpu, 0x10);

        assert_eq!(ilen, 2);
        assert_eq!(addr, 0x0e);
    }

    #[test]
    fn test_decode_indexed_indirect() {
        let mut cpu = setup_tests();



        cpu.memory.store(cpu.registers.pc + 1, 0xcd);

        cpu.memory.store(0xdd, 0xcd);
        cpu.memory.store(0xde, 0xab);


        cpu.registers.x_reg = 0x10;

        let (addr, ilen) = decode_indexed_indirect!(&cpu);

        assert_eq!(ilen, 2);
        assert_eq!(addr, 0xabcd);
    }

    #[test]
    fn test_decode_indexed_indirect_wrapping() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc + 1, 0xff);

        cpu.memory.store(0x0f, 0xcd);
        cpu.memory.store(0x10, 0xab);

        cpu.registers.x_reg = 0x10;

        let (addr, ilen) = decode_indexed_indirect!(&cpu);

        assert_eq!(ilen, 2);
        assert_eq!(addr, 0xabcd);
    }

    #[test]
    fn test_decode_indirect_indexed() {
        let mut cpu = setup_tests();



        cpu.memory.store(cpu.registers.pc + 1, 0xcd);

        cpu.memory.store(0xcd, 0xcd);
        cpu.memory.store(0xce, 0xab);

        cpu.registers.y_reg = 0x10;

        let (addr, ilen) = decode_indirect_indexed!(&cpu);

        assert_eq!(ilen, 2);
        assert_eq!(addr, 0xabdd);
    }

    #[test]
    fn test_decode_indirect_indexed_wrapping() {
        let mut cpu = setup_tests();

        cpu.memory.store(cpu.registers.pc + 1, 0xcd);

        cpu.memory.store(0xcd, 0xfe);
        cpu.memory.store(0xce, 0xff);

        cpu.registers.y_reg = 0x10;

        let (addr, ilen) = decode_indirect_indexed!(&cpu);

        assert_eq!(ilen, 2);
        assert_eq!(addr, 0x000e);
    }
}
