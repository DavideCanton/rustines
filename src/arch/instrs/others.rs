use crate::arch::cpu::Cpu;
use crate::utils::bit_utils::*;

pub fn nop(_: &mut Cpu) -> (u8, u8) {
    // why I'm implementing this?
    (2, 1)
}

pub fn brk(cpu: &mut Cpu) -> (u8, u8) {
    let pc = cpu.registers.pc;
    cpu.push16(pc + 1);
    cpu.registers.set_b();
    let p = cpu.registers.get_p();
    cpu.push8(p);
    let l = cpu.memory.fetch(0xFFFE);
    let h = cpu.memory.fetch(0xFFFF);
    
    cpu.registers.pc = to_u16(l, h);
   
    (7, 0)
}