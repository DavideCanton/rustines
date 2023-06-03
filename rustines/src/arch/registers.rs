use paste::paste;
use std::fmt;

pub struct Registers {
    pub pc: u16,
    pub sp: u8,
    // pub P: u8,
    pub a_reg: u8, // o i8?
    pub x_reg: u8,
    pub y_reg: u8,

    // flags
    nz: u8,
    vc: u8,
    bdi: u8,

    // table
    nz_table: [u8; 1 << 8],
}

pub const FLAG_C: u8 = 1;
pub const FLAG_Z: u8 = 1 << 1;
pub const FLAG_I: u8 = 1 << 2;
pub const FLAG_D: u8 = 1 << 3;
pub const FLAG_B: u8 = 1 << 4;
pub const FLAG_V: u8 = 1 << 6;
pub const FLAG_N: u8 = 1 << 7;

macro_rules! gen_methods {
    ($name: ident, $field: ident, $mask: expr) => {
        paste! {
            pub fn [<get_ $name>](&self) -> bool {
                (self.$field & $mask) != 0
            }

            pub fn [<set_ $name>](&mut self) {
                self.$field |= $mask;
            }

            pub fn [<set_ $name _from_bool>](&mut self, val: bool) {
                if val {
                    self.[<set_ $name>]();
                } else {
                    self.[<clear_ $name>]();
                }
            }

            pub fn [<clear_ $name>](&mut self) {
                self.$field &= !$mask;
            }
        }
    };
}

impl Registers {
    pub fn new() -> Registers {
        let mut nz_table = [0; 1 << 8];
        for i in 0u8..=255 {
            nz_table[i as usize] = (((i & 0x80 != 0) as u8) << 1) | ((i == 0) as u8);
            // println!("{} => nz:{:?}", i, nz_table[i as usize]);
        }

        Registers {
            pc: 0,
            sp: 0xFF,
            a_reg: 0,
            x_reg: 0,
            y_reg: 0,
            nz: 0,
            vc: 0,
            bdi: 0,
            nz_table,
        }
        // reg.P |= 1 << 5; // the unused flag is always 1?
    }

    pub fn compute_nz_flags(&mut self, a: u8) {
        self.nz = self.nz_table[a as usize];
    }

    pub fn compute_vc_flags(&mut self, v: bool, c: bool) {
        self.vc = ((v as u8) << 1) | (c as u8);
    }

    pub fn compute_c_flag(&mut self, c: bool) {
        self.vc = (self.vc & 0x10) | (c as u8);
    }

    pub fn get_p(&self) -> u8 {
        let (n, z) = (((self.nz & 2) >> 1), self.nz & 1);
        let (v, c) = (((self.vc & 2) >> 1), self.vc & 1);

        (n << 7) | (v << 6) | (1 << 5) | (self.bdi << 2) | (z << 1) | c
    }

    pub fn set_p(&mut self, p: u8) {
        self.vc = ((p & 0x40) >> 5) | (p & 0x1);
        self.nz = ((p & 0x80) >> 6) | ((p & 0x2) >> 1);
        self.bdi = (p >> 2) & 0x7;
    }

    gen_methods!(z, nz, 0x1);
    gen_methods!(n, nz, 0x2);
    gen_methods!(v, vc, 0x2);
    gen_methods!(c, vc, 0x1);
    gen_methods!(b, bdi, 0x4);
    gen_methods!(d, bdi, 0x2);
    gen_methods!(i, bdi, 0x1);
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
