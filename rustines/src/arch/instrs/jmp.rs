use crate::arch::cpu::Cpu;
use crate::utils::bit_utils::to_u16;

pub fn absolute(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, _ilen) = decode_absolute!(cpu);
    cpu.registers.pc = addr;
    (3, 0)
}

pub fn indirect_absolute(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, _ilen) = decode_absolute!(cpu);
    let low = cpu.memory.fetch(addr);
    let high = cpu.memory.fetch(addr + 1);
    cpu.registers.pc = to_u16(low, high);
    (5, 0)
}
