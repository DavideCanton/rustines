use log::info;

use crate::{
    arch::{
        bus::{Bus, FetchStore},
        instrs::instr_table::INSTR_TABLE,
        registers::*,
    },
    bin, hex,
    utils::bit_utils::*,
};

pub struct Cpu {
    pub clock: u8,
    pub registers: Registers,
    pub irq: bool,
    pub nmi: bool,
    pub rst: bool,
}

impl Cpu {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Cpu {
            clock: 0,
            registers: Registers::default(),
            irq: false,
            nmi: false,
            rst: true,
        }
    }

    pub fn tick(&mut self, bus: &mut Bus, verbose: bool) -> u8 {
        if verbose {
            info!(
                "[{: <4}] {: <20}A:{} X:{} Y:{} P:{: <2} ({}) SP:{}",
                hex!(self.registers.pc),
                "",
                hex!(self.registers.a_reg),
                hex!(self.registers.x_reg),
                hex!(self.registers.y_reg),
                0,
                "NV_BDIZC",
                hex!(self.registers.sp),
            );
        }

        self.handle_interrupts(bus);

        let opcode = bus.fetch(self.registers.pc);

        let instr = &INSTR_TABLE[opcode as usize];

        if verbose {
            let ilen = if instr.ilen == 0xFF { 1 } else { instr.ilen };
            let mut data = vec![0; ilen];
            bus.fetch_many(self.registers.pc, &mut data);
            let p = self.registers.get_p();
            info!(
                "[{: <4}] {: <20}A:{} X:{} Y:{} P:{: <2} ({}) SP:{}",
                hex!(self.registers.pc),
                instr.get_fname_for_print(&data),
                hex!(self.registers.a_reg),
                hex!(self.registers.x_reg),
                hex!(self.registers.y_reg),
                hex!(p),
                bin!(p),
                hex!(self.registers.sp),
            );
        }

        self.registers.pc += instr.ilen as u16;

        (instr.fun)(self, bus)
    }

    pub fn push32(&mut self, bus: &mut Bus, v: u32) {
        let (low, high) = to_u16_lh(v);

        // TODO is the order right?
        self.push16(bus, high);
        self.push16(bus, low);
    }

    pub fn push16(&mut self, bus: &mut Bus, v: u16) {
        let (low, high) = to_u8_lh(v);

        // TODO is the order right?
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

    pub fn decode_absolute(&self, bus: &mut Bus) -> u16 {
        let low = bus.fetch(self.registers.pc - 2);
        let high = bus.fetch(self.registers.pc - 1);
        to_u16(low, high)
    }

    pub fn decode_immediate(&self, bus: &mut Bus) -> u8 {
        bus.fetch(self.registers.pc - 1)
    }

    pub fn decode_zeropage(&self, bus: &mut Bus) -> u8 {
        bus.fetch(self.registers.pc - 1)
    }

    pub fn decode_absolute_indexed(&self, bus: &mut Bus, offset: u8) -> (u16, u8) {
        let low = bus.fetch(self.registers.pc - 2);
        let high = bus.fetch(self.registers.pc - 1);
        (
            to_u16(low, high).wrapping_add(offset as u16),
            overflow_page_boundary(low, offset),
        )
    }

    pub fn decode_zeropage_indexed(&self, bus: &mut Bus, offset: u8) -> u8 {
        let addr = bus.fetch(self.registers.pc - 1);
        addr.wrapping_add(offset)
    }

    pub fn decode_indexed_indirect(&self, bus: &mut Bus) -> u16 {
        let op = (bus
            .fetch(self.registers.pc - 1)
            .wrapping_add(self.registers.x_reg)) as u16
            & 0xFF;
        let low = bus.fetch(op);
        let high = bus.fetch((op + 1) & 0xFF);

        to_u16(low, high)
    }

    pub fn decode_indirect_indexed(&self, bus: &mut Bus) -> (u16, u8) {
        let op = bus.fetch(self.registers.pc - 1) as u16;
        let low = bus.fetch(op);
        let high = bus.fetch((op + 1) & 0xFF);
        let offset = self.registers.y_reg;

        (
            to_u16(low, high).wrapping_add(offset as u16),
            overflow_page_boundary(low, offset),
        )
    }

    fn save_state_before_interrupt(&mut self, bus: &mut Bus) {
        let pc = self.registers.pc;
        self.push16(bus, pc);
        let p = self.registers.get_p();
        self.push8(bus, p);
    }

    fn perform_irq(&mut self, bus: &mut Bus) {
        self.save_state_before_interrupt(bus);

        let low = bus.fetch(0xFFFE);
        let high = bus.fetch(0xFFFF);

        self.registers.pc = to_u16(low, high);
    }

    pub fn perform_nmi(&mut self, bus: &mut Bus) {
        self.save_state_before_interrupt(bus);

        let low = bus.fetch(0xFFFA);
        let high = bus.fetch(0xFFFB);

        self.registers.pc = to_u16(low, high);
    }

    fn perform_rst(&mut self, bus: &mut Bus) {
        let low = bus.fetch(0xFFFC);
        let high = bus.fetch(0xFFFD);

        self.registers.pc = to_u16(low, high);
        self.rst = false;
    }

    fn handle_interrupts(&mut self, bus: &mut Bus) {
        // TODO verify priority
        if self.nmi {
            self.perform_nmi(bus);
        } else if self.irq {
            self.perform_irq(bus);
        } else if self.rst {
            self.perform_rst(bus);
        }

        self.irq = false;
        self.nmi = false;
        self.rst = false;
    }
}

fn overflow_page_boundary(low: u8, offset: u8) -> u8 {
    if low.overflowing_add(offset).1 { 1 } else { 0 }
}
