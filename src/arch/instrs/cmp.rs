use crate::arch::cpu::Cpu;
use crate::utils::bit_utils::*;

pub fn immediate(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = decode_immediate!(cpu);
    let res = cpu.registers.a_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    cpu.registers.compute_c_flag(res & 0x80 == 0);
    (2, ilen)
}

pub fn zeropage(cpu: &mut Cpu) -> (u8, u8) {
    let (mut addr, ilen) = decode_zeropage!(cpu);
    addr = cpu.memory.fetch(addr as u16);
    let res = cpu.registers.a_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    cpu.registers.compute_c_flag(res & 0x80 == 0);
    (3, ilen)
}

pub fn zeropage_x(cpu: &mut Cpu) -> (u8, u8) {
    let (mut addr, ilen) = decode_zeropage_indexed!(cpu, cpu.registers.x_reg);
    addr = cpu.memory.fetch(addr as u16);
    let res = cpu.registers.a_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    cpu.registers.compute_c_flag(res & 0x80 == 0);
    (4, ilen)
}

pub fn absolute(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = decode_absolute!(cpu);
    let addr = cpu.memory.fetch(addr);
    let res = cpu.registers.a_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    cpu.registers.compute_c_flag(res & 0x80 == 0);
    (4, ilen)
}

pub fn absolute_x(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = decode_absolute_indexed!(cpu, cpu.registers.x_reg);
    let addr = cpu.memory.fetch(addr);
    let res = cpu.registers.a_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    cpu.registers.compute_c_flag(res & 0x80 == 0);
    (4, ilen)
    // TODO +1 if page boundary
}

pub fn absolute_y(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = decode_absolute_indexed!(cpu, cpu.registers.y_reg);
    let addr = cpu.memory.fetch(addr);
    let res = cpu.registers.a_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    cpu.registers.compute_c_flag(res & 0x80 == 0);
    (4, ilen)
    // TODO +1 if page boundary
}

pub fn indirect_x(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = decode_indexed_indirect!(cpu);
    let addr = cpu.memory.fetch(addr);
    let res = cpu.registers.a_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    cpu.registers.compute_c_flag(res & 0x80 == 0);
    (6, ilen)
}

pub fn indirect_y(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = decode_indirect_indexed!(cpu);
    let addr = cpu.memory.fetch(addr);
    let res = cpu.registers.a_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    cpu.registers.compute_c_flag(res & 0x80 == 0);
    (5, ilen)
    // TODO +1 if page boundary
}
