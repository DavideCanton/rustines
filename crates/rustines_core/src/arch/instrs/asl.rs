use crate::arch::{bus::Bus, cpu::Cpu, instrs::utils::store_with_dummy_write};

pub fn accumulator(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    let val = cpu.registers.a_reg;
    let res = do_asl(cpu, val);
    cpu.registers.a_reg = res;
    2
}

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);
    let val = bus.fetch(addr as u16);
    let res = do_asl(cpu, val);
    store_with_dummy_write(bus, addr as u16, val, res);
    5
}

pub fn zeropage_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage_indexed(bus, cpu.registers.x_reg);
    let val = bus.fetch(addr as u16);
    let res = do_asl(cpu, val);
    store_with_dummy_write(bus, addr as u16, val, res);
    6
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    let val = bus.fetch(addr);
    let res = do_asl(cpu, val);
    store_with_dummy_write(bus, addr, val, res);
    6
}

pub fn absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, _) = cpu.decode_absolute_indexed(bus, cpu.registers.x_reg, false);
    let val = bus.fetch(addr);
    let res = do_asl(cpu, val);
    store_with_dummy_write(bus, addr, val, res);
    7
}

fn do_asl(cpu: &mut Cpu, mut val: u8) -> u8 {
    cpu.registers.set_c_from_bool(val & 0x80 != 0);
    val <<= 1;
    cpu.registers.compute_nz_flags(val);
    val
}
