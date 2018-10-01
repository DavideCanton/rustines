#[derive(Debug)]
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
}

pub const FLAG_C: u8 = 1;
pub const FLAG_Z: u8 = 1 << 1;
pub const FLAG_I: u8 = 1 << 2;
pub const FLAG_D: u8 = 1 << 3;
pub const FLAG_B: u8 = 1 << 4;
pub const FLAG_V: u8 = 1 << 6;
pub const FLAG_N: u8 = 1 << 7;

static mut NZ_TABLE: [u8; 1 << 8] = [0; 1 << 8];

pub fn init_flags() {
    unsafe {
        for i in 0u8..=255 {
            NZ_TABLE[i as usize] = (((i & 0x80 != 0) as u8) << 1) | ((i == 0) as u8);
            // println!("{} => nz:{:?}", i, NZ_TABLE[i as usize]);
        }
    }
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            pc: 0,
            sp: 0xFF,
            a_reg: 0,
            x_reg: 0,
            y_reg: 0,
            nz: 0,
            vc: 0,
            bdi: 0,
        }
        // reg.P |= 1 << 5; // the unused flag is always 1?
    }

    pub fn compute_nz_flags(&mut self, a: u8) {
        unsafe {
            self.nz = NZ_TABLE[a as usize];
        }
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

    pub fn get_n(&self) -> bool {
        (self.nz & 0x2) != 0
    }

    pub fn set_n(&mut self) {
        self.nz |= 0x2;
    }

    pub fn clear_n(&mut self) {
        self.nz &= 0xFD;
    }

    pub fn get_z(&self) -> bool {
        (self.nz & 0x1) != 0
    }

    pub fn set_z(&mut self) {
        self.nz |= 0x1;
    }

    pub fn clear_z(&mut self) {
        self.nz &= 0xFE;
    }

    pub fn get_v(&self) -> bool {
        (self.vc & 0x2) != 0
    }

    pub fn set_v(&mut self) {
        self.vc |= 0x2;
    }

    pub fn clear_v(&mut self) {
        self.vc &= 0xFD;
    }

    pub fn get_c(&self) -> bool {
        (self.vc & 0x1) != 0
    }

    pub fn set_c(&mut self) {
        self.vc |= 0x1;
    }

    pub fn clear_c(&mut self) {
        self.vc &= 0xFE;
    }

    pub fn get_b(&self) -> bool {
        (self.bdi & 0x4) != 0
    }

    pub fn set_b(&mut self) {
        self.bdi |= 0x4;
    }

    pub fn clear_b(&mut self) {
        self.nz &= 0xFB;
    }

    pub fn get_d(&self) -> bool {
        (self.bdi & 0x2) != 0
    }

    pub fn set_d(&mut self) {
        self.bdi |= 0x2;
    }

    pub fn clear_d(&mut self) {
        self.bdi &= 0xFB;
    }

    pub fn get_i(&self) -> bool {
        (self.bdi & 0x1) != 0
    }

    pub fn set_i(&mut self) {
        self.bdi |= 0x1;
    }

    pub fn clear_i(&mut self) {
        self.bdi &= 0xFE;
    }
}
