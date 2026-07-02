use crate::arch::{bus::{Bus, FetchStore}, cpu::Cpu};

pub fn accumulator(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    let mut val = cpu.registers.a_reg;
    cpu.registers.set_c_from_bool(val & 0x1 != 0);
    val = (val >> 1) & 0x7f;
    cpu.registers.a_reg = val;
    cpu.registers.compute_nz_flags(val);
    cpu.registers.clear_n();
    2
}

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);

    let mut val = bus.fetch(addr as u16);
    cpu.registers.set_c_from_bool(val & 0x1 != 0);
    val = (val >> 1) & 0x7f;
    cpu.registers.compute_nz_flags(val);
    cpu.registers.clear_n();
    bus.store(addr as u16, val);
    5
}

pub fn zeropage_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage_indexed(bus,cpu.registers.x_reg);

    let mut val = bus.fetch(addr as u16);
    cpu.registers.set_c_from_bool(val & 0x1 != 0);
    val = (val >> 1) & 0x7f;
    cpu.registers.compute_nz_flags(val);
    cpu.registers.clear_n();
    bus.store(addr as u16, val);
    6
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);

    let mut val = bus.fetch(addr);
    cpu.registers.set_c_from_bool(val & 0x1 != 0);
    val = (val >> 1) & 0x7f;
    cpu.registers.compute_nz_flags(val);
    cpu.registers.clear_n();
    bus.store(addr, val);
    6
}

pub fn absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, _) = cpu.decode_absolute_indexed(bus,cpu.registers.x_reg);

    let mut val = bus.fetch(addr);
    cpu.registers.set_c_from_bool(val & 0x1 != 0);
    val = (val >> 1) & 0x7f;
    cpu.registers.compute_nz_flags(val);
    cpu.registers.clear_n();
    bus.store(addr, val);
    7
}
