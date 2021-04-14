use crate::arch::cpu::Cpu;

pub fn implied(cpu: &mut Cpu) -> (u8, u8) {
    let mut val = cpu.registers.y_reg;
    val = val.wrapping_add(1);
    cpu.registers.y_reg = val;

    cpu.registers.compute_nz_flags(val);
    (2, 1)
}
