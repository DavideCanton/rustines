use crate::{
    arch::{bus::Bus, cpu::Cpu, instrs::ror::do_ror},
    utils::bit_utils::{BitIndex, BitManip},
};

pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let old_c = cpu.registers.get_c();
    let val = cpu.decode_immediate(bus);
    let res = cpu.registers.a_reg & val;
    let res = do_ror(cpu, res);
    cpu.registers.a_reg = res;
    cpu.registers.set_c(res.extract_flag(BitIndex::_6));
    cpu.registers
        .set_v(res.extract_flag(BitIndex::_6) ^ res.extract_flag(BitIndex::_5));
    cpu.registers.set_n(old_c);
    2
}
