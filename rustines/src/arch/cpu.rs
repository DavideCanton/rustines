use log::info;

use crate::{
    arch::{
        instrs::instr_table::{get_fname_for_print, Instr, INSTR_TABLE},
        memory::Memory,
        registers::*,
    },
    bin, hex,
    utils::bit_utils::*,
};

pub struct Cpu {
    pub clock: u8,
    pub registers: Registers,
    pub memory: Memory,
    pub irq: bool,
    pub nmi: bool,
    pub rst: bool,
}

impl Cpu {
    pub fn new(mem: Memory) -> Self {
        Cpu {
            clock: 0,
            registers: Registers::new(),
            memory: mem,
            irq: false,
            nmi: false,
            rst: true,
        }
    }

    pub fn execute_verbose(&mut self) {
        let mut cycles_tot = 0;

        loop {
            self.handle_interrupts();

            // fetch
            let opcode = self.memory.fetch(self.registers.pc);

            let instr = &INSTR_TABLE[opcode as usize];
            let Instr { fun, ilen, .. } = instr;

            let data = self.memory.fetch_many(self.registers.pc, *ilen as u16);

            let p = self.registers.get_p();
            info!(
                "[{}] {: <20}A:{} X:{} Y:{} P:{} ({}) SP:{} {}",
                hex!(self.registers.pc),
                get_fname_for_print(instr, &data),
                hex!(self.registers.a_reg),
                hex!(self.registers.x_reg),
                hex!(self.registers.y_reg),
                hex!(p),
                bin!(p),
                hex!(self.registers.sp),
                cycles_tot
            );

            // execute
            let (cycles, actual_ilen) = fun(self);

            if cycles == 0xFF {
                break;
            }
            cycles_tot += cycles;

            self.registers.pc += actual_ilen as u16;
        }
    }

    pub fn execute(&mut self) {
        loop {
            if self.rst {
                self.perform_rst();
            }

            // fetch
            let opcode = self.memory.fetch(self.registers.pc);

            let instr = &INSTR_TABLE[opcode as usize];

            // execute
            let (cycles, actual_ilen) = (*instr.fun)(self);

            if cycles == 0xFF {
                break;
            }

            self.registers.pc += actual_ilen as u16;
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

    // decode functions

    pub fn decode_absolute(&self) -> (u16, u8) {
        let low = self.memory.fetch(self.registers.pc + 1);
        let high = self.memory.fetch(self.registers.pc + 2);
        (to_u16(low, high), 3)
    }

    pub fn decode_immediate(&self) -> (u8, u8) {
        (self.memory.fetch(self.registers.pc + 1), 2)
    }

    pub fn decode_zeropage(&self) -> (u8, u8) {
        (self.memory.fetch(self.registers.pc + 1), 2)
    }

    pub fn decode_absolute_indexed(&self, offset: u8) -> (u16, u8, u8) {
        let low = self.memory.fetch(self.registers.pc + 1);
        let high = self.memory.fetch(self.registers.pc + 2);
        (
            to_u16(low, high).wrapping_add(offset as u16),
            3,
            overflow_page_boundary(low, offset),
        )
    }

    pub fn decode_zeropage_indexed(&self, offset: u8) -> (u8, u8) {
        let addr = self.memory.fetch(self.registers.pc + 1);
        (addr.wrapping_add(offset), 2)
    }

    pub fn decode_indexed_indirect(&self) -> (u16, u8) {
        let op = (self
            .memory
            .fetch(self.registers.pc + 1)
            .wrapping_add(self.registers.x_reg)) as u16
            & 0xFF;
        let low = self.memory.fetch(op);
        let high = self.memory.fetch((op + 1) & 0xFF);

        (to_u16(low, high), 2)
    }

    pub fn decode_indirect_indexed(&self) -> (u16, u8, u8) {
        let op = self.memory.fetch(self.registers.pc + 1) as u16;
        let low = self.memory.fetch(op);
        let high = self.memory.fetch((op + 1) & 0xFF);
        let offset = self.registers.y_reg;

        (
            to_u16(low, high).wrapping_add(offset as u16),
            2,
            overflow_page_boundary(low, offset),
        )
    }

    fn save_state_before_interrupt(&mut self) {
        let pc = self.registers.pc;
        self.push16(pc);
        let p = self.registers.get_p();
        self.push8(p);
    }

    fn perform_irq(&mut self) {
        self.save_state_before_interrupt();

        let low = self.memory.fetch(0xFFFE);
        let high = self.memory.fetch(0xFFFF);

        self.registers.pc = to_u16(low, high);
    }

    fn perform_nmi(&mut self) {
        self.save_state_before_interrupt();

        let low = self.memory.fetch(0xFFFA);
        let high = self.memory.fetch(0xFFFB);

        self.registers.pc = to_u16(low, high);
    }

    fn perform_rst(&mut self) {
        let low = self.memory.fetch(0xFFFC);
        let high = self.memory.fetch(0xFFFD);

        self.registers.pc = to_u16(low, high);
    }

    fn handle_interrupts(&mut self) {
        // TODO verify priority
        if self.nmi {
            self.perform_nmi();
        } else if self.irq {
            self.perform_irq();
        } else if self.rst {
            self.perform_rst();
        }

        self.irq = false;
        self.nmi = false;
        self.rst = false;
    }
}

fn overflow_page_boundary(low: u8, offset: u8) -> u8 {
    if low.overflowing_add(offset).1 {
        1
    } else {
        0
    }
}
