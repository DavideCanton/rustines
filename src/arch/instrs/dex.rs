use crate::arch::cpu::CPU;

pub fn implied(cpu: &mut CPU) -> (u8, u8) {
    let mut val = cpu.registers.x_reg;
    val = val.wrapping_sub(1);
    cpu.registers.x_reg = val;

    cpu.registers.compute_nz_flags(val);
    (2, 1)
}
