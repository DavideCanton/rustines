use arch::cpu::CPU;

pub fn clc(cpu: &mut CPU) -> (u8, u8) {
    cpu.registers.clearC();
    (2, 1)
}

pub fn cld(cpu: &mut CPU) -> (u8, u8) {
    cpu.registers.clearD();
    (2, 1)
}

pub fn cli(cpu: &mut CPU) -> (u8, u8) {
    cpu.registers.clearI();
    (2, 1)
}

pub fn clv(cpu: &mut CPU) -> (u8, u8) {
    cpu.registers.clearV();
    (2, 1)
}

pub fn sec(cpu: &mut CPU) -> (u8, u8) {
    cpu.registers.setC();
    (2, 1)
}

pub fn sed(cpu: &mut CPU) -> (u8, u8) {
    cpu.registers.setD();
    (2, 1)
}

pub fn sei(cpu: &mut CPU) -> (u8, u8) {
    cpu.registers.setI();
    (2, 1)
}