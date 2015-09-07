use arch::bit_utils::*;

pub struct Registers
{
    pub PC: u16,
    pub SP: u8,
    //pub P: u8,
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

static mut NZ_TABLE : [u8; 1 << 8] = [0; 1 << 8];
static mut VC_TABLE : [u8; 1 << 16] = [0; 1 << 16];

pub fn init_flags()
{
    unsafe
    {
        for i in 0u16..256
        {
            NZ_TABLE[i as usize] = (((i & 0x80 != 0) as u8) << 1) | ((i == 0) as u8);
            //println!("{} => NZ:{:?}", i, NZ_TABLE[i as usize]);

            for j in 0u16..256
            {
                let R = i + j;
                let C = (R & 0x100 != 0) as u8;
                let V = ((i >> 7) == (j >> 7) && (i >> 7) != ((R >> 7) & 1)) as u8;

                VC_TABLE[to_u16(j as u8, i as u8) as usize] = (V | C) as u8;

                //println!("{} {} => VC:{:?}", i, j, VC_TABLE[to_u16(j as u8, i as u8) as usize]);
            }
        }
    }
}

impl Registers
{
    pub fn new() -> Registers
    {
        Registers { PC: 0, SP: 0xFF, A: 0, X: 0, Y: 0, NZ: 0, VC: 0, BDI: 0 }
        // reg.P |= 1 << 5; // the unused flag is always 1?
    }

    pub fn compute_NZ_flags(&mut self, a: u8)
    {
        unsafe
        {
            self.NZ = NZ_TABLE[a as usize];
        }
    }

    pub fn compute_VC_flags(&mut self, a: u8, b: u8)
    {
        let ind = to_u16(b, a);
        unsafe
        {
            self.VC = VC_TABLE[ind as usize];
        }
    }

    pub fn getP(&self) -> u8
    {
        let (N, Z) = (((self.NZ & 2) >> 1), self.NZ & 1);
        let (V, C) = (((self.VC & 2) >> 1), self.VC & 1);

        (N << 7) | (V << 6) | (1 << 5) | (self.BDI << 2) | (Z << 1) | C
    }

    pub fn getN(&self) -> bool
    {
        (self.NZ & 0x2) != 0
    }

    pub fn setN(&mut self)
    {
        self.NZ |= 0x2;
    }

    pub fn clearN(&mut self)
    {
        self.NZ &= 0xFD;
    }

    pub fn getZ(&self) -> bool
    {
        (self.NZ & 0x1) != 0
    }

    pub fn setZ(&mut self)
    {
        self.NZ |= 0x1;
    }

    pub fn clearZ(&mut self)
    {
        self.NZ &= 0xFE;
    }

    pub fn getV(&self) -> bool
    {
        (self.VC & 0x2) != 0
    }

    pub fn setV(&mut self)
    {
        self.VC |= 0x2;
    }

    pub fn clearV(&mut self)
    {
        self.VC &= 0xFD;
    }

    pub fn getC(&self) -> bool
    {
        (self.VC & 0x1) != 0
    }

    pub fn setC(&mut self)
    {
        self.VC |= 0x1;
    }

    pub fn clearC(&mut self)
    {
        self.VC &= 0xFE;
    }

    pub fn getB(&self) -> bool
    {
        (self.BDI & 0x4) != 0
    }

    pub fn setB(&mut self)
    {
        self.BDI |= 0x4;
    }

    pub fn clearB(&mut self)
    {
        self.NZ &= 0xFB;
    }

    pub fn getD(&self) -> bool
    {
        (self.BDI & 0x2) != 0
    }

    pub fn setD(&mut self)
    {
        self.BDI |= 0x2;
    }

    pub fn clearD(&mut self)
    {
        self.BDI &= 0xFB;
    }

    pub fn getI(&self) -> bool
    {
        (self.BDI & 0x1) != 0
    }

    pub fn setI(&mut self)
    {
        self.BDI |= 0x1;
    }

    pub fn clearI(&mut self)
    {
        self.BDI &= 0xFE;
    }
}
