use crate::arch::cpu::CPU;

pub fn pha(cpu: &mut CPU) -> (u8, u8) {
    let a = cpu.registers.a_reg;
    cpu.push8(a);
    (3, 1)
}

pub fn php(cpu: &mut CPU) -> (u8, u8) {
    let p = cpu.registers.get_p();
    cpu.push8(p);
    (3, 1)
}

pub fn pla(cpu: &mut CPU) -> (u8, u8) {
    let a = cpu.pop8();
    cpu.registers.a_reg = a;
    cpu.registers.compute_nz_flags(a);
    (4, 1)
}

pub fn plp(cpu: &mut CPU) -> (u8, u8) {
    let p = cpu.pop8();
    cpu.registers.set_p(p);
    (4, 1)
}
