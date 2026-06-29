use crate::arch::{cpu::Cpu, memory::FetchStore};

pub fn immediate(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_immediate();
    cpu.registers.x_reg = addr;
    cpu.registers.compute_nz_flags(addr);
    2
}

pub fn zeropage(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_zeropage();
    cpu.registers.x_reg = cpu.memory.fetch(addr as u16);
    let xval = cpu.registers.x_reg;
    cpu.registers.compute_nz_flags(xval);
    3
}

pub fn zeropage_y(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_zeropage_indexed(cpu.registers.y_reg);
    cpu.registers.x_reg = cpu.memory.fetch(addr as u16);
    let xval = cpu.registers.x_reg;
    cpu.registers.compute_nz_flags(xval);
    4
}

pub fn absolute(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_absolute();
    cpu.registers.x_reg = cpu.memory.fetch(addr);
    let xval = cpu.registers.x_reg;
    cpu.registers.compute_nz_flags(xval);
    4
}

pub fn absolute_y(cpu: &mut Cpu) -> u8 {
    let (addr, boundary) = cpu.decode_absolute_indexed(cpu.registers.y_reg);
    cpu.registers.x_reg = cpu.memory.fetch(addr);
    let xval = cpu.registers.x_reg;
    cpu.registers.compute_nz_flags(xval);
    4 + boundary
}
