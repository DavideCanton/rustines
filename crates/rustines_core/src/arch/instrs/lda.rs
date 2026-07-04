use crate::arch::{bus::{Bus, FetchStore}, cpu::Cpu};

pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_immediate(bus);
    cpu.registers.a_reg = addr;
    cpu.registers.compute_nz_flags(addr);
    2
}

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);
    let aval = bus.fetch(addr as u16);
    cpu.registers.a_reg = aval;
    cpu.registers.compute_nz_flags(aval);
    3
}

pub fn zeropage_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage_indexed(bus,cpu.registers.x_reg);
    cpu.registers.a_reg = bus.fetch(addr as u16);
    let aval = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(aval);
    4
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    cpu.registers.a_reg = bus.fetch(addr);
    let aval = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(aval);
    4
}

pub fn absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, boundary) = cpu.decode_absolute_indexed(bus,cpu.registers.x_reg);
    cpu.registers.a_reg = bus.fetch(addr);
    let aval = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(aval);
    4 + boundary
}

pub fn absolute_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, boundary) = cpu.decode_absolute_indexed(bus,cpu.registers.y_reg);
    cpu.registers.a_reg = bus.fetch(addr);
    let aval = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(aval);
    4 + boundary
}

pub fn indirect_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_indexed_indirect(bus);
    cpu.registers.a_reg = bus.fetch(addr);
    let aval = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(aval);
    6
}

pub fn indirect_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, boundary) = cpu.decode_indirect_indexed(bus);
    cpu.registers.a_reg = bus.fetch(addr);
    let aval = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(aval);
    5 + boundary
}
