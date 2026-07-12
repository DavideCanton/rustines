use crate::arch::{
    bus::{Bus, FetchStore},
    cpu::Cpu,
};

pub fn accumulator(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    let val = cpu.registers.a_reg;
    let res = do_ror(cpu, val);
    cpu.registers.a_reg = res;
    2
}

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);
    let val = bus.fetch(addr as u16);
    let res = do_ror(cpu, val);
    bus.store(addr as u16, res);
    5
}

pub fn zeropage_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage_indexed(bus, cpu.registers.x_reg);
    let val = bus.fetch(addr as u16);
    let res = do_ror(cpu, val);
    bus.store(addr as u16, res);
    6
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    let val = bus.fetch(addr);
    let res = do_ror(cpu, val);
    bus.store(addr, res);
    6
}

pub fn absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (addr, _) = cpu.decode_absolute_indexed(bus, cpu.registers.x_reg);
    let val = bus.fetch(addr);
    let res = do_ror(cpu, val);
    bus.store(addr, res);
    7
}

fn do_ror(cpu: &mut Cpu, val: u8) -> u8 {
    let old_c = cpu.registers.get_c() as u8;
    cpu.registers.set_c_from_bool(val & 0x01 != 0);
    let res = (val >> 1) & 0xFE | (old_c << 7);
    cpu.registers.compute_nz_flags(res);
    res
}
