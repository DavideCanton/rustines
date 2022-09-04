use crate::arch::cpu::Cpu;
use crate::utils::bit_utils::*;

pub fn zeropage(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = decode_zeropage!(cpu);

    
    let mut val =cpu.memory.fetch(addr as u16);
    val = val.wrapping_add(1);
   cpu.memory.store(addr as u16, val);
    cpu.registers.compute_nz_flags(val);

    (5, ilen)
}

pub fn zeropage_x(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = decode_zeropage_indexed!(cpu, cpu.registers.x_reg);

    
    let mut val =cpu.memory.fetch(addr as u16);
    val = val.wrapping_add(1);
   cpu.memory.store(addr as u16, val);
    cpu.registers.compute_nz_flags(val);

    (6, ilen)
}

pub fn absolute(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = decode_absolute!(cpu);

    
    let mut val =cpu.memory.fetch(addr as u16);
    val = val.wrapping_add(1);
   cpu.memory.store(addr, val);

    cpu.registers.compute_nz_flags(val);
    (6, ilen)
}

pub fn absolute_x(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = decode_absolute_indexed!(cpu, cpu.registers.x_reg);

    
    let mut val =cpu.memory.fetch(addr as u16);
    val = val.wrapping_add(1);
   cpu.memory.store(addr, val);

    cpu.registers.compute_nz_flags(val);
    (7, ilen)
}
