use crate::arch::{
    bus::{Bus, FetchStore},
    cpu::Cpu,
};

pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let val = cpu.decode_immediate(bus);
    do_sbc(cpu, val);
    2
}

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);
    let val = bus.fetch(addr as u16);
    do_sbc(cpu, val);
    3
}

pub fn zeropage_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage_indexed(bus, cpu.registers.x_reg);
    let val = bus.fetch(addr as u16);
    do_sbc(cpu, val);
    4
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    let val = bus.fetch(addr);
    do_sbc(cpu, val);
    4
}

pub fn absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, boundary) = cpu.decode_absolute_indexed(bus, cpu.registers.x_reg);
    let val = bus.fetch(addr);
    do_sbc(cpu, val);
    4 + boundary
}

pub fn absolute_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, boundary) = cpu.decode_absolute_indexed(bus, cpu.registers.y_reg);
    let val = bus.fetch(addr);
    do_sbc(cpu, val);
    4 + boundary
}

pub fn indirect_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_indexed_indirect(bus);
    let val = bus.fetch(addr);
    do_sbc(cpu, val);
    6
}

pub fn indirect_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, boundary) = cpu.decode_indirect_indexed(bus);
    let val = bus.fetch(addr);
    do_sbc(cpu, val);
    5 + boundary
}

fn compute_v(a: u8, m: u8, res: u8) -> bool {
    ((a ^ res) & (m ^ !res) & 0x80) != 0
}

fn compute_c(res: u16) -> bool {
    res < 0x100
}

fn do_sbc(cpu: &mut Cpu, val: u8) {
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(val as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(cpu.registers.a_reg, val, res_a), compute_c(res));
    cpu.registers.a_reg = res_a;
}
