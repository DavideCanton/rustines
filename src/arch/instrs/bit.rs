use crate::arch::cpu::CPU;
use crate::utils::bit_utils::*;

pub fn zeropage(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_zeropage!(cpu);
    let addr = cpu.memory.fetch(addr as u16);
    let res = addr & cpu.registers.a_reg;

    if res & 0x80 != 0 {
        cpu.registers.set_n()
    } else {
        cpu.registers.clear_n()
    }
    if res & 0x40 != 0 {
        cpu.registers.set_v()
    } else {
        cpu.registers.clear_v()
    }
    if res == 0 {
        cpu.registers.set_z()
    } else {
        cpu.registers.clear_z()
    }

    (3, ilen)
}

pub fn absolute(cpu: &mut CPU) -> (u8, u8) {
    let (addr, ilen) = decode_absolute!(cpu);
    let addr = cpu.memory.fetch(addr);
    let res = addr & cpu.registers.a_reg;

    if res & 0x80 != 0 {
        cpu.registers.set_n()
    } else {
        cpu.registers.clear_n()
    }
    if res & 0x40 != 0 {
        cpu.registers.set_v()
    } else {
        cpu.registers.clear_v()
    }
    if res == 0 {
        cpu.registers.set_z()
    } else {
        cpu.registers.clear_z()
    }

    (4, ilen)
}
