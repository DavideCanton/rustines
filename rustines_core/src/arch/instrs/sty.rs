use crate::arch::{cpu::Cpu, memory::FetchStore};

pub fn zeropage(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_zeropage();
    cpu.memory.store(addr as u16, cpu.registers.y_reg);
    (3, ilen)
}

pub fn zeropage_x(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_zeropage_indexed(cpu.registers.x_reg);
    cpu.memory.store(addr as u16, cpu.registers.y_reg);
    (4, ilen)
}

pub fn absolute(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_absolute();
    cpu.memory.store(addr, cpu.registers.y_reg);
    (4, ilen)
}
