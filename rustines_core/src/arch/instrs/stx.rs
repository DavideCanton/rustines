use crate::arch::{cpu::Cpu, memory::FetchStore};

pub fn zeropage(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_zeropage();
    cpu.memory.store(addr as u16, cpu.registers.x_reg);
    3
}

pub fn zeropage_y(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_zeropage_indexed(cpu.registers.y_reg);
    cpu.memory.store(addr as u16, cpu.registers.x_reg);
    4
}

pub fn absolute(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_absolute();
    cpu.memory.store(addr, cpu.registers.x_reg);
    4
}
