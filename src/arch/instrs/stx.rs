use arch::cpu::CPU;
use utils::bit_utils::*;

pub fn zeropage(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_zeropage!(cpu);
    cpu.memory.borrow_mut().store(addr as u16, cpu.registers.X);
    (3, ilen)
}

pub fn zeropage_y(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_zeropage_indexed!(cpu, cpu.registers.Y);
    cpu.memory.borrow_mut().store(addr as u16, cpu.registers.X);
    (4, ilen)
}

pub fn absolute(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_absolute!(cpu);
    cpu.memory.borrow_mut().store(addr, cpu.registers.X);
    (4, ilen)
}
