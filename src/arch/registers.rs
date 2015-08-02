pub struct Registers
{
    pub PC: u16,
    pub SP: u8,
    pub P: u8,
    pub A: u8, // o i8?
    pub X: u8,
    pub Y: u8,
}

pub const FLAG_C: u8 = 1 << 0;
pub const FLAG_Z: u8 = 1 << 1;
pub const FLAG_I: u8 = 1 << 2;
pub const FLAG_D: u8 = 1 << 3;
pub const FLAG_B: u8 = 1 << 4;
pub const FLAG_V: u8 = 1 << 6;
pub const FLAG_N: u8 = 1 << 7;


impl Registers
{
    pub fn new() -> Registers
    {
        let mut reg = Registers { PC: 0, SP: 0xFF, P: 0, A: 0, X: 0, Y: 0 };
        reg.P |= 1 << 5; // the unused flag is always 1?
        reg
    }
}
