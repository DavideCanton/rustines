use crate::arch::{
    bus::{Bus, FetchStore},
    cpu::Cpu,
};

pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let val = cpu.decode_immediate(bus);
    do_eor(cpu, val);
    2
}

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);
    let val = bus.fetch(addr as u16);
    do_eor(cpu, val);
    3
}

pub fn zeropage_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage_indexed(bus, cpu.registers.x_reg);
    let val = bus.fetch(addr as u16);
    do_eor(cpu, val);
    4
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    let val = bus.fetch(addr);
    do_eor(cpu, val);
    4
}

pub fn absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, boundary) = cpu.decode_absolute_indexed(bus, cpu.registers.x_reg);
    let val = bus.fetch(addr);
    do_eor(cpu, val);
    4 + boundary
}

pub fn absolute_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, boundary) = cpu.decode_absolute_indexed(bus, cpu.registers.y_reg);
    let val = bus.fetch(addr);
    do_eor(cpu, val);
    4 + boundary
}

pub fn indirect_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_indexed_indirect(bus);
    let val = bus.fetch(addr);
    do_eor(cpu, val);
    6
}

pub fn indirect_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, boundary) = cpu.decode_indirect_indexed(bus);
    let val = bus.fetch(addr);
    do_eor(cpu, val);
    5 + boundary
}

fn do_eor(cpu: &mut Cpu, val: u8) {
    let res = cpu.registers.a_reg ^ val;
    cpu.registers.a_reg = res;
    cpu.registers.compute_nz_flags(res);
}
