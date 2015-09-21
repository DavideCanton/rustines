use arch::cpu::CPU;
use utils::bit_utils::to_u16;

pub fn absolute(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, _ilen) = decode_absolute!(cpu);
    cpu.registers.PC = addr;
    (3, 0)
}

pub fn indirect_absolute(cpu: &mut CPU) -> (u8, u8)
{
    let (addr, _ilen) = decode_absolute!(cpu);
    let low = cpu.memory.borrow().fetch(addr);
    let high = cpu.memory.borrow().fetch(addr + 1);
    cpu.registers.PC = to_u16(low, high);
    (5, 0)
}
