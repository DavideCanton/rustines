use crate::arch::{
    bus::{Bus, FetchStore},
    cpu::Cpu,
};

pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_immediate(bus);
    let res = cpu.registers.a_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    cpu.registers.compute_c_flag(res & 0x80 == 0);
    2
}

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let mut addr = cpu.decode_zeropage(bus);
    addr = bus.fetch(addr as u16);
    let res = cpu.registers.a_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    cpu.registers.compute_c_flag(res & 0x80 == 0);
    3
}

pub fn zeropage_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let mut addr = cpu.decode_zeropage_indexed(bus, cpu.registers.x_reg);
    addr = bus.fetch(addr as u16);
    let res = cpu.registers.a_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    cpu.registers.compute_c_flag(res & 0x80 == 0);
    4
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    let addr = bus.fetch(addr);
    let res = cpu.registers.a_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    cpu.registers.compute_c_flag(res & 0x80 == 0);
    4
}

pub fn absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, boundary) = cpu.decode_absolute_indexed(bus, cpu.registers.x_reg);
    let addr = bus.fetch(addr);
    let res = cpu.registers.a_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    cpu.registers.compute_c_flag(res & 0x80 == 0);
    4 + boundary
}

pub fn absolute_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, boundary) = cpu.decode_absolute_indexed(bus, cpu.registers.y_reg);
    let addr = bus.fetch(addr);
    let res = cpu.registers.a_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    cpu.registers.compute_c_flag(res & 0x80 == 0);
    4 + boundary
}

pub fn indirect_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_indexed_indirect(bus);
    let addr = bus.fetch(addr);
    let res = cpu.registers.a_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    cpu.registers.compute_c_flag(res & 0x80 == 0);
    6
}

pub fn indirect_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, boundary) = cpu.decode_indirect_indexed(bus);
    let addr = bus.fetch(addr);
    let res = cpu.registers.a_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    cpu.registers.compute_c_flag(res & 0x80 == 0);
    5 + boundary
}
