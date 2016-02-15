use arch::cpu::CPU;
use utils::bit_utils::*;

pub fn nop(_: &mut CPU) -> (u8, u8) {
    // why I'm implementing this?
    (2, 1)
}

pub fn brk(cpu: &mut CPU) -> (u8, u8) {
    let pc = cpu.registers.PC;
    cpu.push16(pc + 1);
    cpu.registers.setB();
    let p = cpu.registers.getP();
    cpu.push8(p);
    let l = cpu.memory.fetch(0xFFFE);
    let h = cpu.memory.fetch(0xFFFF);
    
    cpu.registers.PC = to_u16(l, h);
   
    (7, 0)
}