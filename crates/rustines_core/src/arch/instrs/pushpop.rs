use crate::arch::{bus::Bus, cpu::Cpu};

pub fn pha(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.burn_internal_cycle(bus);
    let a = cpu.registers.a_reg;
    cpu.push8(bus, a);
    3
}

pub fn php(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.burn_internal_cycle(bus);
    let p = cpu.registers.get_p(true);
    cpu.push8(bus, p);
    3
}

pub fn pla(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.burn_internal_cycle(bus);
    let a = cpu.pop8(bus);
    cpu.burn_internal_cycle(bus);
    cpu.registers.a_reg = a;
    cpu.registers.compute_nz_flags(a);
    4
}

pub fn plp(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.burn_internal_cycle(bus);
    let p = cpu.pop8(bus);
    cpu.burn_internal_cycle(bus);
    cpu.registers.set_p(p, true);
    4
}
