#[cfg(test)]
pub mod tests {
    use crate::arch::cpu::Cpu;
    use crate::arch::mappers::mapper_0::Mapper0;
    use crate::arch::memory::Memory;
    use crate::arch::rom_structs::{
        CHR_ROM_BANK_SIZE, HEADER, INesHeader, NesRom, PRG_ROM_BANK_SIZE,
    };

    pub fn setup_tests() -> Cpu {
        let mut header = INesHeader::from_bytes(&[0; 16]);
        header.header = *HEADER;
        header.prg_rom_size = 1;
        header.chr_rom_size = 1;

        let mapper = Box::new(Mapper0::new(
            &header,
            vec![0; PRG_ROM_BANK_SIZE + CHR_ROM_BANK_SIZE],
        ));

        let rom = NesRom::new(header, mapper);
        let mem = Memory::new(rom.mapper);
        let mut cpu = Cpu::new(mem);

        cpu.registers.pc = 0x100;

        cpu
    }
}
