use crate::arch::{cpu::Cpu, memory::FetchStore};

pub fn immediate(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_immediate();
    cpu.registers.y_reg = addr;
    cpu.registers.compute_nz_flags(addr);
    (2, ilen)
}

pub fn zeropage(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_zeropage();
    cpu.registers.y_reg = cpu.memory.fetch(addr as u16);
    let yval = cpu.registers.y_reg;
    cpu.registers.compute_nz_flags(yval);
    (3, ilen)
}

pub fn zeropage_x(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_zeropage_indexed(cpu.registers.x_reg);
    cpu.registers.y_reg = cpu.memory.fetch(addr as u16);
    let yval = cpu.registers.y_reg;
    cpu.registers.compute_nz_flags(yval);
    (4, ilen)
}

pub fn absolute(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_absolute();
    cpu.registers.y_reg = cpu.memory.fetch(addr);
    let yval = cpu.registers.y_reg;
    cpu.registers.compute_nz_flags(yval);
    (4, ilen)
}

pub fn absolute_x(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen, boundary) = cpu.decode_absolute_indexed(cpu.registers.x_reg);
    cpu.registers.y_reg = cpu.memory.fetch(addr);
    let yval = cpu.registers.y_reg;
    cpu.registers.compute_nz_flags(yval);
    (4 + boundary, ilen)
}
