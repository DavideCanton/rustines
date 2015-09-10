use arch::cpu::CPU;
use arch::instr_table::{decode_zeropage, decode_zeropage_indexed, decode_absolute,
    decode_absolute_indexed};


pub fn zeropage(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_zeropage(cpu);

    let mut memory = cpu.memory.borrow_mut();
    let mut val = memory.fetch(addr as u16);
    val = val.wrapping_add(1);
    memory.store(addr as u16, val);
    cpu.registers.compute_NZ_flags(val);

    (5, ilen)
}

pub fn zeropage_x(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_zeropage_indexed(cpu, cpu.registers.X);

    let mut memory = cpu.memory.borrow_mut();
    let mut val = memory.fetch(addr as u16);
    val = val.wrapping_add(1);
    memory.store(addr as u16, val);
    cpu.registers.compute_NZ_flags(val);

    (6, ilen)
}

pub fn absolute(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_absolute(cpu);

    let mut memory = cpu.memory.borrow_mut();
    let mut val = memory.fetch(addr as u16);
    val = val.wrapping_add(1);
    memory.store(addr, val);

    cpu.registers.compute_NZ_flags(val);
    (6, ilen)
}

pub fn absolute_x(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_absolute_indexed(cpu, cpu.registers.X);

    let mut memory = cpu.memory.borrow_mut();
    let mut val = memory.fetch(addr as u16);
    val = val.wrapping_add(1);
    memory.store(addr, val);

    cpu.registers.compute_NZ_flags(val);
    (7, ilen)
}
