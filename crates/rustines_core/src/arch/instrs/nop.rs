use crate::arch::bus::Bus;
use crate::arch::cpu::Cpu;

pub fn implied(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.burn_internal_cycle(bus);
    2
}

pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let _ = bus.fetch(cpu.registers.pc);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    2
}

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);
    let _ = bus.fetch(addr as u16);
    3
}

pub fn zeropage_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage_indexed(bus, cpu.registers.x_reg);
    let _ = bus.fetch(addr as u16);
    4
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    let _ = bus.fetch(addr);
    4
}

pub fn absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, boundary) = cpu.decode_absolute_indexed(bus, cpu.registers.x_reg, false);
    let _ = bus.fetch(addr);
    4 + boundary
}
