use paste::paste;
use std::fmt;

use crate::utils::bit_utils::{BitIndex, extract_flag, set_flag};

pub struct Registers {
    pub pc: u16,
    pub sp: u8,
    pub a_reg: u8,
    pub x_reg: u8,
    pub y_reg: u8,
    p_reg: u8,
}

pub const C_INDEX: u8 = 0;
pub const Z_INDEX: u8 = 1;
pub const I_INDEX: u8 = 2;
pub const D_INDEX: u8 = 3;
pub const B_INDEX: u8 = 4;
pub const U_INDEX: u8 = 5;
pub const V_INDEX: u8 = 6;
pub const N_INDEX: u8 = 7;

macro_rules! gen_methods {
    ($name: ident, $mask: expr) => {
        paste! {
            pub fn [<get_ $name>](&self) -> bool {
                extract_flag(self.p_reg, BitIndex::new($mask))
            }

            pub fn [<set_ $name _from_bool>](&mut self, val: bool) {
                self.p_reg = set_flag(self.p_reg, BitIndex::new($mask), val);
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
        Registers {
            pc: 0,
            sp: 0xFF,
            a_reg: 0,
            x_reg: 0,
            y_reg: 0,
            p_reg: 1 << U_INDEX,
        }
    }

    pub fn update_nz_flags(&mut self, value: u8) {
        self.set_n_from_bool(extract_flag(value, BitIndex::BIT_7));
        self.set_z_from_bool(value == 0);
    }

    pub fn get_p(&self, force_b: bool) -> u8 {
        let mut p = self.p_reg;
        if force_b {
            p = set_flag(p, BitIndex::new(B_INDEX), true);
        }
        p
    }

    pub fn set_p(&mut self, p: u8) {
        let old_b = extract_flag(self.p_reg, BitIndex::new(B_INDEX));
        self.p_reg = p | (1 << U_INDEX);
        self.p_reg = set_flag(self.p_reg, BitIndex::new(B_INDEX), old_b);
    }

    pub fn p_to_str(&self) -> String {
        let mut buf = String::with_capacity(8);

        for (mut letter, value) in "NV1BDIZC".chars().zip([
            self.get_n(),
            self.get_v(),
            true,
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

    gen_methods!(z, Z_INDEX);
    gen_methods!(n, N_INDEX);
    gen_methods!(v, V_INDEX);
    gen_methods!(c, C_INDEX);
    gen_methods!(b, B_INDEX);
    gen_methods!(d, D_INDEX);
    gen_methods!(i, I_INDEX);
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
            .field("p_reg", &format!("{:#04x}", self.p_reg))
            .field("z_flag", &self.get_z())
            .field("n_flag", &self.get_n())
            .field("v_flag", &self.get_v())
            .field("c_flag", &self.get_c())
            .field("b_flag", &self.get_b())
            .field("d_flag", &self.get_d())
            .field("i_flag", &self.get_i())
            .finish()
    }
}
