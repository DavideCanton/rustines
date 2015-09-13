#[allow(unused_imports)]
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

#[test]
pub fn test_num_C()
{
    init_flags();
    let mut reg = Registers::new();
    assert!(!reg.getC());

    let mut a = 0x01;
    let mut b = 0x02;
    let mut r;

    r = (a as u16) + (b as u16);
    reg.compute_VC_flags(a, r);
    assert!(!reg.getC());

    a = 0xFF;
    b = 0x02;
    r = (a as u16) + (b as u16);
    reg.compute_VC_flags(a, r);
    assert!(reg.getC());

    a = 0x01;
    b = 0x02;
    r = (a as u16) + (b as u16);
    reg.compute_VC_flags(a, r);
    assert!(!reg.getC());
}

#[test]
pub fn test_C()
{
    init_flags();
    let mut reg = Registers::new();
    assert!(!reg.getC());

    reg.setC();
    assert!(reg.getC());

    reg.clearC();
    assert!(!reg.getC());
}
