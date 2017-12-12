use arch::memory::Memory;
use arch::registers::*;
use arch::instrs::instr_table::{Instr, INSTR_TABLE};
use utils::bit_utils::*;

pub struct CPU {
    pub clock: u8,
    pub registers: Registers,
    pub memory: Memory,
    pub irq: bool,
    pub nmi: bool,
    pub rst: bool,
}

impl CPU {
    pub fn new(mem: Memory) -> Self {
        init_flags();

        CPU {
            clock: 0,
            registers: Registers::new(),
            memory: mem,
            irq: false,
            nmi: false,
            rst: false,
        }
    }

    pub fn execute(&mut self) {
        loop {
            // fetch
            let opcode = self.memory.fetch(self.registers.pc);

            let Instr{ ref fun, fname, ilen } = INSTR_TABLE[opcode as usize];

            println!("Fetched instr {}, pc = {}", fname, self.registers.pc);

            // execute
            let (cycles, _) = fun(self);

            if cycles == 0xFF {
                break;
            }

            self.registers.pc += ilen as u16;

            println!("Registers: {:?}", self.registers);

            // update time?

            println!("{}", cycles);
        }
    }

    pub fn push32(&mut self, v: u32) {
        let (low, high) = to_u16_lh(v);

        // TODO is the order right?
        self.push16(high);
        self.push16(low);
    }

    pub fn push16(&mut self, v: u16) {
        let (low, high) = to_u8_lh(v);

        // TODO is the order right?
        self.push8(high);
        self.push8(low);
    }

    pub fn push8(&mut self, v: u8) {
        self.memory.push8(self.registers.sp, v);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
    }

    pub fn pop8(&mut self) -> u8 {
        self.registers.sp = self.registers.sp.wrapping_add(1);
        self.memory.peek8(self.registers.sp)
    }

    pub fn pop16(&mut self) -> u16 {
        let low = self.pop8();
        let high = self.pop8();

        to_u16(low, high)
    }

    pub fn pop32(&mut self) -> u32 {
        let low = self.pop16();
        let high = self.pop16();

        to_u32(low, high)
    }

    pub fn peek8(&self) -> u8 {
        self.memory.peek8(self.registers.sp + 1)
    }

    pub fn peek16(&self) -> u16 {
        let low = self.peek8();
        let high = self.memory.fetch(self.registers.sp as u16 + 0x0102);

        to_u16(low, high)
    }

    pub fn peek32(&self) -> u32 {
        let low = self.peek16();
        let high_h = self.memory.fetch(self.registers.sp as u16 + 0x0103);
        let high_l = self.memory.fetch(self.registers.sp as u16 + 0x0104);
        let high = to_u16(high_l, high_h);

        to_u32(low, high)
    }
}
