use crate::arch::{
    bus::{Bus, FetchStore},
    cpu::Cpu,
};

pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_immediate(bus);
    let res = cpu.registers.x_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    cpu.registers.set_c_from_bool(res & 0x80 == 0);
    2
}

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let mut addr = cpu.decode_zeropage(bus);
    addr = bus.fetch(addr as u16);
    let res = cpu.registers.x_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    cpu.registers.set_c_from_bool(res & 0x80 == 0);
    3
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    let addr = bus.fetch(addr);
    let res = cpu.registers.x_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    cpu.registers.set_c_from_bool(res & 0x80 == 0);
    4
}
