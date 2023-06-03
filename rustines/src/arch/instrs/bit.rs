use crate::arch::cpu::Cpu;

pub fn zeropage(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_zeropage();
    let addr = cpu.memory.fetch(addr as u16);
    let res = addr & cpu.registers.a_reg;

    cpu.registers.set_n_from_bool(res & 0x80 != 0);
    cpu.registers.set_v_from_bool(res & 0x40 != 0);
    cpu.registers.set_z_from_bool(res == 0);

    (3, ilen)
}

pub fn absolute(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_absolute();
    let addr = cpu.memory.fetch(addr);
    let res = addr & cpu.registers.a_reg;

    cpu.registers.set_n_from_bool(res & 0x80 != 0);
    cpu.registers.set_v_from_bool(res & 0x40 != 0);
    cpu.registers.set_z_from_bool(res == 0);

    (4, ilen)
}
