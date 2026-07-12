use crate::arch::{
    bus::{Bus, FetchStore},
    cpu::Cpu,
};

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);
    let val = bus.fetch(addr as u16);
    let res = val & cpu.registers.a_reg;

    cpu.registers.set_n_from_bool(val & 0x80 != 0);
    cpu.registers.set_v_from_bool(val & 0x40 != 0);
    cpu.registers.set_z_from_bool(res == 0);

    3
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    let val = bus.fetch(addr);
    let res = val & cpu.registers.a_reg;

    cpu.registers.set_n_from_bool(val & 0x80 != 0);
    cpu.registers.set_v_from_bool(val & 0x40 != 0);
    cpu.registers.set_z_from_bool(res == 0);

    4
}
