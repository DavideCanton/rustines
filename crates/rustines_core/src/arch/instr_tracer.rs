use log::{Level, log_enabled, trace};

use crate::{
    Bus,
    arch::{
        instrs::instr_table::{INSTR_TABLE, Instr},
        registers::Registers,
    },
    hex16,
};

pub struct InstructionTracer {
    tracing_enabled: bool,
    function_level: u32,
    previous_clock: u64,
}

impl InstructionTracer {
    pub fn new() -> Self {
        InstructionTracer {
            function_level: 0,
            tracing_enabled: false,
            previous_clock: 0,
        }
    }

    pub fn enable_tracing(&mut self, tracing_enabled: bool) {
        self.tracing_enabled = tracing_enabled;
    }

    pub fn trace_interrupt(
        &mut self,
        irq_type: &str,
        address: u16,
        registers: &Registers,
        clock: u64,
    ) {
        let old_clock = self.previous_clock;
        self.previous_clock = clock;

        if !log_enabled!(Level::Trace) || !self.tracing_enabled {
            return;
        }

        let msg = format!("{} -> ${}", irq_type, hex16!(address));
        self._trace(registers, clock, old_clock, msg);
    }

    pub fn trace_instruction(
        &mut self,
        registers: &Registers,
        clock: u64,
        bus: &Bus,
        instr: &Instr,
    ) {
        let old_clock = self.previous_clock;
        self.previous_clock = clock;

        if !log_enabled!(Level::Trace) || !self.tracing_enabled {
            return;
        }

        if instr.fname.contains("rts") {
            self.function_level -= 1;
        }

        let pc = registers.pc;
        let opcode = bus.peek(pc);
        let instr = &INSTR_TABLE[opcode as usize];

        let mut buf = vec![0; instr.ilen];

        let mut cur = pc;
        for pos in buf.iter_mut() {
            let val = bus.peek(cur);
            *pos = val;
            cur = cur.wrapping_add(1);
        }

        let instr_str = instr.get_fname_for_print(&buf);

        self._trace(registers, clock, old_clock, instr_str);

        if instr.fname.contains("jsr") {
            self.function_level += 1;
        }
    }

    fn _trace(&mut self, registers: &Registers, clock: u64, old_clock: u64, msg: String) {
        trace!(
            "TRACE CPU -> LEVEL: {:<2} | PC: {:#06X} | {:<20} | A: {:#04X} | X: {:#04X} | Y: {:#04X} | SP: {:#04X} | P: {} ({:#04X}) [{:10}](+{:>02})",
            self.function_level,
            registers.pc,
            msg,
            registers.a_reg,
            registers.x_reg,
            registers.y_reg,
            registers.sp,
            registers.p_to_str(),
            registers.get_p(),
            clock,
            clock - old_clock
        );
    }
}
