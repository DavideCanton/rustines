use crate::arch::{bus::Bus, cpu::Cpu};

pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let val = cpu.decode_immediate(bus);
    do_ldx(cpu, val);
    2
}

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);
    let val = bus.read(addr as u16);
    do_ldx(cpu, val);
    3
}

pub fn zeropage_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage_indexed(bus, cpu.registers.y_reg);
    let val = bus.read(addr as u16);
    do_ldx(cpu, val);
    4
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    let val = bus.read(addr);
    do_ldx(cpu, val);
    4
}

pub fn absolute_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let result = cpu.decode_absolute_indexed(bus, cpu.registers.y_reg, false);
    let addr = result.address();
    let val = bus.read(addr);
    do_ldx(cpu, val);
    4 + result.page_boundary_crossed_as_u8()
}

fn do_ldx(cpu: &mut Cpu, val: u8) {
    cpu.registers.x_reg = val;
    cpu.registers.compute_nz_flags(val);
}
