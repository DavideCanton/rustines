use crate::arch::{bus::Bus, cpu::Cpu};

pub fn jsr(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    let t = cpu.registers.pc - 1; // next instr address
    cpu.push16(bus, t);
    cpu.registers.pc = addr;

    6
}

pub fn rts(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let v = cpu.pop16(bus);
    cpu.registers.pc = v + 1;
    6
}

pub fn rti(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let p = cpu.pop8(bus);
    cpu.registers.set_p(p);
    let pc = cpu.pop16(bus);
    cpu.registers.pc = pc;
    6
}
