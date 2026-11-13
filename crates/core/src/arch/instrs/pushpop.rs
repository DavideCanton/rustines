use crate::arch::{bus::Bus, cpu::Cpu};

pub fn pha(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.burn_internal_cycle(bus);
    let a = cpu.registers.a_reg;
    cpu.push8(bus, a);
    3
}

pub fn php(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.burn_internal_cycle(bus);
    let p = cpu.registers.get_p_force_b(true);
    cpu.push8(bus, p);
    3
}

pub fn pla(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.burn_internal_cycle(bus);
    let a = cpu.pop8(bus);
    cpu.burn_internal_cycle(bus);
    cpu.registers.a_reg = a;
    cpu.registers.update_nz_flags(a);
    4
}

pub fn plp(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.burn_internal_cycle(bus);
    let p = cpu.pop8(bus);
    cpu.burn_internal_cycle(bus);
    cpu.registers.set_p(p);
    4
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arch::arch_tests::setup_tests;

    #[test]
    fn test_pha() {
        let (mut cpu, mut bus) = setup_tests();

        cpu.registers.a_reg = 0xAB;
        let old_sp = cpu.registers.sp;

        let cycles = pha(&mut cpu, &mut bus);

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

        cpu.registers.set_c(true);
        cpu.registers.set_n(true);
        cpu.registers.set_z(true);
        cpu.registers.set_v(true);
        cpu.registers.set_d(true);
        cpu.registers.set_i(true);

        let cycles = php(&mut cpu, &mut bus);

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

        cpu.registers.set_c(false);
        cpu.registers.set_n(false);
        cpu.registers.set_z(false);
        cpu.registers.set_v(false);
        cpu.registers.set_d(false);
        cpu.registers.set_i(false);

        let cycles = php(&mut cpu, &mut bus);

        assert_eq!(3, cycles);

        let val = cpu.peek8(&mut bus);
        assert_eq!(val, 0x30);

        let sp = cpu.registers.sp;
        assert_eq!(sp, old_sp - 1);
    }

    #[test]
    fn test_php_alt() {
        let (mut cpu, mut bus) = setup_tests();
        let old_sp = cpu.registers.sp;

        cpu.registers.set_n(true);
        cpu.registers.set_v(false);
        cpu.registers.set_d(true);
        cpu.registers.set_i(false);
        cpu.registers.set_z(true);
        cpu.registers.set_c(false);

        let cycles = php(&mut cpu, &mut bus);

        assert_eq!(3, cycles);

        let val = cpu.peek8(&mut bus);
        assert_eq!(val, 0xBA);

        let sp = cpu.registers.sp;
        assert_eq!(sp, old_sp - 1);
    }

    #[test]
    fn test_pla() {
        let (mut cpu, mut bus) = setup_tests();
        let old_sp = cpu.registers.sp;

        cpu.push8(&mut bus, 0xAB);

        let cycles = pla(&mut cpu, &mut bus);

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

        let cycles = plp(&mut cpu, &mut bus);

        assert_eq!(4, cycles);

        assert!(cpu.registers.get_c());
        assert!(cpu.registers.get_n());
        assert!(cpu.registers.get_z());
        assert!(cpu.registers.get_v());
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

        let cycles = plp(&mut cpu, &mut bus);

        assert_eq!(4, cycles);

        assert!(!cpu.registers.get_c());
        assert!(!cpu.registers.get_n());
        assert!(!cpu.registers.get_z());
        assert!(!cpu.registers.get_v());
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

        let cycles = plp(&mut cpu, &mut bus);

        assert_eq!(4, cycles);

        assert!(cpu.registers.get_n());
        assert!(!cpu.registers.get_v());
        assert!(cpu.registers.get_d());
        assert!(!cpu.registers.get_i());
        assert!(cpu.registers.get_z());
        assert!(!cpu.registers.get_c());

        let sp = cpu.registers.sp;
        assert_eq!(sp, old_sp);
    }
}
