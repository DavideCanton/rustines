use crate::arch::bus::{Bus, FetchStore};
use crate::arch::cpu::Cpu;
use crate::utils::bit_utils::to_u16;

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    cpu.registers.pc = addr;
    3
}

pub fn indirect_absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    let low = bus.fetch(addr);
    let high = bus.fetch(addr + 1);
    cpu.registers.pc = to_u16(low, high);
    5
}
