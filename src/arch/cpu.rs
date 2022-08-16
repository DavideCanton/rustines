use log::info;

use crate::{
    arch::{
        instrs::instr_table::{get_fname_for_print, Instr, INSTR_TABLE},
        memory::Memory,
        registers::*,
    },
    hex,
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
        init_flags();

        Cpu {
            clock: 0,
            registers: Registers::new(),
            memory: mem,
            irq: false,
            nmi: false,
            rst: false,
        }
    }

    pub fn execute_verbose(&mut self) {
        // load pc from reset vector
        self.perform_rst();
        let mut cycles_tot = 0;

        loop {
            // fetch
            let opcode = self.memory.fetch(self.registers.pc);

            let Instr {
                ref fun,
                ref fname,
                ilen,
            } = INSTR_TABLE[opcode as usize];

            let mut data = Vec::with_capacity(ilen);
            for i in 0..ilen {
                data.push(self.memory.fetch(self.registers.pc + (i as u16)));
            }

            let old_pc = self.registers.pc;

            // execute
            let (cycles, actual_ilen) = fun(self);

            if cycles == 0xFF {
                break;
            }
            cycles_tot += cycles;

            info!(
                "[{}] {: <20}A:{} X:{} Y:{} P:{} SP:{} {}",
                hex!(old_pc),
                get_fname_for_print(fname, &data),
                hex!(self.registers.a_reg),
                hex!(self.registers.x_reg),
                hex!(self.registers.y_reg),
                hex!(self.registers.get_p()),
                hex!(self.registers.sp),
                cycles_tot
            );

            self.registers.pc += actual_ilen as u16;
        }
    }

    pub fn execute(&mut self) {
        // load pc from reset vector
        self.perform_rst();

        loop {
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
}
