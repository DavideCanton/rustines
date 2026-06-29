use crate::arch::{cpu::Cpu, memory::FetchStore};

pub fn accumulator(cpu: &mut Cpu) -> u8 {
    let mut val = cpu.registers.a_reg;
    cpu.registers.set_c_from_bool(val & 0x80 != 0);
    val <<= 1;
    cpu.registers.a_reg = val;
    cpu.registers.compute_nz_flags(val);
    2
}

pub fn zeropage(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_zeropage();

    let mut val = cpu.memory.fetch(addr as u16);
    cpu.registers.set_c_from_bool(val & 0x80 != 0);
    val <<= 1;
    cpu.registers.compute_nz_flags(val);
    cpu.memory.store(addr as u16, val);
    5
}

pub fn zeropage_x(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_zeropage_indexed(cpu.registers.x_reg);

    let mut val = cpu.memory.fetch(addr as u16);
    cpu.registers.set_c_from_bool(val & 0x80 != 0);
    val <<= 1;
    cpu.registers.compute_nz_flags(val);
    cpu.memory.store(addr as u16, val);
    6
}

pub fn absolute(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_absolute();

    let mut val = cpu.memory.fetch(addr);
    cpu.registers.set_c_from_bool(val & 0x80 != 0);
    val <<= 1;
    cpu.registers.compute_nz_flags(val);
    cpu.memory.store(addr, val);
    6
}

pub fn absolute_x(cpu: &mut Cpu) -> u8 {
    let (addr, _) = cpu.decode_absolute_indexed(cpu.registers.x_reg);

    let mut val = cpu.memory.fetch(addr);
    cpu.registers.set_c_from_bool(val & 0x80 != 0);
    val <<= 1;
    cpu.registers.compute_nz_flags(val);
    cpu.memory.store(addr, val);
    7
}
