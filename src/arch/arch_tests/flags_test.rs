#[cfg(test)]
mod tests {
    use arch::registers::{Registers, init_flags};

    #[test]
    fn test_num_n() {
        init_flags();
        let mut reg = Registers::new();
        assert!(!reg.get_n());

        reg.compute_nz_flags(0xFF);
        assert!(reg.get_n());

        reg.compute_nz_flags(0x01);
        assert!(!reg.get_n());
    }

    #[test]
    fn test_n() {
        init_flags();
        let mut reg = Registers::new();
        assert!(!reg.get_n());

        reg.set_n();
        assert!(reg.get_n());

        reg.clear_n();
        assert!(!reg.get_n());
    }

    #[test]
    fn test_num_z() {
        init_flags();
        let mut reg = Registers::new();
        assert!(!reg.get_z());

        reg.compute_nz_flags(0x0);
        assert!(reg.get_z());

        reg.compute_nz_flags(0x01);
        assert!(!reg.get_z());
    }

    #[test]
    fn test_z() {
        init_flags();
        let mut reg = Registers::new();
        assert!(!reg.get_z());

        reg.set_z();
        assert!(reg.get_z());

        reg.clear_z();
        assert!(!reg.get_z());
    }

    #[test]
    fn test_c() {
        init_flags();
        let mut reg = Registers::new();
        assert!(!reg.get_c());

        reg.set_c();
        assert!(reg.get_c());

        reg.clear_c();
        assert!(!reg.get_c());
    }
}
