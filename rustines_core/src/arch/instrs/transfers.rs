use crate::arch::{bus::Bus, cpu::Cpu};

pub fn tax(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    cpu.registers.x_reg = cpu.registers.a_reg;
    let x = cpu.registers.x_reg;
    cpu.registers.compute_nz_flags(x);
    2
}

pub fn tay(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    cpu.registers.y_reg = cpu.registers.a_reg;
    let y = cpu.registers.y_reg;
    cpu.registers.compute_nz_flags(y);
    2
}

pub fn txa(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    cpu.registers.a_reg = cpu.registers.x_reg;
    let a = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(a);
    2
}

pub fn tya(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    cpu.registers.a_reg = cpu.registers.y_reg;
    let a = cpu.registers.a_reg;
    cpu.registers.compute_nz_flags(a);
    2
}

pub fn tsx(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    cpu.registers.x_reg = cpu.registers.sp;
    let x = cpu.registers.x_reg;
    cpu.registers.compute_nz_flags(x);
    2
}

pub fn txs(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    cpu.registers.sp = cpu.registers.x_reg;
    2
}
