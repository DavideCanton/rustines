use crate::arch::{cpu::Cpu, memory::FetchStore};

pub fn immediate(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_immediate();
    cpu.registers.a_reg = addr;
    cpu.registers.compute_nz_flags(addr);
    2
}

pub fn zeropage(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_zeropage();
    let aval = cpu.memory.fetch(addr as u16);
    cpu.registers.a_reg = aval;
    cpu.registers.compute_nz_flags(aval);
    3
}

pub fn zeropage_x(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_zeropage_indexed(cpu.registers.x_reg);
    cpu.registers.a_reg = cpu.memory.fetch(addr as u16);
    let aval = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(aval);
    4
}

pub fn absolute(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_absolute();
    cpu.registers.a_reg = cpu.memory.fetch(addr);
    let aval = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(aval);
    4
}

pub fn absolute_x(cpu: &mut Cpu) -> u8 {
    let (addr, boundary) = cpu.decode_absolute_indexed(cpu.registers.x_reg);
    cpu.registers.a_reg = cpu.memory.fetch(addr);
    let aval = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(aval);
    4 + boundary
}

pub fn absolute_y(cpu: &mut Cpu) -> u8 {
    let (addr, boundary) = cpu.decode_absolute_indexed(cpu.registers.y_reg);
    cpu.registers.a_reg = cpu.memory.fetch(addr);
    let aval = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(aval);
    4 + boundary
}

pub fn indirect_x(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_indexed_indirect();
    cpu.registers.a_reg = cpu.memory.fetch(addr);
    let aval = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(aval);
    6
}

pub fn indirect_y(cpu: &mut Cpu) -> u8 {
    let (addr, boundary) = cpu.decode_indirect_indexed();
    cpu.registers.a_reg = cpu.memory.fetch(addr);
    let aval = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(aval);
    5 + boundary
}
