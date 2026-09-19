use bitfield::bitfield;
use paste::paste;
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
    ($name: ident) => {
        paste! {
            pub fn [<get_ $name>](&self) -> bool {
                self.p_reg.[<get_ $name>]()
            }

            pub fn [<set_ $name _from_bool>](&mut self, val: bool) {
                self.p_reg.[<set_ $name>](val);
            }

            pub fn [<set_ $name>](&mut self) {
                self.[<set_ $name _from_bool>](true);
            }

            pub fn [<clear_ $name>](&mut self) {
                self.[<set_ $name _from_bool>](false);
            }
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
        self.set_n_from_bool(extract_flag(value, BitIndex::_7));
        self.set_z_from_bool(value == 0);
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

    gen_methods!(z);
    gen_methods!(n);
    gen_methods!(v);
    gen_methods!(c);
    gen_methods!(d);
    gen_methods!(i);
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
