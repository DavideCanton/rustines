use utils::utils::range_inclusive;

#[derive(Debug)]
pub struct Registers {
    pub PC: u16,
    pub SP: u8,
    // pub P: u8,
    pub A: u8, // o i8?
    pub X: u8,
    pub Y: u8,

    // flags
    pub NZ: u8,
    pub VC: u8,
    pub BDI: u8,
}

pub const FLAG_C: u8 = 1 << 0;
pub const FLAG_Z: u8 = 1 << 1;
pub const FLAG_I: u8 = 1 << 2;
pub const FLAG_D: u8 = 1 << 3;
pub const FLAG_B: u8 = 1 << 4;
pub const FLAG_V: u8 = 1 << 6;
pub const FLAG_N: u8 = 1 << 7;

static mut NZ_TABLE: [u8; 1 << 8] = [0; 1 << 8];

pub fn init_flags() {
    unsafe {
        for i in range_inclusive(0u8, 255) {
            NZ_TABLE[i as usize] = (((i & 0x80 != 0) as u8) << 1) | ((i == 0) as u8);
            // println!("{} => NZ:{:?}", i, NZ_TABLE[i as usize]);
        }
    }
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            PC: 0,
            SP: 0xFF,
            A: 0,
            X: 0,
            Y: 0,
            NZ: 0,
            VC: 0,
            BDI: 0,
        }
        // reg.P |= 1 << 5; // the unused flag is always 1?
    }

    pub fn compute_NZ_flags(&mut self, a: u8) {
        unsafe {
            self.NZ = NZ_TABLE[a as usize];
        }
    }

    pub fn compute_VC_flags(&mut self, v: bool, c: bool) {
        self.VC = ((v as u8) << 1) | (c as u8);
    }

    pub fn compute_C_flag(&mut self, c: bool) {
        self.VC = (self.VC & 0x10) | (c as u8);
    }

    pub fn getP(&self) -> u8 {
        let (N, Z) = (((self.NZ & 2) >> 1), self.NZ & 1);
        let (V, C) = (((self.VC & 2) >> 1), self.VC & 1);

        (N << 7) | (V << 6) | (1 << 5) | (self.BDI << 2) | (Z << 1) | C
    }

    pub fn setP(&mut self, p: u8) {
        self.VC = ((p & 0x40) >> 5) | (p & 0x1);
        self.NZ = ((p & 0x80) >> 6) | ((p & 0x2) >> 1);
        self.BDI = (p >> 2) & 0x7;
    }

    pub fn getN(&self) -> bool {
        (self.NZ & 0x2) != 0
    }

    pub fn setN(&mut self) {
        self.NZ |= 0x2;
    }

    pub fn clearN(&mut self) {
        self.NZ &= 0xFD;
    }

    pub fn getZ(&self) -> bool {
        (self.NZ & 0x1) != 0
    }

    pub fn setZ(&mut self) {
        self.NZ |= 0x1;
    }

    pub fn clearZ(&mut self) {
        self.NZ &= 0xFE;
    }

    pub fn getV(&self) -> bool {
        (self.VC & 0x2) != 0
    }

    pub fn setV(&mut self) {
        self.VC |= 0x2;
    }

    pub fn clearV(&mut self) {
        self.VC &= 0xFD;
    }

    pub fn getC(&self) -> bool {
        (self.VC & 0x1) != 0
    }

    pub fn setC(&mut self) {
        self.VC |= 0x1;
    }

    pub fn clearC(&mut self) {
        self.VC &= 0xFE;
    }

    pub fn getB(&self) -> bool {
        (self.BDI & 0x4) != 0
    }

    pub fn setB(&mut self) {
        self.BDI |= 0x4;
    }

    pub fn clearB(&mut self) {
        self.NZ &= 0xFB;
    }

    pub fn getD(&self) -> bool {
        (self.BDI & 0x2) != 0
    }

    pub fn setD(&mut self) {
        self.BDI |= 0x2;
    }

    pub fn clearD(&mut self) {
        self.BDI &= 0xFB;
    }

    pub fn getI(&self) -> bool {
        (self.BDI & 0x1) != 0
    }

    pub fn setI(&mut self) {
        self.BDI |= 0x1;
    }

    pub fn clearI(&mut self) {
        self.BDI &= 0xFE;
    }
}
