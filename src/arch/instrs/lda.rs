use arch::cpu::CPU;
use arch::instr_table::{decode_immediate, decode_zeropage, decode_zeropage_indexed,
    decode_absolute, decode_absolute_indexed, decode_indexed_indirect, decode_indirect_indexed};

pub fn immediate(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_immediate(cpu);
    cpu.registers.A = addr;
    (2, ilen)
}

pub fn zeropage(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_zeropage(cpu);
    cpu.registers.A = cpu.memory.borrow().fetch(addr as u16);
    (3, ilen)
}

pub fn zeropage_x(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_zeropage_indexed(cpu, cpu.registers.X);
    cpu.registers.A = cpu.memory.borrow().fetch(addr as u16);
    (4, ilen)
}

pub fn absolute(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_absolute(cpu);
    cpu.registers.A = cpu.memory.borrow().fetch(addr as u16);
    (4, ilen)
}

pub fn absolute_x(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_absolute_indexed(cpu, cpu.registers.X);
    cpu.registers.A = cpu.memory.borrow().fetch(addr as u16);
    (4, ilen)
    //TODO +1 if page boundary
}

pub fn absolute_y(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_absolute_indexed(cpu, cpu.registers.Y);
    cpu.registers.A = cpu.memory.borrow().fetch(addr as u16);
    (4, ilen)
    //TODO +1 if page boundary
}

pub fn indirect_x(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_indexed_indirect(cpu);
    cpu.registers.A = cpu.memory.borrow().fetch(addr as u16);
    (6, ilen)
}

pub fn indirect_y(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_indirect_indexed(cpu);
    cpu.registers.A = cpu.memory.borrow().fetch(addr as u16);
    (5, ilen)
    //TODO +1 if page boundary
}
