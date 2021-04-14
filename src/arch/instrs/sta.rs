use crate::arch::cpu::Cpu;
use crate::utils::bit_utils::*;

pub fn zeropage(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = decode_zeropage!(cpu);
    cpu.memory.store(addr as u16, cpu.registers.a_reg);
    (3, ilen)
}

pub fn zeropage_x(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = decode_zeropage_indexed!(cpu, cpu.registers.x_reg);
    cpu.memory.store(addr as u16, cpu.registers.a_reg);
    (4, ilen)
}

pub fn absolute(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = decode_absolute!(cpu);
    cpu.memory.store(addr, cpu.registers.a_reg);
    (4, ilen)
}

pub fn absolute_x(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = decode_absolute_indexed!(cpu, cpu.registers.x_reg);
    cpu.memory.store(addr, cpu.registers.a_reg);
    (5, ilen)
}

pub fn absolute_y(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = decode_absolute_indexed!(cpu, cpu.registers.y_reg);
    cpu.memory.store(addr, cpu.registers.a_reg);
    (5, ilen)
}

pub fn indirect_x(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = decode_indexed_indirect!(cpu);
    cpu.memory.store(addr, cpu.registers.a_reg);
    (6, ilen)
}

pub fn indirect_y(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = decode_indirect_indexed!(cpu);
    cpu.memory.store(addr, cpu.registers.a_reg);
    (6, ilen)
}
