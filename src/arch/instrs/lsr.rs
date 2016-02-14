use arch::cpu::CPU;
use utils::bit_utils::*;

pub fn accumulator(cpu: &mut CPU) -> (u8, u8) {
    let mut val = cpu.registers.A;
    if val & 0x1 != 0 {
        cpu.registers.setC()
    } else {
        cpu.registers.clearC()
    };
    val = (val >> 1) & 0x7f;
    cpu.registers.A = val;
    cpu.registers.compute_NZ_flags(val);
    cpu.registers.clearN();
    (2, 1)
}

pub fn zeropage(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_zeropage!(cpu);
    
    let mut val = cpu.memory.fetch(addr as u16);
    if val & 0x1 != 0 {
        cpu.registers.setC()
    } else {
        cpu.registers.clearC()
    };
    val = (val >> 1) & 0x7f;
    cpu.registers.compute_NZ_flags(val);
    cpu.registers.clearN();
    cpu.memory.store(addr as u16, val);
    (5, ilen)
}

pub fn zeropage_x(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_zeropage_indexed!(cpu, cpu.registers.X);
    
    let mut val = cpu.memory.fetch(addr as u16);
    if val & 0x1 != 0 {
        cpu.registers.setC()
    } else {
        cpu.registers.clearC()
    };
    val = (val >> 1) & 0x7f;
    cpu.registers.compute_NZ_flags(val);
    cpu.registers.clearN();
    cpu.memory.store(addr as u16, val);
    (6, ilen)
}

pub fn absolute(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_absolute!(cpu);
    
    let mut val = cpu.memory.fetch(addr as u16);
    if val & 0x1 != 0 {
        cpu.registers.setC()
    } else {
        cpu.registers.clearC()
    };
    val = (val >> 1) & 0x7f;
    cpu.registers.compute_NZ_flags(val);
    cpu.registers.clearN();
    cpu.memory.store(addr as u16, val);
    (6, ilen)
}

pub fn absolute_x(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_absolute_indexed!(cpu, cpu.registers.X);
    
    let mut val = cpu.memory.fetch(addr as u16);
    if val & 0x1 != 0 {
        cpu.registers.setC()
    } else {
        cpu.registers.clearC()
    };
    val = (val >> 1) & 0x7f;
    cpu.registers.compute_NZ_flags(val);
    cpu.registers.clearN();
    cpu.memory.store(addr as u16, val);
    (7, ilen)
}
