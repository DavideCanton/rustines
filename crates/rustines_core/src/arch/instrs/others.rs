use crate::arch::bus::{Bus, FetchStore};
use crate::arch::cpu::Cpu;
use crate::utils::bit_utils::*;

pub fn nop(_: &mut Cpu, _: &mut Bus) -> u8 {
    2
}

pub fn brk(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let pc = cpu.registers.pc;
    cpu.push16(bus, pc + 2);
    let p = cpu.registers.get_p(true);
    cpu.push8(bus, p);
    let l = bus.fetch(0xFFFE);
    let h = bus.fetch(0xFFFF);

    cpu.registers.pc = to_u16(l, h);

    7
}
