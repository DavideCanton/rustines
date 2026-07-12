use crate::arch::{
    bus::{Bus, FetchStore},
    cpu::Cpu,
};

pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let val = cpu.decode_immediate(bus);
    let a_reg = cpu.registers.a_reg;
    let res = (a_reg as u16).wrapping_sub(val as u16);

    cpu.registers.set_c_from_bool(a_reg >= val);
    cpu.registers.set_n_from_bool((res & 0x80) != 0);
    cpu.registers.set_z_from_bool(a_reg == val);

    2
}

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);
    let val = bus.fetch(addr as u16);
    let a_reg = cpu.registers.a_reg;
    let res = (a_reg as u16).wrapping_sub(val as u16);

    cpu.registers.set_c_from_bool(a_reg >= val);
    cpu.registers.set_n_from_bool((res & 0x80) != 0);
    cpu.registers.set_z_from_bool(a_reg == val);

    3
}

pub fn zeropage_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage_indexed(bus, cpu.registers.x_reg);
    let val = bus.fetch(addr as u16);
    let a_reg = cpu.registers.a_reg;
    let res = (a_reg as u16).wrapping_sub(val as u16);

    cpu.registers.set_c_from_bool(a_reg >= val);
    cpu.registers.set_n_from_bool((res & 0x80) != 0);
    cpu.registers.set_z_from_bool(a_reg == val);
    4
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    let val = bus.fetch(addr);
    let a_reg = cpu.registers.a_reg;
    let res = (a_reg as u16).wrapping_sub(val as u16);

    cpu.registers.set_c_from_bool(a_reg >= val);
    cpu.registers.set_n_from_bool((res & 0x80) != 0);
    cpu.registers.set_z_from_bool(a_reg == val);

    4
}

pub fn absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, boundary) = cpu.decode_absolute_indexed(bus, cpu.registers.x_reg);
    let val = bus.fetch(addr);
    let a_reg = cpu.registers.a_reg;
    let res = (a_reg as u16).wrapping_sub(val as u16);

    cpu.registers.set_c_from_bool(a_reg >= val);
    cpu.registers.set_n_from_bool((res & 0x80) != 0);
    cpu.registers.set_z_from_bool(a_reg == val);

    4 + boundary
}

pub fn absolute_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, boundary) = cpu.decode_absolute_indexed(bus, cpu.registers.y_reg);
    let val = bus.fetch(addr);
    let a_reg = cpu.registers.a_reg;
    let res = (a_reg as u16).wrapping_sub(val as u16);

    cpu.registers.set_c_from_bool(a_reg >= val);
    cpu.registers.set_n_from_bool((res & 0x80) != 0);
    cpu.registers.set_z_from_bool(a_reg == val);
    4 + boundary
}

pub fn indirect_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_indexed_indirect(bus);
    let val = bus.fetch(addr);
    let a_reg = cpu.registers.a_reg;
    let res = (a_reg as u16).wrapping_sub(val as u16);

    cpu.registers.set_c_from_bool(a_reg >= val);
    cpu.registers.set_n_from_bool((res & 0x80) != 0);
    cpu.registers.set_z_from_bool(a_reg == val);
    6
}

pub fn indirect_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, boundary) = cpu.decode_indirect_indexed(bus);
    let val = bus.fetch(addr);
    let a_reg = cpu.registers.a_reg;
    let res = (a_reg as u16).wrapping_sub(val as u16);

    cpu.registers.set_c_from_bool(a_reg >= val);
    cpu.registers.set_n_from_bool((res & 0x80) != 0);
    cpu.registers.set_z_from_bool(a_reg == val);
    5 + boundary
}
