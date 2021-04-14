#[cfg(test)]
pub mod tests {
    use crate::arch::cpu::Cpu;
    use crate::arch::memory::Memory;
    
    pub fn setup_tests() -> Cpu {
        let mem = Memory::new();
        let mut cpu = Cpu::new(mem);

        cpu.registers.pc = 0x100;

        cpu
    }
}
