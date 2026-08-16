use crate::arch::{bus::Bus, cpu::Cpu, instrs::utils::store_with_dummy_write};

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);
    let val = bus.fetch(addr as u16);
    do_dec(cpu, bus, addr as u16, val);
    5
}

pub fn zeropage_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage_indexed(bus, cpu.registers.x_reg);
    let val = bus.fetch(addr as u16);
    do_dec(cpu, bus, addr as u16, val);
    6
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    let val = bus.fetch(addr);
    do_dec(cpu, bus, addr, val);
    6
}

pub fn absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, _) = cpu.decode_absolute_indexed(bus, cpu.registers.x_reg, true);
    let val = bus.fetch(addr);
    do_dec(cpu, bus, addr, val);
    7
}

fn do_dec(cpu: &mut Cpu, bus: &mut Bus, addr: u16, val: u8) {
    let res = val.wrapping_sub(1);
    store_with_dummy_write(bus, addr, val, res);
    cpu.registers.compute_nz_flags(res);
}
