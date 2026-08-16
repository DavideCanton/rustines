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
    if flag {
        let addr = cpu.decode_zeropage(bus) as i8;
        let old_pc = cpu.registers.pc;
        let new_pc = old_pc.wrapping_add(addr as u16);
        cpu.burn_internal_cycle(bus);
        cpu.registers.pc = new_pc;
        let page_crossed = (old_pc & 0xFF00) != (new_pc & 0xFF00);
        if page_crossed {
            cpu.burn_internal_cycle(bus);
        }
        if page_crossed { 4 } else { 3 }
    } else {
        let _offset = bus.fetch(cpu.registers.pc - 1);
        2
    }
}
