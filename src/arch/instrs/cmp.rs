use arch::cpu::CPU;
use arch::instrs::instr_table::{decode_immediate, decode_zeropage, decode_zeropage_indexed,
    decode_absolute, decode_absolute_indexed, decode_indexed_indirect, decode_indirect_indexed};

pub fn immediate(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_immediate(cpu);
    let res = cpu.registers.A.wrapping_sub(addr);
    cpu.registers.compute_NZ_flags(res);
    cpu.registers.compute_C_flag(res & 0x80 == 0);
    (2, ilen)
}

pub fn zeropage(cpu: &mut CPU) -> (u8, u8)
{
    let (mut addr, ilen) = decode_zeropage(cpu);
    addr = cpu.memory.borrow().fetch(addr as u16);
    let res = cpu.registers.A.wrapping_sub(addr);
    cpu.registers.compute_NZ_flags(res);
    cpu.registers.compute_C_flag(res & 0x80 == 0);
    (3, ilen)
}

pub fn zeropage_x(cpu: &mut CPU) -> (u8, u8)
{
    let (mut addr, ilen) = decode_zeropage_indexed(cpu, cpu.registers.X);
    addr = cpu.memory.borrow().fetch(addr as u16);
    let res = cpu.registers.A.wrapping_sub(addr);
    cpu.registers.compute_NZ_flags(res);
    cpu.registers.compute_C_flag(res & 0x80 == 0);
    (4, ilen)
}

pub fn absolute(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_absolute(cpu);
    let addr = cpu.memory.borrow().fetch(addr);
    let res = cpu.registers.A.wrapping_sub(addr);
    cpu.registers.compute_NZ_flags(res);
    cpu.registers.compute_C_flag(res & 0x80 == 0);
    (4, ilen)
}

pub fn absolute_x(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_absolute_indexed(cpu, cpu.registers.X);
    let addr = cpu.memory.borrow().fetch(addr);
    let res = cpu.registers.A.wrapping_sub(addr);
    cpu.registers.compute_NZ_flags(res);
    cpu.registers.compute_C_flag(res & 0x80 == 0);
    (4, ilen)
    //TODO +1 if page boundary
}

pub fn absolute_y(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_absolute_indexed(cpu, cpu.registers.Y);
    let addr = cpu.memory.borrow().fetch(addr);
    let res = cpu.registers.A.wrapping_sub(addr);
    cpu.registers.compute_NZ_flags(res);
    cpu.registers.compute_C_flag(res & 0x80 == 0);
    (4, ilen)
    //TODO +1 if page boundary
}

pub fn indirect_x(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_indexed_indirect(cpu);
    let addr = cpu.memory.borrow().fetch(addr);
    let res = cpu.registers.A.wrapping_sub(addr);
    cpu.registers.compute_NZ_flags(res);
    cpu.registers.compute_C_flag(res & 0x80 == 0);
    (6, ilen)
}

pub fn indirect_y(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_indirect_indexed(cpu);
    let addr = cpu.memory.borrow().fetch(addr);
    let res = cpu.registers.A.wrapping_sub(addr);
    cpu.registers.compute_NZ_flags(res);
    cpu.registers.compute_C_flag(res & 0x80 == 0);
    (5, ilen)
    //TODO +1 if page boundary
}
