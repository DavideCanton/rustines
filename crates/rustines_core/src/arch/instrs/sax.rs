use crate::arch::{bus::Bus, cpu::Cpu};

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);
    do_sax(bus, addr as u16, cpu);
    3
}

pub fn zeropage_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage_indexed(bus, cpu.registers.y_reg);
    do_sax(bus, addr as u16, cpu);
    4
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    do_sax(bus, addr, cpu);
    4
}

pub fn indirect_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_indexed_indirect(bus);
    do_sax(bus, addr, cpu);
    6
}

fn do_sax(bus: &mut Bus, addr: u16, cpu: &mut Cpu) {
    bus.store(addr, cpu.registers.a_reg & cpu.registers.x_reg);
}
