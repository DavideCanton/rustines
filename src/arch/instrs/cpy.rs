use arch::cpu::CPU;
use utils::bit_utils::*;

pub fn immediate(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_immediate!(cpu);
    let res = cpu.registers.Y.wrapping_sub(addr);
    cpu.registers.compute_NZ_flags(res);
    if res & 0x80 == 0
    {
        cpu.registers.setC();
    }
    else
    {
        cpu.registers.clearC();
    }
    (2, ilen)
}

pub fn zeropage(cpu: &mut CPU) -> (u8, u8)
{
    let (mut addr, ilen) = decode_zeropage!(cpu);
    addr = cpu.memory.borrow().fetch(addr as u16);
    let res = cpu.registers.Y.wrapping_sub(addr);
    cpu.registers.compute_NZ_flags(res);
    if res & 0x80 == 0
    {
        cpu.registers.setC();
    }
    else
    {
        cpu.registers.clearC();
    }
    (3, ilen)
}

pub fn absolute(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_absolute!(cpu);
    let addr = cpu.memory.borrow().fetch(addr);
    let res = cpu.registers.Y.wrapping_sub(addr);
    cpu.registers.compute_NZ_flags(res);
    if res & 0x80 == 0
    {
        cpu.registers.setC();
    }
    else
    {
        cpu.registers.clearC();
    }
    (4, ilen)
}
