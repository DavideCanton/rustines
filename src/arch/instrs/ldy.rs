use arch::cpu::CPU;
use arch::instr_table::{decode_immediate, decode_zeropage, decode_zeropage_indexed,
    decode_absolute, decode_absolute_indexed};

pub fn immediate(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_immediate(cpu);
    cpu.registers.Y = addr;
    cpu.registers.compute_NZ_flags(addr);
    (2, ilen)
}

pub fn zeropage(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_zeropage(cpu);
    cpu.registers.Y = cpu.memory.borrow().fetch(addr as u16);
    cpu.registers.compute_NZ_flags(addr);
    (3, ilen)
}

pub fn zeropage_x(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_zeropage_indexed(cpu, cpu.registers.X);
    cpu.registers.Y = cpu.memory.borrow().fetch(addr as u16);
    cpu.registers.compute_NZ_flags(addr);
    (4, ilen)
}

pub fn absolute(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_absolute(cpu);
    cpu.registers.Y = cpu.memory.borrow().fetch(addr);
    cpu.registers.compute_NZ_flags(addr as u8);
    (4, ilen)
}

pub fn absolute_x(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_absolute_indexed(cpu, cpu.registers.X);
    cpu.registers.Y = cpu.memory.borrow().fetch(addr);
    cpu.registers.compute_NZ_flags(addr as u8);
    (4, ilen)
    //TODO +1 if page boundary
}
