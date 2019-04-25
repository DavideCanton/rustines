#[cfg(test)]
pub mod tests {
    use crate::arch::cpu::CPU;
    use crate::arch::memory::Memory;
    
    pub fn setup_tests() -> CPU {
        let mem = Memory::new();
        let mut cpu = CPU::new(mem);

        cpu.registers.pc = 0x100;

        cpu
    }
}
