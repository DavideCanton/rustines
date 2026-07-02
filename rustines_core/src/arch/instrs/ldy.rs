use crate::arch::{bus::{Bus, FetchStore}, cpu::Cpu};

pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_immediate(bus);
    cpu.registers.y_reg = addr;
    cpu.registers.compute_nz_flags(addr);
    2
}

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);
    cpu.registers.y_reg = bus.fetch(addr as u16);
    let yval = cpu.registers.y_reg;
    cpu.registers.compute_nz_flags(yval);
    3
}

pub fn zeropage_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage_indexed(bus,cpu.registers.x_reg);
    cpu.registers.y_reg = bus.fetch(addr as u16);
    let yval = cpu.registers.y_reg;
    cpu.registers.compute_nz_flags(yval);
    4
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    cpu.registers.y_reg = bus.fetch(addr);
    let yval = cpu.registers.y_reg;
    cpu.registers.compute_nz_flags(yval);
    4
}

pub fn absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, boundary) = cpu.decode_absolute_indexed(bus,cpu.registers.x_reg);
    cpu.registers.y_reg = bus.fetch(addr);
    let yval = cpu.registers.y_reg;
    cpu.registers.compute_nz_flags(yval);
    4 + boundary
}
