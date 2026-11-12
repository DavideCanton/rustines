use bytemuck::Zeroable;

use crate::{
    Apu, Cpu, NoAudio, NoopRenderer, Ppu,
    arch::{
        bus::{Bus, Controller2},
        mappers::mapper_factory::instantiate_mapper,
        rom_structs::{CHR_ROM_BANK_SIZE, HEADER, INesHeader, PRG_ROM_BANK_SIZE},
    },
};

pub fn setup_tests() -> (Cpu, Bus) {
    let mut header = INesHeader::zeroed();

    header.header = *HEADER;
    header.prg_rom_banks = 1;
    header.chr_rom_banks = 1;

    let mapper =
        instantiate_mapper(&header, vec![0; PRG_ROM_BANK_SIZE + CHR_ROM_BANK_SIZE]).unwrap();

    let bus = Bus::new(
        mapper,
        Ppu::new(Box::new(NoopRenderer)),
        Apu::new(Box::new(NoAudio)),
        Controller2::nes_controller(),
    );
    let mut cpu = Cpu::new();

    cpu.registers.pc = 0x100;

    (cpu, bus)
}
