use crate::arch::{
    bus::{Bus, FetchStore},
    cpu::Cpu,
};

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);
    let val = bus.fetch(addr as u16);
    do_bit(cpu, val);
    3
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    let val = bus.fetch(addr);
    do_bit(cpu, val);
    4
}

fn do_bit(cpu: &mut Cpu, val: u8) {
    let res = val & cpu.registers.a_reg;
    cpu.registers.set_n_from_bool(val & 0x80 != 0);
    cpu.registers.set_v_from_bool(val & 0x40 != 0);
    cpu.registers.set_z_from_bool(res == 0);
}
