use arch::cpu::CPU;
use utils::bit_utils::*;

pub fn accumulator(cpu: &mut CPU) -> (u8, u8) {
    let mut val = cpu.registers.a_reg;
    if val & 0x1 != 0 {
        cpu.registers.set_c()
    } else {
        cpu.registers.clear_c()
    };
    val = (val >> 1) & 0x7f;
    cpu.registers.a_reg = val;
    cpu.registers.compute_nz_flags(val);
    cpu.registers.clear_n();
    (2, 1)
}

pub fn zeropage(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_zeropage!(cpu);
    
    let mut val = cpu.memory.fetch(addr as u16);
    if val & 0x1 != 0 {
        cpu.registers.set_c()
    } else {
        cpu.registers.clear_c()
    };
    val = (val >> 1) & 0x7f;
    cpu.registers.compute_nz_flags(val);
    cpu.registers.clear_n();
    cpu.memory.store(addr as u16, val);
    (5, ilen)
}

pub fn zeropage_x(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_zeropage_indexed!(cpu, cpu.registers.x_reg);
    
    let mut val = cpu.memory.fetch(addr as u16);
    if val & 0x1 != 0 {
        cpu.registers.set_c()
    } else {
        cpu.registers.clear_c()
    };
    val = (val >> 1) & 0x7f;
    cpu.registers.compute_nz_flags(val);
    cpu.registers.clear_n();
    cpu.memory.store(addr as u16, val);
    (6, ilen)
}

pub fn absolute(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_absolute!(cpu);
    
    let mut val = cpu.memory.fetch(addr as u16);
    if val & 0x1 != 0 {
        cpu.registers.set_c()
    } else {
        cpu.registers.clear_c()
    };
    val = (val >> 1) & 0x7f;
    cpu.registers.compute_nz_flags(val);
    cpu.registers.clear_n();
    cpu.memory.store(addr as u16, val);
    (6, ilen)
}

pub fn absolute_x(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_absolute_indexed!(cpu, cpu.registers.x_reg);
    
    let mut val = cpu.memory.fetch(addr as u16);
    if val & 0x1 != 0 {
        cpu.registers.set_c()
    } else {
        cpu.registers.clear_c()
    };
    val = (val >> 1) & 0x7f;
    cpu.registers.compute_nz_flags(val);
    cpu.registers.clear_n();
    cpu.memory.store(addr as u16, val);
    (7, ilen)
}
