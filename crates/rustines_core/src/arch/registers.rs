use bitfield::bitfield;
use std::fmt;

use crate::utils::bit_utils::{BitIndex, extract_flag};

bitfield! {
    #[derive(Clone, Copy)]
    #[repr(C)]
    struct P_Register(u8);
    impl Debug;
    u8;
    pub get_n, set_n: 7;
    pub get_v, set_v: 6;
    pub get_u, set_u: 5;
    pub get_b, set_b: 4;
    pub get_d, set_d: 3;
    pub get_i, set_i: 2;
    pub get_z, set_z: 1;
    pub get_c, set_c: 0;
}

impl P_Register {
    fn stringify(&self) -> String {
        let mut buf = String::with_capacity(8);

        for (mut letter, value) in "NVUBDIZC".chars().zip([
            self.get_n(),
            self.get_v(),
            self.get_u(),
            self.get_b(),
            self.get_d(),
            self.get_i(),
            self.get_z(),
            self.get_c(),
        ]) {
            if !value {
                letter = letter.to_lowercase().next().unwrap();
            }
            buf.push(letter);
        }

        buf
    }
}

pub struct Registers {
    pub pc: u16,
    pub sp: u8,
    pub a_reg: u8,
    pub x_reg: u8,
    pub y_reg: u8,
    p_reg: P_Register,
}

macro_rules! gen_methods {
    ($getter:ident, $setter:ident) => {
        pub fn $getter(&self) -> bool {
            self.p_reg.$getter()
        }

        pub fn $setter(&mut self, val: bool) {
            self.p_reg.$setter(val);
        }
    };
}

impl Registers {
    pub fn new() -> Registers {
        let mut p_reg = P_Register(0);
        p_reg.set_u(true);

        Registers {
            pc: 0,
            sp: 0xFF,
            a_reg: 0,
            x_reg: 0,
            y_reg: 0,
            p_reg,
        }
    }

    pub fn update_nz_flags(&mut self, value: u8) {
        self.set_n(extract_flag(value, BitIndex::_7));
        self.set_z(value == 0);
    }

    pub fn get_p(&self) -> u8 {
        self.p_reg.0
    }

    pub fn get_p_force_b(&self, value: bool) -> u8 {
        let mut p = self.p_reg;
        p.set_b(value);
        p.0
    }

    pub fn set_p(&mut self, p: u8) {
        let mut p_reg = P_Register(p);
        p_reg.set_b(false);
        p_reg.set_u(true);
        self.p_reg = p_reg;
    }

    pub fn p_to_str(&self) -> String {
        self.p_reg.stringify()
    }

    gen_methods!(get_z, set_z);
    gen_methods!(get_n, set_n);
    gen_methods!(get_v, set_v);
    gen_methods!(get_c, set_c);
    gen_methods!(get_d, set_d);
    gen_methods!(get_i, set_i);
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Registers")
            .field("pc", &format!("{:#04x}", self.pc))
            .field("sp", &format!("{:#04x}", self.sp))
            .field("a_reg", &format!("{:#04x}", self.a_reg))
            .field("x_reg", &format!("{:#04x}", self.x_reg))
            .field("y_reg", &format!("{:#04x}", self.y_reg))
            .field("p_reg", &format!("{:?}", self.p_reg))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::Registers;

    #[test]
    fn test_num_n() {
        let mut reg = Registers::default();
        assert!(!reg.get_n());

        reg.update_nz_flags(0xFF);
        assert!(reg.get_n());

        reg.update_nz_flags(0x01);
        assert!(!reg.get_n());
    }

    #[test]
    fn test_num_z() {
        let mut reg = Registers::default();
        assert!(!reg.get_z());

        reg.update_nz_flags(0x0);
        assert!(reg.get_z());

        reg.update_nz_flags(0x01);
        assert!(!reg.get_z());
    }

    #[test]
    fn test_z() {
        _run_test(
            &mut Registers::default(),
            Registers::get_z,
            Registers::set_z,
        );
    }

    #[test]
    fn test_n() {
        _run_test(
            &mut Registers::default(),
            Registers::get_n,
            Registers::set_n,
        );
    }

    #[test]
    fn test_c() {
        _run_test(
            &mut Registers::default(),
            Registers::get_c,
            Registers::set_c,
        );
    }

    #[test]
    fn test_v() {
        _run_test(
            &mut Registers::default(),
            Registers::get_v,
            Registers::set_v,
        );
    }

    #[test]
    fn test_i() {
        _run_test(
            &mut Registers::default(),
            Registers::get_i,
            Registers::set_i,
        );
    }

    #[test]
    fn test_d() {
        _run_test(
            &mut Registers::default(),
            Registers::get_d,
            Registers::set_d,
        );
    }

    #[test]
    fn test_b_is_synthesized_when_exporting_p() {
        let mut registers = Registers::default();
        registers.set_c(true);

        assert_eq!(registers.get_p(), 0x21);
        assert_eq!(registers.get_p_force_b(true), 0x31);
        assert_eq!(registers.get_p_force_b(false), 0x21);
        assert_eq!(registers.get_p(), 0x21);
    }

    #[test]
    fn test_b_is_not_persisted_when_importing_p() {
        let mut registers = Registers::default();
        registers.set_p(0xFF);

        assert_eq!(registers.get_p(), 0xEF);
        assert_eq!(registers.get_p_force_b(true), 0xFF);
    }

    fn _run_test(
        reg: &mut Registers,
        get: impl Fn(&Registers) -> bool,
        set: impl Fn(&mut Registers, bool),
    ) {
        assert!(!get(reg));
        set(reg, true);
        assert!(get(reg));
        set(reg, false);
        assert!(!get(reg));
    }
}
