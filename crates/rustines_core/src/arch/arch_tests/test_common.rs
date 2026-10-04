use bytemuck::Zeroable;

use crate::arch::apu::Apu;
use crate::arch::bus::Bus;
use crate::arch::cpu::Cpu;
use crate::arch::mappers::mapper_0::Mapper0;
use crate::arch::ppu::Ppu;
use crate::arch::rom_structs::{CHR_ROM_BANK_SIZE, HEADER, INesHeader, PRG_ROM_BANK_SIZE};
use crate::renderer::NoopRenderer;

pub fn setup_tests() -> (Cpu, Bus) {
    let mut header = INesHeader::zeroed();

    header.header = *HEADER;
    header.prg_rom_banks = 1;
    header.chr_rom_banks = 1;

    let mapper = Mapper0::new(&header, vec![0; PRG_ROM_BANK_SIZE + CHR_ROM_BANK_SIZE]).unwrap();
    let mapper = Box::new(mapper);

    let bus = Bus::new(mapper, Ppu::new(Box::new(NoopRenderer)), Apu::default());
    let mut cpu = Cpu::new();

    cpu.registers.pc = 0x100;

    (cpu, bus)
}
