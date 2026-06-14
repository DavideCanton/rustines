use crate::arch::{cpu::Cpu, memory::FetchStore};

pub fn immediate(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_immediate();
    let res = (cpu.registers.a_reg as u16) + (addr as u16) + (cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let old_a = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(old_a, res_a), compute_c(res));
    cpu.registers.a_reg = res_a;
    (2, ilen)
}

pub fn zeropage(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_zeropage();
    let addr = cpu.memory.fetch(addr as u16);
    let res = (cpu.registers.a_reg as u16) + (addr as u16) + (cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let old_a = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(old_a, res_a), compute_c(res));
    cpu.registers.a_reg = res_a;
    (3, ilen)
}

pub fn zeropage_x(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_zeropage_indexed(cpu.registers.x_reg);
    let addr = cpu.memory.fetch(addr as u16);
    let res = (cpu.registers.a_reg as u16) + (addr as u16) + (cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let old_a = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(old_a, res_a), compute_c(res));
    cpu.registers.a_reg = res_a;
    (4, ilen)
}

pub fn absolute(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_absolute();
    let addr = cpu.memory.fetch(addr);
    let res = (cpu.registers.a_reg as u16) + (addr as u16) + (cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let old_a = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(old_a, res_a), compute_c(res));
    cpu.registers.a_reg = res_a;
    (4, ilen)
}

pub fn absolute_x(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen, boundary) = cpu.decode_absolute_indexed(cpu.registers.x_reg);
    let addr = cpu.memory.fetch(addr);
    let res = (cpu.registers.a_reg as u16) + (addr as u16) + (cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let old_a = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(old_a, res_a), compute_c(res));
    cpu.registers.a_reg = res_a;
    (4 + boundary, ilen)
}

pub fn absolute_y(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen, boundary) = cpu.decode_absolute_indexed(cpu.registers.y_reg);
    let addr = cpu.memory.fetch(addr);
    let res = (cpu.registers.a_reg as u16) + (addr as u16) + (cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let old_a = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(old_a, res_a), compute_c(res));
    cpu.registers.a_reg = res_a;
    (4 + boundary, ilen)
}

pub fn indirect_x(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_indexed_indirect();
    let addr = cpu.memory.fetch(addr);
    let res = (cpu.registers.a_reg as u16) + (addr as u16) + (cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let old_a = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(old_a, res_a), compute_c(res));
    cpu.registers.a_reg = res_a;
    (6, ilen)
}

pub fn indirect_y(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen, boundary) = cpu.decode_indirect_indexed();
    let addr = cpu.memory.fetch(addr);
    let res = (cpu.registers.a_reg as u16) + (addr as u16) + (cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let old_a = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(old_a, res_a), compute_c(res));
    cpu.registers.a_reg = res_a;
    (5 + boundary, ilen)
}

fn compute_v(old: u8, res: u8) -> bool {
    old >> 7 != res >> 7
}

fn compute_c(res: u16) -> bool {
    res & 0x100 != 0
}
