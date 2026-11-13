use crate::arch::{bus::Bus, cpu::Cpu};

pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let val = cpu.decode_immediate(bus);
    do_ora(cpu, val);
    2
}

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);
    let val = bus.read(addr as u16);
    do_ora(cpu, val);
    3
}

pub fn zeropage_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage_indexed(bus, cpu.registers.x_reg);
    let val = bus.read(addr as u16);
    do_ora(cpu, val);
    4
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    let val = bus.read(addr);
    do_ora(cpu, val);
    4
}

pub fn absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let result = cpu.decode_absolute_indexed(bus, cpu.registers.x_reg, false);
    let addr = result.address();
    let val = bus.read(addr);
    do_ora(cpu, val);
    4 + result.page_boundary_crossed_as_u8()
}

pub fn absolute_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let result = cpu.decode_absolute_indexed(bus, cpu.registers.y_reg, false);
    let addr = result.address();
    let val = bus.read(addr);
    do_ora(cpu, val);
    4 + result.page_boundary_crossed_as_u8()
}

pub fn indirect_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_indexed_indirect(bus);
    let val = bus.read(addr);
    do_ora(cpu, val);
    6
}

pub fn indirect_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let result = cpu.decode_indirect_indexed(bus, false);
    let addr = result.address();
    let val = bus.read(addr);
    do_ora(cpu, val);
    5 + result.page_boundary_crossed_as_u8()
}

fn do_ora(cpu: &mut Cpu, addr: u8) {
    let res = cpu.registers.a_reg | addr;
    cpu.registers.a_reg = res;
    cpu.registers.update_nz_flags(res);
}
