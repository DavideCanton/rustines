use log::{Level, log_enabled, trace};

use crate::{
    arch::{bus::Bus, instrs::instr_table::INSTR_TABLE, registers::*},
    hex16,
    utils::bit_utils::*,
};

pub struct Cpu {
    pub registers: Registers,
    nmi: bool,
    rst: bool,
    trace: bool,
    clock: u64,
    function_level: u32,
    pending_irq_execution: bool,
    pending_nmi_execution: bool,
    pending_rst_execution: bool,
}

impl Cpu {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Cpu {
            registers: Registers::default(),
            nmi: false,
            rst: true,
            trace: false,
            clock: 0,
            function_level: 0,
            pending_irq_execution: false,
            pending_nmi_execution: false,
            pending_rst_execution: true,
        }
    }

    pub fn tick(&mut self, bus: &mut Bus) -> u8 {
        if let Some(value) = self.handle_interrupts(bus) {
            return value;
        }

        bus.start_tick();

        let pc = self.registers.pc;
        let opcode = bus.fetch(self.registers.pc);

        let instr = &INSTR_TABLE[opcode as usize];

        if instr.fname.contains("rts") {
            self.function_level -= 1;
        }

        if log_enabled!(Level::Trace) && self.trace {
            self.trace_instr(bus);
        }

        if instr.fname.contains("jsr") {
            self.function_level += 1;
        }

        self.registers.pc = self.registers.pc.wrapping_add(1);

        let cycles = (instr.fun)(self, bus);

        self.clock += cycles as u64;

        if let Some(cnt) = bus.check(cycles) {
            panic!(
                "Bus tick count mismatch: expected {}, got {}, pc = {:#06X}, opcode = {:#04X}, instr = {}",
                cycles, cnt, pc, opcode, instr.fname,
            );
        }

        self.poll_interrupts(bus);

        cycles
    }

    pub fn burn_internal_cycle(&mut self, bus: &mut Bus) {
        bus.burn_cycle_from_cpu();
    }

    fn trace_instr(&mut self, bus: &mut Bus) {
        let pc = self.registers.pc;
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

        trace!(
            "TRACE CPU -> LEVEL: {:<2} | PC: {:#06X} | {:<20} | A: {:#04X} | X: {:#04X} | Y: {:#04X} | SP: {:#04X} | P: {} ({:#04X}) [{:010}]",
            self.function_level,
            self.registers.pc,
            instr_str,
            self.registers.a_reg,
            self.registers.x_reg,
            self.registers.y_reg,
            self.registers.sp,
            self.registers.p_str(),
            self.registers.get_p(false),
            self.clock
        );
    }

    pub fn push32(&mut self, bus: &mut Bus, v: u32) {
        let (low, high) = to_u16_lh(v);

        self.push16(bus, high);
        self.push16(bus, low);
    }

    pub fn push16(&mut self, bus: &mut Bus, v: u16) {
        let (low, high) = to_u8_lh(v);

        self.push8(bus, high);
        self.push8(bus, low);
    }

    pub fn push8(&mut self, bus: &mut Bus, v: u8) {
        bus.push8(self.registers.sp, v);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
    }

    pub fn pop8(&mut self, bus: &mut Bus) -> u8 {
        self.registers.sp = self.registers.sp.wrapping_add(1);
        bus.peek8(self.registers.sp)
    }

    pub fn pop16(&mut self, bus: &mut Bus) -> u16 {
        let low = self.pop8(bus);
        let high = self.pop8(bus);

        to_u16(low, high)
    }

    pub fn pop32(&mut self, bus: &mut Bus) -> u32 {
        let low = self.pop16(bus);
        let high = self.pop16(bus);

        to_u32(low, high)
    }

    pub fn peek8(&self, bus: &mut Bus) -> u8 {
        bus.peek8(self.registers.sp + 1)
    }

    pub fn peek16(&self, bus: &mut Bus) -> u16 {
        let low = self.peek8(bus);
        let high = bus.fetch(self.registers.sp as u16 + 0x0102);

        to_u16(low, high)
    }

    pub fn peek32(&self, bus: &mut Bus) -> u32 {
        let low = self.peek16(bus);
        let high_h = bus.fetch(self.registers.sp as u16 + 0x0103);
        let high_l = bus.fetch(self.registers.sp as u16 + 0x0104);
        let high = to_u16(high_l, high_h);

        to_u32(low, high)
    }

    // decode functions

    pub fn decode_absolute(&mut self, bus: &mut Bus) -> u16 {
        let low = bus.fetch(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        let high = bus.fetch(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        to_u16(low, high)
    }

    pub fn decode_immediate(&mut self, bus: &mut Bus) -> u8 {
        let val = bus.fetch(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        val
    }

    pub fn decode_zeropage(&mut self, bus: &mut Bus) -> u8 {
        let val = bus.fetch(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        val
    }

    pub fn decode_absolute_indexed(
        &mut self,
        bus: &mut Bus,
        offset: u8,
        is_write: bool,
    ) -> (u16, u8) {
        let low = bus.fetch(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        let high = bus.fetch(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        bus.read_with_dummy(low, high, offset, is_write)
    }

    pub fn decode_zeropage_indexed(&mut self, bus: &mut Bus, offset: u8) -> u8 {
        let addr = bus.fetch(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        self.burn_internal_cycle(bus);

        addr.wrapping_add(offset)
    }

    pub fn decode_indexed_indirect(&mut self, bus: &mut Bus) -> u16 {
        let base = bus.fetch(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        self.burn_internal_cycle(bus);

        let op = (base.wrapping_add(self.registers.x_reg)) as u16 & 0xFF;
        let low = bus.fetch(op);
        let high = bus.fetch((op + 1) & 0xFF);

        to_u16(low, high)
    }

    pub fn decode_indirect_indexed(&mut self, bus: &mut Bus, is_write: bool) -> (u16, u8) {
        let op = bus.fetch(self.registers.pc) as u16;
        self.registers.pc = self.registers.pc.wrapping_add(1);

        let low = bus.fetch(op);
        let high = bus.fetch((op + 1) & 0xFF);

        bus.read_with_dummy(low, high, self.registers.y_reg, is_write)
    }

    pub fn set_trace(&mut self, value: bool) {
        self.trace = value;
    }

    fn save_state_before_interrupt(&mut self, bus: &mut Bus) {
        let pc = self.registers.pc;
        self.push16(bus, pc);

        let p = self.registers.get_p(false);
        let p_to_push = (p & !0x10) | 0x20;
        self.push8(bus, p_to_push);
        self.registers.set_i();
    }

    fn perform_irq(&mut self, bus: &mut Bus) {
        self.save_state_before_interrupt(bus);

        let low = bus.fetch(0xFFFE);
        let high = bus.fetch(0xFFFF);

        let pc = to_u16(low, high);
        self.registers.pc = pc;
        if self.trace {
            trace!("An IRQ has occurred, jumping to {}", hex16!(pc));
        }
    }

    pub fn perform_nmi(&mut self, bus: &mut Bus) {
        self.save_state_before_interrupt(bus);

        let low = bus.fetch(0xFFFA);
        let high = bus.fetch(0xFFFB);

        let nmi_address = to_u16(low, high);
        self.registers.pc = nmi_address;
        if self.trace {
            trace!(
                "A NMI interrupt has occurred, jumping to {}",
                hex16!(nmi_address)
            );
        }
    }

    fn perform_rst(&mut self, bus: &mut Bus) {
        for _ in 0..5 {
            bus.burn_cycle_from_cpu();
        }

        let low = bus.fetch(0xFFFC);
        let high = bus.fetch(0xFFFD);

        let pc = to_u16(low, high);
        self.registers.pc = pc;

        self.registers.set_i();
        self.registers.sp = self.registers.sp.wrapping_sub(3);

        if self.trace {
            trace!("A RST has occurred, jumping to {}", hex16!(pc));
        }
    }

    fn handle_interrupts(&mut self, bus: &mut Bus) -> Option<u8> {
        if self.pending_rst_execution {
            self.pending_rst_execution = false;
            self.pending_nmi_execution = false;
            self.pending_irq_execution = false;

            self.perform_rst(bus);
            return Some(7);
        }
        if self.pending_irq_execution {
            self.pending_irq_execution = false;
            self.perform_irq(bus);
            return Some(7);
        }

        if self.pending_nmi_execution {
            self.pending_nmi_execution = false;
            self.perform_nmi(bus);
            return Some(7);
        }
        None
    }

    fn poll_interrupts(&mut self, bus: &mut Bus) {
        if self.rst {
            self.rst = false;
            self.pending_rst_execution = true;
            return;
        }

        if self.nmi {
            self.nmi = false;
            self.pending_nmi_execution = true;
            return;
        }

        let irq_line_low = bus.apu().irq_active() || bus.mapper().irq_active();

        if irq_line_low && !self.registers.get_i() {
            self.pending_irq_execution = true;
        }
    }
}
