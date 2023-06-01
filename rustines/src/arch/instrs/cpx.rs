use crate::arch::cpu::Cpu;

pub fn immediate(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_immediate();
    let res = cpu.registers.x_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    if res & 0x80 == 0 {
        cpu.registers.set_c();
    } else {
        cpu.registers.clear_c();
    }
    (2, ilen)
}

pub fn zeropage(cpu: &mut Cpu) -> (u8, u8) {
    let (mut addr, ilen) = cpu.decode_zeropage();
    addr = cpu.memory.fetch(addr as u16);
    let res = cpu.registers.x_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    if res & 0x80 == 0 {
        cpu.registers.set_c();
    } else {
        cpu.registers.clear_c();
    }
    (3, ilen)
}

pub fn absolute(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_absolute();
    let addr = cpu.memory.fetch(addr);
    let res = cpu.registers.x_reg.wrapping_sub(addr);
    cpu.registers.compute_nz_flags(res);
    if res & 0x80 == 0 {
        cpu.registers.set_c();
    } else {
        cpu.registers.clear_c();
    }
    (4, ilen)
}
