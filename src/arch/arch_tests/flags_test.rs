use arch::registers::*;

#[test]
pub fn test_num_N()
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
pub fn test_N()
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
pub fn test_num_Z()
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
pub fn test_Z()
{
    init_flags();
    let mut reg = Registers::new();
    assert!(!reg.getZ());

    reg.setZ();
    assert!(reg.getZ());

    reg.clearZ();
    assert!(!reg.getZ());
}
