#[cfg(test)]
pub mod tests {
    use crate::arch::bus::Bus;
    use crate::arch::cpu::Cpu;
    use crate::arch::mappers::mapper_0::Mapper0;
    use crate::arch::ppu::Ppu;
    use crate::arch::rom_structs::{
        CHR_ROM_BANK_SIZE, HEADER, INesHeader, NesRom, PRG_ROM_BANK_SIZE,
    };
    use crate::renderer::NoopRenderer;

    pub fn setup_tests() -> (Cpu, Bus) {
        let mut header = INesHeader::from_bytes(&[0; 16]);
        header.header = *HEADER;
        header.prg_rom_size = 1;
        header.chr_rom_size = 1;

        let mapper = Box::new(
            Mapper0::new(&header, vec![0; PRG_ROM_BANK_SIZE + CHR_ROM_BANK_SIZE]).unwrap(),
        );

        let rom = NesRom::new(header, mapper);
        let bus = Bus::new(rom.mapper, Ppu::new(Box::new(NoopRenderer)));
        let mut cpu = Cpu::new();

        cpu.registers.pc = 0x100;

        (cpu, bus)
    }
}
