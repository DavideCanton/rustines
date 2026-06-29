use crate::arch::cpu::Cpu;
use crate::arch::memory::FetchStore;
use crate::utils::bit_utils::to_u16;

pub fn absolute(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_absolute();
    cpu.registers.pc = addr;
    3
}

pub fn indirect_absolute(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_absolute();
    let low = cpu.memory.fetch(addr);
    let high = cpu.memory.fetch(addr + 1);
    cpu.registers.pc = to_u16(low, high);
    5
}
