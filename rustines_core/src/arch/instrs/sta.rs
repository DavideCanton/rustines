use crate::arch::{cpu::Cpu, memory::FetchStore};

pub fn zeropage(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_zeropage();
    cpu.memory.store(addr as u16, cpu.registers.a_reg);
    3
}

pub fn zeropage_x(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_zeropage_indexed(cpu.registers.x_reg);
    cpu.memory.store(addr as u16, cpu.registers.a_reg);
    4
}

pub fn absolute(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_absolute();
    cpu.memory.store(addr, cpu.registers.a_reg);
    4
}

pub fn absolute_x(cpu: &mut Cpu) -> u8 {
    let (addr, _) = cpu.decode_absolute_indexed(cpu.registers.x_reg);
    cpu.memory.store(addr, cpu.registers.a_reg);
    5
}

pub fn absolute_y(cpu: &mut Cpu) -> u8 {
    let (addr, _) = cpu.decode_absolute_indexed(cpu.registers.y_reg);
    cpu.memory.store(addr, cpu.registers.a_reg);
    5
}

pub fn indirect_x(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_indexed_indirect();
    cpu.memory.store(addr, cpu.registers.a_reg);
    6
}

pub fn indirect_y(cpu: &mut Cpu) -> u8 {
    let (addr, _) = cpu.decode_indirect_indexed();
    cpu.memory.store(addr, cpu.registers.a_reg);
    6
}
