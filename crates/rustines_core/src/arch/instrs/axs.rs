use crate::arch::{bus::Bus, cpu::Cpu};

pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let val = cpu.decode_immediate(bus);
    let base = cpu.registers.a_reg & cpu.registers.x_reg;
    let res = base.wrapping_sub(val);
    cpu.registers.x_reg = res;
    cpu.registers.set_c_from_bool(base >= val);
    cpu.registers.compute_nz_flags(res);
    2
}
