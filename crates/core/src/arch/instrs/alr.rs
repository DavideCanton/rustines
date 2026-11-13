use crate::arch::{bus::Bus, cpu::Cpu, instrs::lsr::do_lsr};

pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let val = cpu.decode_immediate(bus);
    let res = cpu.registers.a_reg & val;
    do_lsr(cpu, res);
    2
}
