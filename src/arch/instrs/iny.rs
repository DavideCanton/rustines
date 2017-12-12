use arch::cpu::CPU;

pub fn implied(cpu: &mut CPU) -> (u8, u8) {
    let mut val = cpu.registers.y_reg;
    val = val.wrapping_add(1);
    cpu.registers.y_reg = val;

    cpu.registers.compute_nz_flags(val);
    (2, 1)
}
