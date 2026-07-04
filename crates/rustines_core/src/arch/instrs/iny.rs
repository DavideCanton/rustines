use crate::arch::{bus::Bus, cpu::Cpu};

pub fn implied(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    let mut val = cpu.registers.y_reg;
    val = val.wrapping_add(1);
    cpu.registers.y_reg = val;

    cpu.registers.compute_nz_flags(val);
    2
}
