use crate::{
    arch::{bus::Bus, cpu::Cpu, instrs::and::do_and},
    utils::bit_utils::{BitIndex, extract_flag},
};

pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let val = cpu.decode_immediate(bus);
    let res = do_and(cpu, val);
    cpu.registers.set_c(extract_flag(res, BitIndex::_7));
    2
}
