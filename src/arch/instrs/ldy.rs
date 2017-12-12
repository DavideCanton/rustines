use arch::cpu::CPU;
use utils::bit_utils::*;

pub fn immediate(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_immediate!(cpu);
    cpu.registers.y_reg = addr;
    cpu.registers.compute_nz_flags(addr);
    (2, ilen)
}

pub fn zeropage(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_zeropage!(cpu);
    cpu.registers.y_reg = cpu.memory.fetch(addr as u16);
    let yval = cpu.registers.y_reg;
    cpu.registers.compute_nz_flags(yval);
    (3, ilen)
}

pub fn zeropage_x(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_zeropage_indexed!(cpu, cpu.registers.x_reg);
    cpu.registers.y_reg = cpu.memory.fetch(addr as u16);
    let yval = cpu.registers.y_reg;
    cpu.registers.compute_nz_flags(yval);
    (4, ilen)
}

pub fn absolute(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_absolute!(cpu);
    cpu.registers.y_reg = cpu.memory.fetch(addr);
    let yval = cpu.registers.y_reg;
    cpu.registers.compute_nz_flags(yval);
    (4, ilen)
}

pub fn absolute_x(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_absolute_indexed!(cpu, cpu.registers.x_reg);
    cpu.registers.y_reg = cpu.memory.fetch(addr);
    let yval = cpu.registers.y_reg;
    cpu.registers.compute_nz_flags(yval);
    (4, ilen)
    // TODO +1 if page boundary
}
