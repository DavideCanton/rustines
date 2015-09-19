#[cfg(test)]
mod tests
{
    use arch::registers::{Registers, init_flags};

    #[test]
    fn test_num_n()
    {
        init_flags();
        let mut reg = Registers::new();
        assert!(!reg.getN());

        reg.compute_NZ_flags(0xFF);
        assert!(reg.getN());

        reg.compute_NZ_flags(0x01);
        assert!(!reg.getN());
    }

    #[test]
    fn test_n()
    {
        init_flags();
        let mut reg = Registers::new();
        assert!(!reg.getN());

        reg.setN();
        assert!(reg.getN());

        reg.clearN();
        assert!(!reg.getN());
    }

    #[test]
    fn test_num_z()
    {
        init_flags();
        let mut reg = Registers::new();
        assert!(!reg.getZ());

        reg.compute_NZ_flags(0x0);
        assert!(reg.getZ());

        reg.compute_NZ_flags(0x01);
        assert!(!reg.getZ());
    }

    #[test]
    fn test_z()
    {
        init_flags();
        let mut reg = Registers::new();
        assert!(!reg.getZ());

        reg.setZ();
        assert!(reg.getZ());

        reg.clearZ();
        assert!(!reg.getZ());
    }

    #[test]
    fn test_c()
    {
        init_flags();
        let mut reg = Registers::new();
        assert!(!reg.getC());

        reg.setC();
        assert!(reg.getC());

        reg.clearC();
        assert!(!reg.getC());
    }
}
