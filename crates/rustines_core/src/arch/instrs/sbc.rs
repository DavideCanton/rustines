use crate::arch::{bus::{Bus, FetchStore}, cpu::Cpu};

pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_immediate(bus);
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(res), compute_c(res));
    cpu.registers.a_reg = res_a;
    2
}

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);
    let addr = bus.fetch(addr as u16);
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(res), compute_c(res));
    cpu.registers.a_reg = res_a;
    3
}

pub fn zeropage_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage_indexed(bus,cpu.registers.x_reg);
    let addr = bus.fetch(addr as u16);
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(res), compute_c(res));
    cpu.registers.a_reg = res_a;
    4
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    let addr = bus.fetch(addr);
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(res), compute_c(res));
    cpu.registers.a_reg = res_a;
    4
}

pub fn absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, boundary) = cpu.decode_absolute_indexed(bus,cpu.registers.x_reg);
    let addr = bus.fetch(addr);
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(res), compute_c(res));
    cpu.registers.a_reg = res_a;
    4 + boundary
}

pub fn absolute_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, boundary) = cpu.decode_absolute_indexed(bus,cpu.registers.y_reg);
    let addr = bus.fetch(addr);
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(res), compute_c(res));
    cpu.registers.a_reg = res_a;
    4 + boundary
}

pub fn indirect_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_indexed_indirect(bus);
    let addr = bus.fetch(addr);
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(res), compute_c(res));
    cpu.registers.a_reg = res_a;
    6
}

pub fn indirect_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, boundary) = cpu.decode_indirect_indexed(bus);
    let addr = bus.fetch(addr);
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(compute_v(res), compute_c(res));
    cpu.registers.a_reg = res_a;
    5 + boundary
}

fn compute_v(res: i16) -> bool {
    !(-128..=127).contains(&res)
}

fn compute_c(res: i16) -> bool {
    res & 0x80 == 0
}
