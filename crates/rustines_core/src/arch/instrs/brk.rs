use crate::arch::bus::Bus;
use crate::arch::cpu::Cpu;
use crate::utils::bit_utils::*;

pub fn implied(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let pc = cpu.registers.pc;

    let _padding = bus.read(pc);

    cpu.push16(bus, pc.wrapping_add(1));

    let p = cpu.registers.get_p_force_b(true);
    cpu.push8(bus, p);

    let l = bus.read(0xFFFE);
    let h = bus.read(0xFFFF);

    cpu.registers.pc = to_u16(l, h);
    cpu.registers.set_i();

    7
}
