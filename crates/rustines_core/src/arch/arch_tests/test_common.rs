use bytemuck::Zeroable;

use crate::{
    Apu, Cpu, NoAudio, NoopRenderer, Ppu,
    arch::{
        bus::{Bus, Controller2},
        mappers::Mapper0,
        rom_structs::{CHR_ROM_BANK_SIZE, HEADER, INesHeader, PRG_ROM_BANK_SIZE},
    },
};

pub fn setup_tests() -> (Cpu, Bus) {
    let mut header = INesHeader::zeroed();

    header.header = *HEADER;
    header.prg_rom_banks = 1;
    header.chr_rom_banks = 1;

    let mapper = Mapper0::new(&header, vec![0; PRG_ROM_BANK_SIZE + CHR_ROM_BANK_SIZE]).unwrap();
    let mapper = Box::new(mapper);

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
