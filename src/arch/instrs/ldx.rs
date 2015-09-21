use arch::cpu::CPU;
use utils::bit_utils::*;

pub fn immediate(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_immediate!(cpu);
    cpu.registers.X = addr;
    cpu.registers.compute_NZ_flags(addr);
    (2, ilen)
}

pub fn zeropage(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_zeropage!(cpu);
    cpu.registers.X = cpu.memory.borrow().fetch(addr as u16);
    let xval = cpu.registers.X;
    cpu.registers.compute_NZ_flags(xval);
    (3, ilen)
}

pub fn zeropage_y(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_zeropage_indexed!(cpu, cpu.registers.Y);
    cpu.registers.X = cpu.memory.borrow().fetch(addr as u16);
    let xval = cpu.registers.X;
    cpu.registers.compute_NZ_flags(xval);
    (4, ilen)
}

pub fn absolute(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_absolute!(cpu);
    cpu.registers.X = cpu.memory.borrow().fetch(addr);
    let xval = cpu.registers.X;
    cpu.registers.compute_NZ_flags(xval);
    (4, ilen)
}

pub fn absolute_y(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_absolute_indexed!(cpu, cpu.registers.Y);
    cpu.registers.X = cpu.memory.borrow().fetch(addr);
    let xval = cpu.registers.X;
    cpu.registers.compute_NZ_flags(xval);
    (4, ilen)
    //TODO +1 if page boundary
}
