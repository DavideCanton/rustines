use crate::arch::{bus::Bus, cpu::Cpu, instrs::and::do_and};

pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let val = cpu.decode_immediate(bus);
    let res = do_and(cpu, val);
    cpu.registers.set_c_from_bool(res & 0b1000_0000 != 0);
    2
}
