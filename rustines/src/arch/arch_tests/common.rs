#[cfg(test)]
pub mod tests {
    use crate::arch::cpu::Cpu;
    use crate::arch::memory::Memory;
    use crate::arch::rom_structs::{Bank, INesHeader, NesRom};

    pub fn setup_tests() -> Cpu {
        let mut header = INesHeader::from_bytes(&[0; 16]);
        header.prg_rom_size = 1;

        let rom = NesRom::new(
            header,
            vec![Bank {
                ram: false,
                enabled: true,
                writable: false,
                battery: false,
                id: "bank0".to_string(),
                data: vec![0; 0x8000],
            }],
            1,
        );
        let mem = Memory::new(rom);
        let mut cpu = Cpu::new(mem);

        cpu.registers.pc = 0x100;

        cpu
    }
}
