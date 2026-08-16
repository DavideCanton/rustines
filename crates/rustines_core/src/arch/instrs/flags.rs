use crate::arch::{bus::Bus, cpu::Cpu};

pub fn clc(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.burn_internal_cycle(bus);
    cpu.registers.clear_c();
    2
}

pub fn cld(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.burn_internal_cycle(bus);
    cpu.registers.clear_d();
    2
}

pub fn cli(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.burn_internal_cycle(bus);
    cpu.registers.clear_i();
    2
}

pub fn clv(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.burn_internal_cycle(bus);
    cpu.registers.clear_v();
    2
}

pub fn sec(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.burn_internal_cycle(bus);
    cpu.registers.set_c();
    2
}

pub fn sed(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.burn_internal_cycle(bus);
    cpu.registers.set_d();
    2
}

pub fn sei(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.burn_internal_cycle(bus);
    cpu.registers.set_i();
    2
}
