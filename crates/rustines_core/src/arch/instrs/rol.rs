use crate::arch::{
    bus::{Bus, FetchStore},
    cpu::Cpu,
};

pub fn accumulator(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    let mut val = cpu.registers.a_reg;
    let old_c = cpu.registers.get_c() as u8;
    cpu.registers.set_c_from_bool(val & 0x80 != 0);
    val = (val << 1) & 0xFE | old_c;
    cpu.registers.a_reg = val;
    cpu.registers.compute_nz_flags(val);
    2
}

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);

    let mut val = bus.fetch(addr as u16);
    let old_c = cpu.registers.get_c() as u8;
    cpu.registers.set_c_from_bool(val & 0x80 != 0);
    val = (val << 1) & 0xFE | old_c;
    cpu.registers.compute_nz_flags(val);
    bus.store(addr as u16, val);
    5
}

pub fn zeropage_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage_indexed(bus, cpu.registers.x_reg);

    let mut val = bus.fetch(addr as u16);
    let old_c = cpu.registers.get_c() as u8;
    cpu.registers.set_c_from_bool(val & 0x80 != 0);
    val = (val << 1) & 0xFE | old_c;
    cpu.registers.compute_nz_flags(val);
    bus.store(addr as u16, val);
    6
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);

    let mut val = bus.fetch(addr);
    let old_c = cpu.registers.get_c() as u8;
    cpu.registers.set_c_from_bool(val & 0x80 != 0);
    val = (val << 1) & 0xFE | old_c;
    cpu.registers.compute_nz_flags(val);
    bus.store(addr, val);
    6
}

pub fn absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, _) = cpu.decode_absolute_indexed(bus, cpu.registers.x_reg);

    let mut val = bus.fetch(addr);
    let old_c = cpu.registers.get_c() as u8;
    cpu.registers.set_c_from_bool(val & 0x80 != 0);
    val = (val << 1) & 0xFE | old_c;
    cpu.registers.compute_nz_flags(val);
    bus.store(addr, val);
    7
}
