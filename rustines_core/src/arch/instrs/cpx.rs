use crate::arch::{cpu::Cpu, memory::FetchStore};

pub fn immediate(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_immediate();
    let res = cpu.registers.x_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    cpu.registers.set_c_from_bool(res & 0x80 == 0);
    2
}

pub fn zeropage(cpu: &mut Cpu) -> u8 {
    let mut addr = cpu.decode_zeropage();
    addr = cpu.memory.fetch(addr as u16);
    let res = cpu.registers.x_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    cpu.registers.set_c_from_bool(res & 0x80 == 0);
    3
}

pub fn absolute(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_absolute();
    let addr = cpu.memory.fetch(addr);
    let res = cpu.registers.x_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    cpu.registers.set_c_from_bool(res & 0x80 == 0);
    4
}
