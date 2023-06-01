use crate::arch::cpu::Cpu;

pub fn immediate(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_immediate();
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(!(-128..=127).contains(&res), res & 0x80 == 0);
    cpu.registers.a_reg = res_a;
    (2, ilen)
}

pub fn zeropage(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_zeropage();
    let addr = cpu.memory.fetch(addr as u16);
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(!(-128..=127).contains(&res), res & 0x80 == 0);
    cpu.registers.a_reg = res_a;
    (3, ilen)
}

pub fn zeropage_x(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_zeropage_indexed(cpu.registers.x_reg);
    let addr = cpu.memory.fetch(addr as u16);
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(!(-128..=127).contains(&res), res & 0x80 == 0);
    cpu.registers.a_reg = res_a;
    (4, ilen)
}

pub fn absolute(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_absolute();
    let addr = cpu.memory.fetch(addr);
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(!(-128..=127).contains(&res), res & 0x80 == 0);
    cpu.registers.a_reg = res_a;
    (4, ilen)
}

pub fn absolute_x(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_absolute_indexed(cpu.registers.x_reg);
    let addr = cpu.memory.fetch(addr);
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(!(-128..=127).contains(&res), res & 0x80 == 0);
    cpu.registers.a_reg = res_a;
    (4, ilen)
    // TODO +1 if page boundary
}

pub fn absolute_y(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_absolute_indexed(cpu.registers.y_reg);
    let addr = cpu.memory.fetch(addr);
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(!(-128..=127).contains(&res), res & 0x80 == 0);
    cpu.registers.a_reg = res_a;
    (4, ilen)
    // TODO +1 if page boundary
}

pub fn indirect_x(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_indexed_indirect();
    let addr = cpu.memory.fetch(addr);
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(!(-128..=127).contains(&res), res & 0x80 == 0);
    cpu.registers.a_reg = res_a;
    (6, ilen)
}

pub fn indirect_y(cpu: &mut Cpu) -> (u8, u8) {
    let (addr, ilen) = cpu.decode_indirect_indexed();
    let addr = cpu.memory.fetch(addr);
    let res = (cpu.registers.a_reg as u16)
        .wrapping_sub(addr as u16)
        .wrapping_sub(!cpu.registers.get_c() as u16);
    let res_a = (res & 0xFF) as u8;
    let res = res as i16;
    cpu.registers.compute_nz_flags(res_a);
    cpu.registers
        .compute_vc_flags(!(-128..=127).contains(&res), res & 0x80 == 0);
    cpu.registers.a_reg = res_a;
    (5, ilen)
    // TODO +1 if page boundary
}
