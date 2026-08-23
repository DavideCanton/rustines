use crate::arch::{bus::Bus, cpu::Cpu};

pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let val = cpu.decode_immediate(bus);
    do_adc(cpu, val);
    2
}

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);
    let val = bus.read(addr as u16);
    do_adc(cpu, val);
    3
}

pub fn zeropage_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage_indexed(bus, cpu.registers.x_reg);
    let val = bus.read(addr as u16);
    do_adc(cpu, val);
    4
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    let val = bus.read(addr);
    do_adc(cpu, val);
    4
}

pub fn absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let result = cpu.decode_absolute_indexed(bus, cpu.registers.x_reg, false);
    let addr = result.address();
    let val = bus.read(addr);
    do_adc(cpu, val);
    4 + result.page_boundary_crossed_as_u8()
}

pub fn absolute_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let result = cpu.decode_absolute_indexed(bus, cpu.registers.y_reg, false);
    let addr = result.address();
    let val = bus.read(addr);
    do_adc(cpu, val);
    4 + result.page_boundary_crossed_as_u8()
}

pub fn indirect_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_indexed_indirect(bus);
    let val = bus.read(addr);
    do_adc(cpu, val);
    6
}

pub fn indirect_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let result = cpu.decode_indirect_indexed(bus, false);
    let addr = result.address();
    let val = bus.read(addr);
    do_adc(cpu, val);
    5 + result.page_boundary_crossed_as_u8()
}

pub(crate) fn do_adc(cpu: &mut Cpu, val: u8) {
    let res = (cpu.registers.a_reg as u16) + (val as u16) + (cpu.registers.get_c() as u16);
    let res_a = (res & 0b1111_1111) as u8;
    let old_a = cpu.registers.a_reg;
    cpu.registers.update_nz_flags(res_a);
    cpu.registers.set_v_from_bool(compute_v(old_a, res_a, val));
    cpu.registers.set_c_from_bool(compute_c(res));
    cpu.registers.a_reg = res_a;
}

fn compute_v(a: u8, res_a: u8, m: u8) -> bool {
    ((a ^ res_a) & (m ^ res_a) & 0b1000_0000) != 0
}

fn compute_c(res: u16) -> bool {
    res & 0b1_0000_0000 != 0
}
