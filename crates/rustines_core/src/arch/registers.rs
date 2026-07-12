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

    pub fn get_p(&self, force_b: bool) -> u8 {
        // NZ
        // NZ & 2 -> N0, N0 >> 1 -> N
        // NZ & 1 -> Z
        let (n, z) = (((self.nz & 2) >> 1), self.nz & 1);
        // VC
        // VC & 2 -> V0, V0 >> 1 -> V
        // VC & 1 -> C
        let (v, c) = (((self.vc & 2) >> 1), self.vc & 1);
        // P = N | V | 1 | B | D | I | Z | C
        let mut ret = (n << 7) | (v << 6) | (1 << 5) | (self.bdi << 2) | (z << 1) | c;
        // force B to 1
        if force_b {
            ret |= 1 << 4;
        }
        ret
    }

    pub fn set_p(&mut self, p: u8, ignore_b: bool) {
        // P = N | V | 1 | B | D | I | Z | C
        // V0 = NV1BDIZC & (01000000) -> 0V000000 >> 5 -> 000000V0
        // C = NV1BDIZC & (0000001) -> 0000000C
        // VC = V0 | C
        self.vc = ((p & 0x40) >> 5) | (p & 0x1);
        // N0 = NV1BDIZC & (10000000) -> N0000000 >> 6 -> 000000N0
        // Z = NV1BDIZC & (00000010) -> 000000Z0 >> 1 -> 0000000Z
        // NZ = N0 | Z
        self.nz = ((p & 0x80) >> 6) | ((p & 0x2) >> 1);
        if ignore_b {
            // ignore B
            // T = (NV1BDIZC) >> 2 -> 00NV1BDI & (11) -> 000000DI
            // BDI = (BDI & (100)) | 000000DI
            self.bdi = (self.bdi & 0x4) | (p >> 2) & 0x3;
        } else {
            // T = (NV1BDIZC) >> 2 -> 00NV1BDI & (111) -> 00000BDI
            // BDI = 00000BDI
            self.bdi = (p >> 2) & 0x7;
        }
    }

    pub fn p_str(&self) -> String {
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

    gen_methods!(z, nz, 0x1);
    gen_methods!(n, nz, 0x2);
    gen_methods!(v, vc, 0x2);
    gen_methods!(c, vc, 0x1);
    gen_methods!(b, bdi, 0x4);
    gen_methods!(d, bdi, 0x2);
    gen_methods!(i, bdi, 0x1);
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
