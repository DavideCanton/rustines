#[cfg(test)]
pub mod tests {
    use arch::cpu::CPU;
    use arch::memory::Memory;
    
    pub fn setup_tests() -> CPU {
        let mem = Memory::new();
        let mut cpu = CPU::new(mem);

        cpu.registers.PC = 0x100;

        cpu
    }
}
