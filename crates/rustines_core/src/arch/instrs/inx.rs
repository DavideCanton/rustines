use crate::arch::{bus::Bus, cpu::Cpu};

pub fn implied(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.burn_internal_cycle(bus);

    let mut val = cpu.registers.x_reg;
    val = val.wrapping_add(1);
    cpu.registers.x_reg = val;

    cpu.registers.compute_nz_flags(val);
    2
}
