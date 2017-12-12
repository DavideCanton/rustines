use arch::cpu::CPU;
use utils::bit_utils::*;

pub fn immediate(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_immediate!(cpu);
    cpu.registers.a_reg = addr;
    cpu.registers.compute_nz_flags(addr);
    (2, ilen)
}

pub fn zeropage(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_zeropage!(cpu);
    let aval = cpu.memory.fetch(addr as u16);
    cpu.registers.a_reg = aval;
    cpu.registers.compute_nz_flags(aval);
    (3, ilen)
}

pub fn zeropage_x(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_zeropage_indexed!(cpu, cpu.registers.x_reg);
    cpu.registers.a_reg = cpu.memory.fetch(addr as u16);
    let aval = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(aval);
    (4, ilen)
}

pub fn absolute(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_absolute!(cpu);
    cpu.registers.a_reg = cpu.memory.fetch(addr);
    let aval = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(aval);
    (4, ilen)
}

pub fn absolute_x(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_absolute_indexed!(cpu, cpu.registers.x_reg);
    cpu.registers.a_reg = cpu.memory.fetch(addr);
    let aval = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(aval);
    (4, ilen)
    // TODO +1 if page boundary
}

pub fn absolute_y(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_absolute_indexed!(cpu, cpu.registers.y_reg);
    cpu.registers.a_reg = cpu.memory.fetch(addr);
    let aval = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(aval);
    (4, ilen)
    // TODO +1 if page boundary
}

pub fn indirect_x(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_indexed_indirect!(cpu);
    cpu.registers.a_reg = cpu.memory.fetch(addr);
    let aval = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(aval);
    (6, ilen)
}

pub fn indirect_y(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_indirect_indexed!(cpu);
    cpu.registers.a_reg = cpu.memory.fetch(addr);
    let aval = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(aval);
    (5, ilen)
    // TODO +1 if page boundary
}
