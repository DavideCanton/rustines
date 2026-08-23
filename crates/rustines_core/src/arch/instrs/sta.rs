use crate::arch::{bus::Bus, cpu::Cpu};

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);
    bus.write(addr as u16, cpu.registers.a_reg);
    3
}

pub fn zeropage_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage_indexed(bus, cpu.registers.x_reg);
    bus.write(addr as u16, cpu.registers.a_reg);
    4
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    bus.write(addr, cpu.registers.a_reg);
    4
}

pub fn absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu
        .decode_absolute_indexed(bus, cpu.registers.x_reg, true)
        .address();
    bus.write(addr, cpu.registers.a_reg);
    5
}

pub fn absolute_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu
        .decode_absolute_indexed(bus, cpu.registers.y_reg, true)
        .address();
    bus.write(addr, cpu.registers.a_reg);
    5
}

pub fn indirect_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_indexed_indirect(bus);
    bus.write(addr, cpu.registers.a_reg);
    6
}

pub fn indirect_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_indirect_indexed(bus, true).address();
    bus.write(addr, cpu.registers.a_reg);
    6
}
