use crate::arch::{
    bus::{Bus, FetchStore},
    cpu::Cpu,
};

pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let val = cpu.decode_immediate(bus);
    let y_reg = cpu.registers.y_reg;
    let res = (y_reg as u16).wrapping_sub(val as u16);

    cpu.registers.set_c_from_bool(y_reg >= val);
    cpu.registers.set_n_from_bool((res & 0x80) != 0);
    cpu.registers.set_z_from_bool(y_reg == val);

    2
}

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);
    let val = bus.fetch(addr as u16);
    let y_reg = cpu.registers.y_reg;
    let res = (y_reg as u16).wrapping_sub(val as u16);

    cpu.registers.set_c_from_bool(y_reg >= val);
    cpu.registers.set_n_from_bool((res & 0x80) != 0);
    cpu.registers.set_z_from_bool(y_reg == val);

    3
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    let val = bus.fetch(addr);
    let y_reg = cpu.registers.y_reg;
    let res = (y_reg as u16).wrapping_sub(val as u16);

    cpu.registers.set_c_from_bool(y_reg >= val);
    cpu.registers.set_n_from_bool((res & 0x80) != 0);
    cpu.registers.set_z_from_bool(y_reg == val);

    4
}
