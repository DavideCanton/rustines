use arch::cpu::CPU;
use utils::bit_utils::*;

pub fn zeropage(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_zeropage!(cpu);
    cpu.memory.borrow_mut().store(addr as u16, cpu.registers.A);
    (3, ilen)
}

pub fn zeropage_x(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_zeropage_indexed!(cpu, cpu.registers.X);
    cpu.memory.borrow_mut().store(addr as u16, cpu.registers.A);
    (4, ilen)
}

pub fn absolute(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_absolute!(cpu);
    cpu.memory.borrow_mut().store(addr, cpu.registers.A);
    (4, ilen)
}

pub fn absolute_x(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_absolute_indexed!(cpu, cpu.registers.X);
    cpu.memory.borrow_mut().store(addr, cpu.registers.A);
    (5, ilen)
}

pub fn absolute_y(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_absolute_indexed!(cpu, cpu.registers.Y);
    cpu.memory.borrow_mut().store(addr, cpu.registers.A);
    (5, ilen)
}

pub fn indirect_x(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_indexed_indirect!(cpu);
    cpu.memory.borrow_mut().store(addr, cpu.registers.A);
    (6, ilen)
}

pub fn indirect_y(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_indirect_indexed!(cpu);
    cpu.memory.borrow_mut().store(addr, cpu.registers.A);
    (6, ilen)
}
