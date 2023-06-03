#[cfg(test)]
mod tests {
    use crate::arch::registers::Registers;

    #[test]
    fn test_num_n() {
        let mut reg = Registers::new();
        assert!(!reg.get_n());

        reg.compute_nz_flags(0xFF);
        assert!(reg.get_n());

        reg.compute_nz_flags(0x01);
        assert!(!reg.get_n());
    }

    #[test]
    fn test_num_z() {
        let mut reg = Registers::new();
        assert!(!reg.get_z());

        reg.compute_nz_flags(0x0);
        assert!(reg.get_z());

        reg.compute_nz_flags(0x01);
        assert!(!reg.get_z());
    }

    #[test]
    fn test_z() {
        _run_test(
            &mut Registers::new(),
            Registers::get_z,
            Registers::set_z,
            Registers::clear_z,
            Registers::set_z_from_bool,
        );
    }

    #[test]
    fn test_n() {
        _run_test(
            &mut Registers::new(),
            Registers::get_n,
            Registers::set_n,
            Registers::clear_n,
            Registers::set_n_from_bool,
        );
    }

    #[test]
    fn test_c() {
        _run_test(
            &mut Registers::new(),
            Registers::get_c,
            Registers::set_c,
            Registers::clear_c,
            Registers::set_c_from_bool,
        );
    }

    #[test]
    fn test_v() {
        _run_test(
            &mut Registers::new(),
            Registers::get_v,
            Registers::set_v,
            Registers::clear_v,
            Registers::set_v_from_bool,
        );
    }

    #[test]
    fn test_b() {
        _run_test(
            &mut Registers::new(),
            Registers::get_b,
            Registers::set_b,
            Registers::clear_b,
            Registers::set_b_from_bool,
        );
    }

    #[test]
    fn test_i() {
        _run_test(
            &mut Registers::new(),
            Registers::get_i,
            Registers::set_i,
            Registers::clear_i,
            Registers::set_i_from_bool,
        );
    }

    #[test]
    fn test_d() {
        _run_test(
            &mut Registers::new(),
            Registers::get_d,
            Registers::set_d,
            Registers::clear_d,
            Registers::set_d_from_bool,
        );
    }

    fn _run_test<
        G: Fn(&Registers) -> bool,
        S: Fn(&mut Registers),
        C: Fn(&mut Registers),
        SF: Fn(&mut Registers, bool),
    >(
        reg: &mut Registers,
        get: G,
        set: S,
        clear: C,
        set_from: SF,
    ) {
        assert!(!get(reg));
        set(reg);
        assert!(get(reg));
        clear(reg);
        assert!(!get(reg));
        set_from(reg, true);
        assert!(get(reg));
        set_from(reg, false);
        assert!(!get(reg));
    }
}
