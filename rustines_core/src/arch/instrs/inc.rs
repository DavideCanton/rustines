use crate::arch::{cpu::Cpu, memory::FetchStore};

pub fn zeropage(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_zeropage();

    let mut val = cpu.memory.fetch(addr as u16);
    val = val.wrapping_add(1);
    cpu.memory.store(addr as u16, val);
    cpu.registers.compute_nz_flags(val);

    5
}

pub fn zeropage_x(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_zeropage_indexed(cpu.registers.x_reg);

    let mut val = cpu.memory.fetch(addr as u16);
    val = val.wrapping_add(1);
    cpu.memory.store(addr as u16, val);
    cpu.registers.compute_nz_flags(val);

    6
}

pub fn absolute(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_absolute();

    let mut val = cpu.memory.fetch(addr);
    val = val.wrapping_add(1);
    cpu.memory.store(addr, val);

    cpu.registers.compute_nz_flags(val);
    6
}

pub fn absolute_x(cpu: &mut Cpu) -> u8 {
    let (addr, _) = cpu.decode_absolute_indexed(cpu.registers.x_reg);

    let mut val = cpu.memory.fetch(addr);
    val = val.wrapping_add(1);
    cpu.memory.store(addr, val);

    cpu.registers.compute_nz_flags(val);
    7
}
