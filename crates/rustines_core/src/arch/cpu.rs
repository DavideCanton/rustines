use log::warn;

use crate::{
    arch::{
        bus::{Bus, DummyReadResult},
        instr_tracer::InstructionTracer,
        instrs::instr_table::INSTR_TABLE,
        registers::*,
    },
    utils::bit_utils::*,
};

pub struct Cpu {
    pub(crate) registers: Registers,
    clock: u64,
    pub(crate) pending_irq_execution: bool,
    pub(crate) pending_nmi_execution: bool,
    pub(crate) pending_rst_execution: bool,
    tracer: InstructionTracer,
}

impl Cpu {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Cpu {
            registers: Registers::default(),
            clock: 0,
            pending_irq_execution: false,
            pending_nmi_execution: false,
            pending_rst_execution: true,
            tracer: InstructionTracer::new(),
        }
    }

    pub fn tick(&mut self, bus: &mut Bus) -> u8 {
        if let Some(value) = self.handle_interrupts(bus) {
            self.clock += value as u64;
            return value;
        }

        bus.tick_started();

        let pc = self.registers.pc;
        let opcode = bus.read(self.registers.pc);

        self.poll_irq(bus, self.registers.get_i());

        let instr = &INSTR_TABLE[opcode as usize];

        self.tracer
            .trace_instruction(&self.registers, self.clock, bus, instr);

        self.registers.pc = self.registers.pc.wrapping_add(1);

        let cycles = (instr.fun)(self, bus);

        self.clock += cycles as u64;

        if let Some(cnt) = bus.check_tick_end(cycles) {
            warn!(
                "Bus tick count mismatch: expected {}, got {}, pc = {:#06X}, opcode = {:#04X}, instr = {}",
                cycles, cnt, pc, opcode, instr.fname,
            );
        }

        self.poll_non_maskable_interrupts(bus);

        cycles
    }

    pub fn burn_internal_cycle(&mut self, bus: &mut Bus) {
        bus.burn_cycle_from_cpu();
    }

    pub fn push16(&mut self, bus: &mut Bus, v: u16) {
        let (low, high) = to_u8_lh(v);

        self.push8(bus, high);
        self.push8(bus, low);
    }

    pub fn push8(&mut self, bus: &mut Bus, v: u8) {
        bus.push(self.registers.sp, v);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
    }

    pub fn pop8(&mut self, bus: &mut Bus) -> u8 {
        self.registers.sp = self.registers.sp.wrapping_add(1);
        bus.pop(self.registers.sp)
    }

    pub fn pop16(&mut self, bus: &mut Bus) -> u16 {
        let low = self.pop8(bus);
        let high = self.pop8(bus);

        to_u16(low, high)
    }

    pub fn peek8(&self, bus: &mut Bus) -> u8 {
        bus.pop(self.registers.sp.wrapping_add(1))
    }

    // decode functions

    pub fn decode_absolute(&mut self, bus: &mut Bus) -> u16 {
        let low = bus.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        let high = bus.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        to_u16(low, high)
    }

    pub fn decode_immediate(&mut self, bus: &mut Bus) -> u8 {
        let val = bus.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        val
    }

    pub fn decode_zeropage(&mut self, bus: &mut Bus) -> u8 {
        let val = bus.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        val
    }

    pub fn decode_absolute_indexed(
        &mut self,
        bus: &mut Bus,
        offset: u8,
        is_write: bool,
    ) -> DummyReadResult {
        let low = bus.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        let high = bus.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        bus.perform_dummy_read(low, high, offset, is_write)
    }

    pub fn decode_zeropage_indexed(&mut self, bus: &mut Bus, offset: u8) -> u8 {
        let addr = bus.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        self.burn_internal_cycle(bus);

        addr.wrapping_add(offset)
    }

    pub fn decode_indexed_indirect(&mut self, bus: &mut Bus) -> u16 {
        let base = bus.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        self.burn_internal_cycle(bus);

        let op = (base.wrapping_add(self.registers.x_reg)) as u16 & 0b1111_1111;
        let low = bus.read(op);
        let high = bus.read((op + 1) & 0b1111_1111);

        to_u16(low, high)
    }

    pub fn decode_indirect_indexed(&mut self, bus: &mut Bus, is_write: bool) -> DummyReadResult {
        let op = bus.read(self.registers.pc) as u16;
        self.registers.pc = self.registers.pc.wrapping_add(1);

        let low = bus.read(op);
        let high = bus.read((op + 1) & 0b1111_1111);

        bus.perform_dummy_read(low, high, self.registers.y_reg, is_write)
    }

    pub fn enable_tracing(&mut self, tracing_enabled: bool) {
        self.tracer.enable_tracing(tracing_enabled);
    }

    fn save_state_before_interrupt(&mut self, bus: &mut Bus) {
        let pc = self.registers.pc;
        self.push16(bus, pc);

        let p = self.registers.get_p_force_b(false);
        self.push8(bus, p);
        self.registers.set_i();
    }

    fn perform_irq(&mut self, bus: &mut Bus) -> u8 {
        bus.read(self.registers.pc);
        bus.read(self.registers.pc);

        self.save_state_before_interrupt(bus);

        let low = bus.read(0xFFFE);
        let high = bus.read(0xFFFF);

        let irq_address = to_u16(low, high);
        self.tracer
            .trace_interrupt("IRQ", irq_address, &self.registers, self.clock);

        self.registers.pc = irq_address;

        7
    }

    pub fn perform_nmi(&mut self, bus: &mut Bus) -> u8 {
        bus.read(self.registers.pc);
        bus.read(self.registers.pc);

        self.save_state_before_interrupt(bus);

        let low = bus.read(0xFFFA);
        let high = bus.read(0xFFFB);

        let nmi_address = to_u16(low, high);
        self.tracer
            .trace_interrupt("NMI", nmi_address, &self.registers, self.clock);

        self.registers.pc = nmi_address;

        7
    }

    fn perform_rst(&mut self, bus: &mut Bus) -> u8 {
        for _ in 0..5 {
            bus.burn_cycle_from_cpu();
        }

        let low = bus.read(0xFFFC);
        let high = bus.read(0xFFFD);

        let rst_address = to_u16(low, high);
        self.tracer
            .trace_interrupt("RST", rst_address, &self.registers, self.clock);

        self.registers.pc = rst_address;

        self.registers.set_i();
        self.registers.sp = self.registers.sp.wrapping_sub(3);

        7
    }

    fn handle_interrupts(&mut self, bus: &mut Bus) -> Option<u8> {
        if self.pending_rst_execution {
            self.pending_rst_execution = false;
            Some(self.perform_rst(bus))
        } else if self.pending_nmi_execution {
            self.clear_nmi(bus);
            Some(self.perform_nmi(bus))
        } else if self.pending_irq_execution {
            self.pending_irq_execution = false;
            Some(self.perform_irq(bus))
        } else {
            None
        }
    }

    pub fn clear_nmi(&mut self, bus: &mut Bus) {
        self.pending_nmi_execution = false;
        bus.ppu_mut().clear_nmi();
    }

    pub fn poll_non_maskable_interrupts(&mut self, bus: &mut Bus) {
        if bus.ppu_mut().nmi_requested() {
            self.pending_nmi_execution = true;
        }
    }

    fn poll_irq(&mut self, bus: &Bus, irq_masked: bool) {
        let irq_line_low = bus.apu().irq_active() || bus.mapper_ref().irq_active();

        if irq_line_low && !irq_masked {
            self.pending_irq_execution = true;
        }
    }
}
