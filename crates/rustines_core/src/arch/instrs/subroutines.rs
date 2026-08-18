use crate::{
    arch::{bus::Bus, cpu::Cpu},
    utils::bit_utils::to_u16,
};

pub fn jsr(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let low = bus.fetch(cpu.registers.pc);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

    cpu.burn_internal_cycle(bus);

    let return_addr = cpu.registers.pc;

    cpu.push16(bus, return_addr);

    let high = bus.fetch(cpu.registers.pc);

    cpu.registers.pc = to_u16(low, high);

    6
}

pub fn rts(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.burn_internal_cycle(bus);
    cpu.burn_internal_cycle(bus);
    let return_addr = cpu.pop16(bus);
    cpu.burn_internal_cycle(bus);
    cpu.registers.pc = return_addr.wrapping_add(1);
    6
}

pub fn rti(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.burn_internal_cycle(bus);
    let p = cpu.pop8(bus);
    cpu.registers.set_p(p, true);
    let pc = cpu.pop16(bus);
    cpu.registers.pc = pc;
    cpu.burn_internal_cycle(bus);
    6
}
