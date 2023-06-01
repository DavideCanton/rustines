use crate::arch::cpu::Cpu;

pub fn zeropage(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_zeropage();
    cpu.memory.store(addr as u16, cpu.registers.x_reg);
    (3, ilen)
}

pub fn zeropage_y(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_zeropage_indexed(cpu.registers.y_reg);
    cpu.memory.store(addr as u16, cpu.registers.x_reg);
    (4, ilen)
}

pub fn absolute(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_absolute();
    cpu.memory.store(addr, cpu.registers.x_reg);
    (4, ilen)
}
