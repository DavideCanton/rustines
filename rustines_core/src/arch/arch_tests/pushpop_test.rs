#[cfg(test)]
mod tests {
    use crate::arch::arch_tests::test_common::tests::setup_tests;
    use crate::arch::instrs::pushpop;

    #[test]
    fn test_pha() {
        let (mut cpu, mut bus) = setup_tests();

        cpu.registers.a_reg = 0xAB;
        let old_sp = cpu.registers.sp;

        let cycles = pushpop::pha(&mut cpu, &mut bus);

        assert_eq!(3, cycles);

        let val = cpu.peek8(&mut bus);
        assert_eq!(val, 0xAB);

        let sp = cpu.registers.sp;
        assert_eq!(sp, old_sp - 1);
    }

    #[test]
    fn test_php_1() {
        let (mut cpu, mut bus) = setup_tests();
        let old_sp = cpu.registers.sp;

        cpu.registers.set_c();
        cpu.registers.set_n();
        cpu.registers.set_z();
        cpu.registers.set_v();
        cpu.registers.set_b();
        cpu.registers.set_d();
        cpu.registers.set_i();

        let cycles = pushpop::php(&mut cpu, &mut bus);

        assert_eq!(3, cycles);

        let val = cpu.peek8(&mut bus);
        assert_eq!(val, 0xFF);

        let sp = cpu.registers.sp;
        assert_eq!(sp, old_sp - 1);
    }

    #[test]
    fn test_php_0() {
        let (mut cpu, mut bus) = setup_tests();
        let old_sp = cpu.registers.sp;

        cpu.registers.clear_c();
        cpu.registers.clear_n();
        cpu.registers.clear_z();
        cpu.registers.clear_v();
        cpu.registers.clear_b();
        cpu.registers.clear_d();
        cpu.registers.clear_i();

        let cycles = pushpop::php(&mut cpu, &mut bus);

        assert_eq!(3, cycles);

        let val = cpu.peek8(&mut bus);
        assert_eq!(val, 0x20);

        let sp = cpu.registers.sp;
        assert_eq!(sp, old_sp - 1);
    }

    #[test]
    fn test_php_alt() {
        let (mut cpu, mut bus) = setup_tests();
        let old_sp = cpu.registers.sp;

        cpu.registers.set_n();
        cpu.registers.clear_v();
        cpu.registers.clear_b();
        cpu.registers.set_d();
        cpu.registers.clear_i();
        cpu.registers.set_z();
        cpu.registers.clear_c();

        let cycles = pushpop::php(&mut cpu, &mut bus);

        assert_eq!(3, cycles);

        let val = cpu.peek8(&mut bus);
        assert_eq!(val, 0xAA);

        let sp = cpu.registers.sp;
        assert_eq!(sp, old_sp - 1);
    }

    #[test]
    fn test_pla() {
        let (mut cpu, mut bus) = setup_tests();
        let old_sp = cpu.registers.sp;

        cpu.push8(&mut bus, 0xAB);

        let cycles = pushpop::pla(&mut cpu, &mut bus);

        assert_eq!(4, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);

        let sp = cpu.registers.sp;
        assert_eq!(sp, old_sp);
    }

    #[test]
    fn test_plp1() {
        let (mut cpu, mut bus) = setup_tests();
        let old_sp = cpu.registers.sp;

        cpu.push8(&mut bus, 0xFF);

        let cycles = pushpop::plp(&mut cpu, &mut bus);

        assert_eq!(4, cycles);

        assert!(cpu.registers.get_c());
        assert!(cpu.registers.get_n());
        assert!(cpu.registers.get_z());
        assert!(cpu.registers.get_v());
        assert!(cpu.registers.get_b());
        assert!(cpu.registers.get_d());
        assert!(cpu.registers.get_i());

        let sp = cpu.registers.sp;
        assert_eq!(sp, old_sp);
    }

    #[test]
    fn test_plp0() {
        let (mut cpu, mut bus) = setup_tests();
        let old_sp = cpu.registers.sp;

        cpu.push8(&mut bus, 0x00);

        let cycles = pushpop::plp(&mut cpu, &mut bus);

        assert_eq!(4, cycles);

        assert!(!cpu.registers.get_c());
        assert!(!cpu.registers.get_n());
        assert!(!cpu.registers.get_z());
        assert!(!cpu.registers.get_v());
        assert!(!cpu.registers.get_b());
        assert!(!cpu.registers.get_d());
        assert!(!cpu.registers.get_i());

        let sp = cpu.registers.sp;
        assert_eq!(sp, old_sp);
    }

    #[test]
    fn test_plp_alt() {
        let (mut cpu, mut bus) = setup_tests();
        let old_sp = cpu.registers.sp;

        cpu.push8(&mut bus, 0xAA);

        let cycles = pushpop::plp(&mut cpu, &mut bus);

        assert_eq!(4, cycles);

        assert!(cpu.registers.get_n());
        assert!(!cpu.registers.get_v());
        assert!(!cpu.registers.get_b());
        assert!(cpu.registers.get_d());
        assert!(!cpu.registers.get_i());
        assert!(cpu.registers.get_z());
        assert!(!cpu.registers.get_c());

        let sp = cpu.registers.sp;
        assert_eq!(sp, old_sp);
    }
}
