#[cfg(test)]
mod tests {
    use arch::arch_tests::common::tests::setup_tests;
    use arch::instrs::pushpop;

    #[test]
    fn test_pha() {
        let mut cpu = setup_tests();

        cpu.registers.a_reg = 0xAB;
        let old_sp = cpu.registers.sp;

        let (cycles, ilen) = pushpop::pha(&mut cpu);

        assert_eq!(3, cycles);
        assert_eq!(1, ilen);

        let val = cpu.peek8();
        assert_eq!(val, 0xAB);

        let sp = cpu.registers.sp;
        assert_eq!(sp, old_sp - 1);
    }

    #[test]
    fn test_php_1() {
        let mut cpu = setup_tests();
        let old_sp = cpu.registers.sp;

        cpu.registers.set_c();
        cpu.registers.set_n();
        cpu.registers.set_z();
        cpu.registers.set_v();
        cpu.registers.set_b();
        cpu.registers.set_d();
        cpu.registers.set_i();

        let (cycles, ilen) = pushpop::php(&mut cpu);

        assert_eq!(3, cycles);
        assert_eq!(1, ilen);

        let val = cpu.peek8();
        assert_eq!(val, 0xFF);

        let sp = cpu.registers.sp;
        assert_eq!(sp, old_sp - 1);
    }

    #[test]
    fn test_php_0() {
        let mut cpu = setup_tests();
        let old_sp = cpu.registers.sp;

        cpu.registers.clear_c();
        cpu.registers.clear_n();
        cpu.registers.clear_z();
        cpu.registers.clear_v();
        cpu.registers.clear_b();
        cpu.registers.clear_d();
        cpu.registers.clear_i();

        let (cycles, ilen) = pushpop::php(&mut cpu);

        assert_eq!(3, cycles);
        assert_eq!(1, ilen);

        let val = cpu.peek8();
        assert_eq!(val, 0x20);

        let sp = cpu.registers.sp;
        assert_eq!(sp, old_sp - 1);
    }

    #[test]
    fn test_php_alt() {
        let mut cpu = setup_tests();
        let old_sp = cpu.registers.sp;

        cpu.registers.set_n();
        cpu.registers.clear_v();
        cpu.registers.clear_b();
        cpu.registers.set_d();
        cpu.registers.clear_i();
        cpu.registers.set_z();
        cpu.registers.clear_c();

        let (cycles, ilen) = pushpop::php(&mut cpu);

        assert_eq!(3, cycles);
        assert_eq!(1, ilen);

        let val = cpu.peek8();
        assert_eq!(val, 0xAA);

        let sp = cpu.registers.sp;
        assert_eq!(sp, old_sp - 1);
    }

    #[test]
    fn test_pla() {
        let mut cpu = setup_tests();
        let old_sp = cpu.registers.sp;

        cpu.push8(0xAB);

        let (cycles, ilen) = pushpop::pla(&mut cpu);

        assert_eq!(4, cycles);
        assert_eq!(1, ilen);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);

        let sp = cpu.registers.sp;
        assert_eq!(sp, old_sp);
    }

    #[test]
    fn test_plp1() {
        let mut cpu = setup_tests();
        let old_sp = cpu.registers.sp;

        cpu.push8(0xFF);

        let (cycles, ilen) = pushpop::plp(&mut cpu);

        assert_eq!(4, cycles);
        assert_eq!(1, ilen);

        assert_eq!(cpu.registers.get_c(), true);
        assert_eq!(cpu.registers.get_n(), true);
        assert_eq!(cpu.registers.get_z(), true);
        assert_eq!(cpu.registers.get_v(), true);
        assert_eq!(cpu.registers.get_b(), true);
        assert_eq!(cpu.registers.get_d(), true);
        assert_eq!(cpu.registers.get_i(), true);

        let sp = cpu.registers.sp;
        assert_eq!(sp, old_sp);
    }

    #[test]
    fn test_plp0() {
        let mut cpu = setup_tests();
        let old_sp = cpu.registers.sp;

        cpu.push8(0x00);

        let (cycles, ilen) = pushpop::plp(&mut cpu);

        assert_eq!(4, cycles);
        assert_eq!(1, ilen);

        assert_eq!(cpu.registers.get_c(), false);
        assert_eq!(cpu.registers.get_n(), false);
        assert_eq!(cpu.registers.get_z(), false);
        assert_eq!(cpu.registers.get_v(), false);
        assert_eq!(cpu.registers.get_b(), false);
        assert_eq!(cpu.registers.get_d(), false);
        assert_eq!(cpu.registers.get_i(), false);

        let sp = cpu.registers.sp;
        assert_eq!(sp, old_sp);
    }

    #[test]
    fn test_plp_alt() {
        let mut cpu = setup_tests();
        let old_sp = cpu.registers.sp;

        cpu.push8(0xAA);

        let (cycles, ilen) = pushpop::plp(&mut cpu);

        assert_eq!(4, cycles);
        assert_eq!(1, ilen);

        assert_eq!(cpu.registers.get_n(), true);
        assert_eq!(cpu.registers.get_v(), false);
        assert_eq!(cpu.registers.get_b(), false);
        assert_eq!(cpu.registers.get_d(), true);
        assert_eq!(cpu.registers.get_i(), false);
        assert_eq!(cpu.registers.get_z(), true);
        assert_eq!(cpu.registers.get_c(), false);

        let sp = cpu.registers.sp;
        assert_eq!(sp, old_sp);
    }
}
