use crate::arch::cpu::CPU;
use crate::utils::bit_utils::*;

pub fn zeropage(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_zeropage!(cpu);
    cpu.memory.store(addr as u16, cpu.registers.y_reg);
    (3, ilen)
}

pub fn zeropage_x(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_zeropage_indexed!(cpu, cpu.registers.x_reg);
    cpu.memory.store(addr as u16, cpu.registers.y_reg);
    (4, ilen)
}

pub fn absolute(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_absolute!(cpu);
    cpu.memory.store(addr, cpu.registers.y_reg);
    (4, ilen)
}
