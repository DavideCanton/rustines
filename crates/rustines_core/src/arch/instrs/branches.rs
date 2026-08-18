use crate::arch::{bus::Bus, cpu::Cpu};

pub fn bcc(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    do_branch(!cpu.registers.get_c(), cpu, bus)
}

pub fn bcs(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    do_branch(cpu.registers.get_c(), cpu, bus)
}

pub fn beq(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    do_branch(cpu.registers.get_z(), cpu, bus)
}

pub fn bne(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    do_branch(!cpu.registers.get_z(), cpu, bus)
}

pub fn bmi(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    do_branch(cpu.registers.get_n(), cpu, bus)
}

pub fn bpl(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    do_branch(!cpu.registers.get_n(), cpu, bus)
}

pub fn bvs(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    do_branch(cpu.registers.get_v(), cpu, bus)
}

pub fn bvc(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    do_branch(!cpu.registers.get_v(), cpu, bus)
}

fn do_branch(flag: bool, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = bus.fetch(cpu.registers.pc) as i8 as i16;
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

    if flag {
        let old_pc = cpu.registers.pc;
        let new_pc = old_pc.wrapping_add(offset as u16);

        cpu.burn_internal_cycle(bus);
        cpu.registers.pc = new_pc;

        let page_crossed = (old_pc & 0xFF00) != (new_pc & 0xFF00);
        if page_crossed {
            cpu.burn_internal_cycle(bus);
            4
        } else {
            3
        }
    } else {
        2
    }
}
