#[cfg(test)]
mod tests {
    use arch::arch_tests::common::tests::setup_tests;
    use arch::instrs::pushpop;

    #[test]
    fn test_pha() {
        let (mut cpu, _) = setup_tests();

        cpu.registers.A = 0xAB;
        let old_sp = cpu.registers.SP;

        let (cycles, ilen) = pushpop::pha(&mut cpu);

        assert_eq!(3, cycles);
        assert_eq!(1, ilen);

        let val = cpu.peek8();
        assert_eq!(val, 0xAB);

        let sp = cpu.registers.SP;
        assert_eq!(sp, old_sp - 1);
    }

    #[test]
    fn test_php_1() {
        let (mut cpu, _) = setup_tests();
        let old_sp = cpu.registers.SP;

        cpu.registers.setC();
        cpu.registers.setN();
        cpu.registers.setZ();
        cpu.registers.setV();
        cpu.registers.setB();
        cpu.registers.setD();
        cpu.registers.setI();

        let (cycles, ilen) = pushpop::php(&mut cpu);

        assert_eq!(3, cycles);
        assert_eq!(1, ilen);

        let val = cpu.peek8();
        assert_eq!(val, 0xFF);

        let sp = cpu.registers.SP;
        assert_eq!(sp, old_sp - 1);
    }

    #[test]
    fn test_php_0() {
        let (mut cpu, _) = setup_tests();
        let old_sp = cpu.registers.SP;

        cpu.registers.clearC();
        cpu.registers.clearN();
        cpu.registers.clearZ();
        cpu.registers.clearV();
        cpu.registers.clearB();
        cpu.registers.clearD();
        cpu.registers.clearI();

        let (cycles, ilen) = pushpop::php(&mut cpu);

        assert_eq!(3, cycles);
        assert_eq!(1, ilen);

        let val = cpu.peek8();
        assert_eq!(val, 0x20);

        let sp = cpu.registers.SP;
        assert_eq!(sp, old_sp - 1);
    }

    #[test]
    fn test_php_alt() {
        let (mut cpu, _) = setup_tests();
        let old_sp = cpu.registers.SP;

        cpu.registers.setN();
        cpu.registers.clearV();
        cpu.registers.clearB();
        cpu.registers.setD();
        cpu.registers.clearI();
        cpu.registers.setZ();
        cpu.registers.clearC();

        let (cycles, ilen) = pushpop::php(&mut cpu);

        assert_eq!(3, cycles);
        assert_eq!(1, ilen);

        let val = cpu.peek8();
        assert_eq!(val, 0xAA);

        let sp = cpu.registers.SP;
        assert_eq!(sp, old_sp - 1);
    }

    #[test]
    fn test_pla() {
        let (mut cpu, _) = setup_tests();
        let old_sp = cpu.registers.SP;

        cpu.push8(0xAB);

        let (cycles, ilen) = pushpop::pla(&mut cpu);

        assert_eq!(4, cycles);
        assert_eq!(1, ilen);

        let val = cpu.registers.A;
        assert_eq!(val, 0xAB);

        let sp = cpu.registers.SP;
        assert_eq!(sp, old_sp);
    }

    #[test]
    fn test_plp1() {
        let (mut cpu, _) = setup_tests();
        let old_sp = cpu.registers.SP;

        cpu.push8(0xFF);

        let (cycles, ilen) = pushpop::plp(&mut cpu);

        assert_eq!(4, cycles);
        assert_eq!(1, ilen);

        assert_eq!(cpu.registers.getC(), true);
        assert_eq!(cpu.registers.getN(), true);
        assert_eq!(cpu.registers.getZ(), true);
        assert_eq!(cpu.registers.getV(), true);
        assert_eq!(cpu.registers.getB(), true);
        assert_eq!(cpu.registers.getD(), true);
        assert_eq!(cpu.registers.getI(), true);

        let sp = cpu.registers.SP;
        assert_eq!(sp, old_sp);
    }

    #[test]
    fn test_plp0() {
        let (mut cpu, _) = setup_tests();
        let old_sp = cpu.registers.SP;

        cpu.push8(0x00);

        let (cycles, ilen) = pushpop::plp(&mut cpu);

        assert_eq!(4, cycles);
        assert_eq!(1, ilen);

        assert_eq!(cpu.registers.getC(), false);
        assert_eq!(cpu.registers.getN(), false);
        assert_eq!(cpu.registers.getZ(), false);
        assert_eq!(cpu.registers.getV(), false);
        assert_eq!(cpu.registers.getB(), false);
        assert_eq!(cpu.registers.getD(), false);
        assert_eq!(cpu.registers.getI(), false);

        let sp = cpu.registers.SP;
        assert_eq!(sp, old_sp);
    }

    #[test]
    fn test_plp_alt() {
        let (mut cpu, _) = setup_tests();
        let old_sp = cpu.registers.SP;

        cpu.push8(0xAA);

        let (cycles, ilen) = pushpop::plp(&mut cpu);

        assert_eq!(4, cycles);
        assert_eq!(1, ilen);

        assert_eq!(cpu.registers.getN(), true);
        assert_eq!(cpu.registers.getV(), false);
        assert_eq!(cpu.registers.getB(), false);
        assert_eq!(cpu.registers.getD(), true);
        assert_eq!(cpu.registers.getI(), false);
        assert_eq!(cpu.registers.getZ(), true);
        assert_eq!(cpu.registers.getC(), false);

        let sp = cpu.registers.SP;
        assert_eq!(sp, old_sp);
    }
}
