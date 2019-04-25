use crate::arch::cpu::CPU;
use crate::utils::bit_utils::*;

pub fn accumulator(cpu: &mut CPU) -> (u8, u8) {
    let mut val = cpu.registers.a_reg;
    if val & 0x80 != 0 {
        cpu.registers.set_c()
    } else {
        cpu.registers.clear_c()
    };
    val <<= 1;
    cpu.registers.a_reg = val;
    cpu.registers.compute_nz_flags(val);
    (2, 1)
}

pub fn zeropage(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_zeropage!(cpu);
    
    let mut val = cpu.memory.fetch(addr as u16);
    if val & 0x80 != 0 {
        cpu.registers.set_c()
    } else {
        cpu.registers.clear_c()
    };
    val <<= 1;
    cpu.registers.compute_nz_flags(val);
    cpu.memory.store(addr as u16, val);
    (5, ilen)
}

pub fn zeropage_x(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_zeropage_indexed!(cpu, cpu.registers.x_reg);
    
    let mut val = cpu.memory.fetch(addr as u16);
    if val & 0x80 != 0 {
        cpu.registers.set_c()
    } else {
        cpu.registers.clear_c()
    };
    val <<= 1;
    cpu.registers.compute_nz_flags(val);
    cpu.memory.store(addr as u16, val);
    (6, ilen)
}

pub fn absolute(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_absolute!(cpu);
    
    let mut val = cpu.memory.fetch(addr as u16);
    if val & 0x80 != 0 {
        cpu.registers.set_c()
    } else {
        cpu.registers.clear_c()
    };
    val <<= 1;
    cpu.registers.compute_nz_flags(val);
    cpu.memory.store(addr as u16, val);
    (6, ilen)
}

pub fn absolute_x(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_absolute_indexed!(cpu, cpu.registers.x_reg);
    
    let mut val = cpu.memory.fetch(addr as u16);
    if val & 0x80 != 0 {
        cpu.registers.set_c()
    } else {
        cpu.registers.clear_c()
    };
    val <<= 1;
    cpu.registers.compute_nz_flags(val);
    cpu.memory.store(addr as u16, val);
    (7, ilen)
}
