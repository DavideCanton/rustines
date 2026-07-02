use crate::arch::{bus::{Bus, FetchStore}, cpu::Cpu};

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);
    bus.store(addr as u16, cpu.registers.y_reg);
    3
}

pub fn zeropage_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage_indexed(bus,cpu.registers.x_reg);
    bus.store(addr as u16, cpu.registers.y_reg);
    4
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    bus.store(addr, cpu.registers.y_reg);
    4
}
