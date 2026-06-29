use crate::arch::cpu::Cpu;

pub fn jsr(cpu: &mut Cpu) -> u8 {
    let addr = cpu.decode_absolute();
    let t = cpu.registers.pc; // next instr address
    cpu.push16(t);
    cpu.registers.pc = addr;

    6
}

pub fn rts(cpu: &mut Cpu) -> u8 {
    let v = cpu.pop16();
    cpu.registers.pc = v + 1;
    6
}

pub fn rti(cpu: &mut Cpu) -> u8 {
    let p = cpu.pop8();
    cpu.registers.set_p(p);
    let pc = cpu.pop16();
    cpu.registers.pc = pc;
    6
}
