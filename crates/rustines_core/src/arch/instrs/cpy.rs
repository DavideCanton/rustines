use crate::arch::{
    bus::{Bus, FetchStore},
    cpu::Cpu,
};

pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let val = cpu.decode_immediate(bus);
    do_cpy(cpu, val);
    2
}

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);
    let val = bus.fetch(addr as u16);
    do_cpy(cpu, val);
    3
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    let val = bus.fetch(addr);
    do_cpy(cpu, val);
    4
}

fn do_cpy(cpu: &mut Cpu, val: u8) {
    let y_reg = cpu.registers.y_reg;
    let res = (y_reg as u16).wrapping_sub(val as u16);

    cpu.registers.set_c_from_bool(y_reg >= val);
    cpu.registers.set_n_from_bool((res & 0x80) != 0);
    cpu.registers.set_z_from_bool(y_reg == val);
}
