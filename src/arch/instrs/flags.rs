use crate::arch::cpu::CPU;

pub fn clc(cpu: &mut CPU) -> (u8, u8) {
    cpu.registers.clear_c();
    (2, 1)
}

pub fn cld(cpu: &mut CPU) -> (u8, u8) {
    cpu.registers.clear_d();
    (2, 1)
}

pub fn cli(cpu: &mut CPU) -> (u8, u8) {
    cpu.registers.clear_i();
    (2, 1)
}

pub fn clv(cpu: &mut CPU) -> (u8, u8) {
    cpu.registers.clear_v();
    (2, 1)
}

pub fn sec(cpu: &mut CPU) -> (u8, u8) {
    cpu.registers.set_c();
    (2, 1)
}

pub fn sed(cpu: &mut CPU) -> (u8, u8) {
    cpu.registers.set_d();
    (2, 1)
}

pub fn sei(cpu: &mut CPU) -> (u8, u8) {
    cpu.registers.set_i();
    (2, 1)
}