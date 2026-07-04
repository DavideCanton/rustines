use crate::arch::{bus::{Bus, FetchStore}, cpu::Cpu};

pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_immediate(bus);
    cpu.registers.x_reg = addr;
    cpu.registers.compute_nz_flags(addr);
    2
}

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);
    cpu.registers.x_reg = bus.fetch(addr as u16);
    let xval = cpu.registers.x_reg;
    cpu.registers.compute_nz_flags(xval);
    3
}

pub fn zeropage_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage_indexed(bus,cpu.registers.y_reg);
    cpu.registers.x_reg = bus.fetch(addr as u16);
    let xval = cpu.registers.x_reg;
    cpu.registers.compute_nz_flags(xval);
    4
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    cpu.registers.x_reg = bus.fetch(addr);
    let xval = cpu.registers.x_reg;
    cpu.registers.compute_nz_flags(xval);
    4
}

pub fn absolute_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, boundary) = cpu.decode_absolute_indexed(bus,cpu.registers.y_reg);
    cpu.registers.x_reg = bus.fetch(addr);
    let xval = cpu.registers.x_reg;
    cpu.registers.compute_nz_flags(xval);
    4 + boundary
}
