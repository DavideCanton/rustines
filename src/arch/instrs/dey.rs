use arch::cpu::CPU;

pub fn implied(cpu: &mut CPU) -> (u8, u8) {
    let mut val = cpu.registers.Y;
    val = val.wrapping_sub(1);
    cpu.registers.Y = val;

    cpu.registers.compute_NZ_flags(val);
    (2, 1)
}
