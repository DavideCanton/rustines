use arch::cpu::CPU;
use utils::bit_utils::*;

pub fn zeropage(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_zeropage!(cpu);
    let addr = cpu.memory.borrow().fetch(addr as u16);
    let res = addr & cpu.registers.A;

    if res & 0x80 != 0 { cpu.registers.setN() } else { cpu.registers.clearN() }
    if res & 0x40 != 0 { cpu.registers.setV() } else { cpu.registers.clearV() }
    if res == 0 { cpu.registers.setZ() } else { cpu.registers.clearZ() }

    (3, ilen)
}

pub fn absolute(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, ilen) = decode_absolute!(cpu);
    let addr = cpu.memory.borrow().fetch(addr);
    let res = addr & cpu.registers.A;

    if res & 0x80 != 0 { cpu.registers.setN() } else { cpu.registers.clearN() }
    if res & 0x40 != 0 { cpu.registers.setV() } else { cpu.registers.clearV() }
    if res == 0 { cpu.registers.setZ() } else { cpu.registers.clearZ() }

    (4, ilen)
}
