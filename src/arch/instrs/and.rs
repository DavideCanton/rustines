use arch::cpu::CPU;
use utils::bit_utils::*;

pub fn immediate(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_immediate!(cpu);
    let res = cpu.registers.A & addr;
    cpu.registers.A = res;
    cpu.registers.compute_NZ_flags(res);
    (2, ilen)
}

pub fn zeropage(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_zeropage!(cpu);
    let addr = cpu.memory.borrow().fetch(addr as u16);
    let res = cpu.registers.A & addr;
    cpu.registers.A = res;
    cpu.registers.compute_NZ_flags(res);
    (3, ilen)
}

pub fn zeropage_x(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_zeropage_indexed!(cpu, cpu.registers.X);
    let addr = cpu.memory.borrow().fetch(addr as u16);
    let res = cpu.registers.A & addr;
    cpu.registers.A = res;
    cpu.registers.compute_NZ_flags(res);
    (4, ilen)
}

pub fn absolute(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_absolute!(cpu);
    let addr = cpu.memory.borrow().fetch(addr);
    let res = cpu.registers.A & addr;
    cpu.registers.A = res;
    cpu.registers.compute_NZ_flags(res);
    (4, ilen)
}

pub fn absolute_x(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_absolute_indexed!(cpu, cpu.registers.X);
    let addr = cpu.memory.borrow().fetch(addr);
    let res = cpu.registers.A & addr;
    cpu.registers.A = res;
    cpu.registers.compute_NZ_flags(res);
    (4, ilen)
    // TODO +1 if page boundary
}

pub fn absolute_y(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_absolute_indexed!(cpu, cpu.registers.Y);
    let addr = cpu.memory.borrow().fetch(addr);
    let res = cpu.registers.A & addr;
    cpu.registers.A = res;
    cpu.registers.compute_NZ_flags(res);
    (4, ilen)
    // TODO +1 if page boundary
}

pub fn indirect_x(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_indexed_indirect!(cpu);
    let addr = cpu.memory.borrow().fetch(addr);
    let res = cpu.registers.A & addr;
    cpu.registers.A = res;
    cpu.registers.compute_NZ_flags(res);
    (6, ilen)
}

pub fn indirect_y(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_indirect_indexed!(cpu);
    let addr = cpu.memory.borrow().fetch(addr);
    let res = cpu.registers.A & addr;
    cpu.registers.A = res;
    cpu.registers.compute_NZ_flags(res);
    (5, ilen)
    // TODO +1 if page boundary
}
