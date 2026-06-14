use crate::arch::{cpu::Cpu, memory::FetchStore};

pub fn immediate(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_immediate();
    let res = cpu.registers.a_reg ^ addr;
    cpu.registers.a_reg = res;
    cpu.registers.compute_nz_flags(res);
    (2, ilen)
}

pub fn zeropage(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_zeropage();
    let addr = cpu.memory.fetch(addr as u16);
    let res = cpu.registers.a_reg ^ addr;
    cpu.registers.a_reg = res;
    cpu.registers.compute_nz_flags(res);
    (3, ilen)
}

pub fn zeropage_x(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_zeropage_indexed(cpu.registers.x_reg);
    let addr = cpu.memory.fetch(addr as u16);
    let res = cpu.registers.a_reg ^ addr;
    cpu.registers.a_reg = res;
    cpu.registers.compute_nz_flags(res);
    (4, ilen)
}

pub fn absolute(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_absolute();
    let addr = cpu.memory.fetch(addr);
    let res = cpu.registers.a_reg ^ addr;
    cpu.registers.a_reg = res;
    cpu.registers.compute_nz_flags(res);
    (4, ilen)
}

pub fn absolute_x(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen, boundary) = cpu.decode_absolute_indexed(cpu.registers.x_reg);
    let addr = cpu.memory.fetch(addr);
    let res = cpu.registers.a_reg ^ addr;
    cpu.registers.a_reg = res;
    cpu.registers.compute_nz_flags(res);
    (4 + boundary, ilen)
}

pub fn absolute_y(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen, boundary) = cpu.decode_absolute_indexed(cpu.registers.y_reg);
    let addr = cpu.memory.fetch(addr);
    let res = cpu.registers.a_reg ^ addr;
    cpu.registers.a_reg = res;
    cpu.registers.compute_nz_flags(res);
    (4 + boundary, ilen)
}

pub fn indirect_x(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_indexed_indirect();
    let addr = cpu.memory.fetch(addr);
    let res = cpu.registers.a_reg ^ addr;
    cpu.registers.a_reg = res;
    cpu.registers.compute_nz_flags(res);
    (6, ilen)
}

pub fn indirect_y(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen, boundary) = cpu.decode_indirect_indexed();
    let addr = cpu.memory.fetch(addr);
    let res = cpu.registers.a_reg ^ addr;
    cpu.registers.a_reg = res;
    cpu.registers.compute_nz_flags(res);
    (5 + boundary, ilen)
}
