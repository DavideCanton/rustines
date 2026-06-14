use crate::arch::{cpu::Cpu, memory::FetchStore};

pub fn immediate(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_immediate();
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(res), compute_c(res));
    cpu.registers.a_reg = res_a;
    (2, ilen)
}

pub fn zeropage(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_zeropage();
    let addr = cpu.memory.fetch(addr as u16);
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(res), compute_c(res));
    cpu.registers.a_reg = res_a;
    (3, ilen)
}

pub fn zeropage_x(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_zeropage_indexed(cpu.registers.x_reg);
    let addr = cpu.memory.fetch(addr as u16);
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(res), compute_c(res));
    cpu.registers.a_reg = res_a;
    (4, ilen)
}

pub fn absolute(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_absolute();
    let addr = cpu.memory.fetch(addr);
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(res), compute_c(res));
    cpu.registers.a_reg = res_a;
    (4, ilen)
}

pub fn absolute_x(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen, boundary) = cpu.decode_absolute_indexed(cpu.registers.x_reg);
    let addr = cpu.memory.fetch(addr);
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(res), compute_c(res));
    cpu.registers.a_reg = res_a;
    (4 + boundary, ilen)
}

pub fn absolute_y(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen, boundary) = cpu.decode_absolute_indexed(cpu.registers.y_reg);
    let addr = cpu.memory.fetch(addr);
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(res), compute_c(res));
    cpu.registers.a_reg = res_a;
    (4 + boundary, ilen)
}

pub fn indirect_x(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_indexed_indirect();
    let addr = cpu.memory.fetch(addr);
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(res), compute_c(res));
    cpu.registers.a_reg = res_a;
    (6, ilen)
}

pub fn indirect_y(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen, boundary) = cpu.decode_indirect_indexed();
    let addr = cpu.memory.fetch(addr);
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(res), compute_c(res));
    cpu.registers.a_reg = res_a;
    (5 + boundary, ilen)
}

fn compute_v(res: i16) -> bool {
    !(-128..=127).contains(&res)
}

fn compute_c(res: i16) -> bool {
    res & 0x80 == 0
}
