use crate::arch::cpu::Cpu;
use crate::utils::bit_utils::*;

pub fn immediate(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = decode_immediate!(cpu);
    cpu.registers.x_reg = addr;
    cpu.registers.compute_nz_flags(addr);
    (2, ilen)
}

pub fn zeropage(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = decode_zeropage!(cpu);
    cpu.registers.x_reg = cpu.memory.fetch(addr as u16);
    let xval = cpu.registers.x_reg;
    cpu.registers.compute_nz_flags(xval);
    (3, ilen)
}

pub fn zeropage_y(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = decode_zeropage_indexed!(cpu, cpu.registers.y_reg);
    cpu.registers.x_reg = cpu.memory.fetch(addr as u16);
    let xval = cpu.registers.x_reg;
    cpu.registers.compute_nz_flags(xval);
    (4, ilen)
}

pub fn absolute(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = decode_absolute!(cpu);
    cpu.registers.x_reg = cpu.memory.fetch(addr);
    let xval = cpu.registers.x_reg;
    cpu.registers.compute_nz_flags(xval);
    (4, ilen)
}

pub fn absolute_y(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = decode_absolute_indexed!(cpu, cpu.registers.y_reg);
    cpu.registers.x_reg = cpu.memory.fetch(addr);
    let xval = cpu.registers.x_reg;
    cpu.registers.compute_nz_flags(xval);
    (4, ilen)
    // TODO +1 if page boundary
}
