use crate::arch::bus::{Bus, FetchStore};
use crate::arch::cpu::Cpu;
use crate::utils::bit_utils::to_u16;

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    cpu.registers.pc = addr;
    3
}

pub fn indirect_absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    let low = bus.fetch(addr);

    // reproduce bug on jmp indirect
    // in case the address ends with FF, the +1 should not wrap the other byte
    // so jmp [0x02FF] should read from 0x02FF and 0x0200 (not 0x0300)
    let high = if (addr & 0x00FF) == 0x00FF {
        bus.fetch(addr & 0xFF00)
    } else {
        bus.fetch(addr + 1)
    };

    cpu.registers.pc = to_u16(low, high);
    5
}
