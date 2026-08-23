use crate::arch::bus::Bus;
use crate::arch::cpu::Cpu;
use crate::utils::bit_utils::*;

pub fn implied(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let pc = cpu.registers.pc;

    let _padding = bus.read(pc);

    cpu.push16(bus, pc.wrapping_add(1));

    let p = cpu.registers.get_p_force_b(true);
    cpu.push8(bus, p);

    cpu.poll_non_maskable_interrupts(bus);

    let address = if cpu.pending_nmi_execution {
        cpu.clear_nmi(bus);
        0xFFFA
    } else {
        0xFFFE
    };

    let l = bus.read(address);
    let h = bus.read(address.wrapping_add(1));

    cpu.registers.pc = to_u16(l, h);
    cpu.registers.set_i();

    7
}
