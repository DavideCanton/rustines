use paste::paste;
use std::fmt;

pub struct Registers {
    pub pc: u16,
    pub sp: u8,
    pub a_reg: u8,
    pub x_reg: u8,
    pub y_reg: u8,
    p_reg: u8,
}

pub const FLAG_C: u8 = 1 << 0;
pub const FLAG_Z: u8 = 1 << 1;
pub const FLAG_I: u8 = 1 << 2;
pub const FLAG_D: u8 = 1 << 3;
pub const FLAG_B: u8 = 1 << 4;
pub const FLAG_U: u8 = 1 << 5;
pub const FLAG_V: u8 = 1 << 6;
pub const FLAG_N: u8 = 1 << 7;

macro_rules! gen_methods {
    ($name: ident, $mask: expr) => {
        paste! {
            pub fn [<get_ $name>](&self) -> bool {
                self.p_reg & $mask != 0
            }

            pub fn [<set_ $name>](&mut self) {
                self.p_reg |= $mask;
            }

            pub fn [<set_ $name _from_bool>](&mut self, val: bool) {
                if val {
                    self.[<set_ $name>]();
                } else {
                    self.[<clear_ $name>]();
                }
            }

            pub fn [<clear_ $name>](&mut self) {
                self.p_reg &= !$mask;
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
            p_reg: FLAG_U,
        }
    }

    pub fn update_nz_flags(&mut self, value: u8) {
        self.set_n_from_bool(value & FLAG_N != 0);
        self.set_z_from_bool(value == 0);
    }

    pub fn get_p(&self, force_b: bool) -> u8 {
        let mut p = self.p_reg;
        if force_b {
            p |= FLAG_B;
        }
        p
    }

    pub fn set_p(&mut self, p: u8) {
        let old_b = self.p_reg & FLAG_B;
        self.p_reg = p | FLAG_U;
        self.p_reg = (self.p_reg & !FLAG_B) | old_b;
    }

    pub fn p_to_str(&self) -> String {
        let mut s = String::with_capacity(8);

        for (letter, value) in "NV1BDIZC".chars().zip([
            self.get_n(),
            self.get_v(),
            true,
            self.get_b(),
            self.get_d(),
            self.get_i(),
            self.get_z(),
            self.get_c(),
        ]) {
            if value {
                s.push(letter);
            } else {
                s.push(letter.to_lowercase().next().unwrap());
            }
        }

        s
    }

    gen_methods!(z, FLAG_Z);
    gen_methods!(n, FLAG_N);
    gen_methods!(v, FLAG_V);
    gen_methods!(c, FLAG_C);
    gen_methods!(b, FLAG_B);
    gen_methods!(d, FLAG_D);
    gen_methods!(i, FLAG_I);
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
