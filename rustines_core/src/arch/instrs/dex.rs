use crate::arch::{bus::Bus, cpu::Cpu};

pub fn implied(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let _ = bus;
    let mut val = cpu.registers.x_reg;
    val = val.wrapping_sub(1);
    cpu.registers.x_reg = val;

    cpu.registers.compute_nz_flags(val);
    2
}
