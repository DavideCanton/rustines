use crate::arch::{bus::{Bus, FetchStore}, cpu::Cpu};

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);

    let mut val = bus.fetch(addr as u16);
    val = val.wrapping_sub(1);
    bus.store(addr as u16, val);
    cpu.registers.compute_nz_flags(val);

    5
}

pub fn zeropage_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage_indexed(bus,cpu.registers.x_reg);

    let mut val = bus.fetch(addr as u16);
    val = val.wrapping_sub(1);
    bus.store(addr as u16, val);
    cpu.registers.compute_nz_flags(val);

    6
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);

    let mut val = bus.fetch(addr);
    val = val.wrapping_sub(1);
    bus.store(addr, val);

    cpu.registers.compute_nz_flags(val);
    6
}

pub fn absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, _) = cpu.decode_absolute_indexed(bus,cpu.registers.x_reg);

    let mut val = bus.fetch(addr);
    val = val.wrapping_sub(1);
    bus.store(addr, val);

    cpu.registers.compute_nz_flags(val);
    7
}
