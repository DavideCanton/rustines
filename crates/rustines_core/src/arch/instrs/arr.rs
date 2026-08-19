use crate::{
    arch::{bus::Bus, cpu::Cpu, instrs::ror::do_ror},
    utils::bit_utils::extract_flag,
};

pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let old_c = cpu.registers.get_c();
    let val = cpu.decode_immediate(bus);
    let res = cpu.registers.a_reg & val;
    let res = do_ror(cpu, res);
    cpu.registers.a_reg = res;
    cpu.registers.set_c_from_bool(extract_flag(res, 6));
    cpu.registers
        .set_v_from_bool(extract_flag(res, 6) ^ extract_flag(res, 5));
    cpu.registers.set_n_from_bool(old_c);
    2
}
