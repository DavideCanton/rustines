use arch::cpu::CPU;
use utils::bit_utils::to_u16;

pub fn jsr(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_absolute!(cpu);
    let t = cpu.registers.PC + ilen - 1; // next instr address
    cpu.push16(t);
    cpu.registers.PC = addr;
    
    (6, 0) // this is a jump
}

pub fn rts(cpu: &mut CPU) -> (u8, u8) {
    let v = cpu.pop16();
    cpu.registers.PC = v + 1;
    (6, 1)
}

pub fn rti(cpu: &mut CPU) -> (u8, u8) {
    let p = cpu.pop8();
    cpu.registers.setP(p);
    let pc = cpu.pop16();
    cpu.registers.PC = pc;
    (6, 0)
}