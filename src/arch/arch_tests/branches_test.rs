#[cfg(test)]
mod tests {
    use arch::arch_tests::common::tests::setup_tests;
    use arch::instrs::branches;

    #[test]
    fn test_bcc_ok() {
        let mut cpu = setup_tests();

        let old_pc = cpu.registers.pc;

        {
            cpu.memory.store(cpu.registers.pc, 0x90);
            cpu.memory.store(cpu.registers.pc + 1, 0xFF);

            cpu.registers.clear_c();
        }

        let (cycles, ilen) = branches::bcc(&mut cpu);

        assert_eq!(3, cycles);
        assert_eq!(0, ilen);

        let val = cpu.registers.pc;
        assert_eq!(val, old_pc - 1);
    }

    #[test]
    fn test_bcc_no() {
        let mut cpu = setup_tests();

        let old_pc = cpu.registers.pc;

        {
            cpu.memory.store(cpu.registers.pc, 0x90);
            cpu.memory.store(cpu.registers.pc + 1, 0xFF);

            cpu.registers.set_c();
        }

        let (cycles, ilen) = branches::bcc(&mut cpu);

        assert_eq!(2, cycles);
        assert_eq!(2, ilen);

        let val = cpu.registers.pc;
        assert_eq!(val, old_pc);
    }

    #[test]
    fn test_bcs_ok() {
        let mut cpu = setup_tests();

        let old_pc = cpu.registers.pc;

        {
            cpu.memory.store(cpu.registers.pc, 0x90);
            cpu.memory.store(cpu.registers.pc + 1, 0xFF);

            cpu.registers.set_c();
        }

        let (cycles, ilen) = branches::bcs(&mut cpu);

        assert_eq!(3, cycles);
        assert_eq!(0, ilen);

        let val = cpu.registers.pc;
        assert_eq!(val, old_pc - 1);
    }

    #[test]
    fn test_bcs_no() {
        let mut cpu = setup_tests();

        let old_pc = cpu.registers.pc;

        {
            cpu.memory.store(cpu.registers.pc, 0x90);
            cpu.memory.store(cpu.registers.pc + 1, 0xFF);

            cpu.registers.clear_c();
        }

        let (cycles, ilen) = branches::bcs(&mut cpu);

        assert_eq!(2, cycles);
        assert_eq!(2, ilen);

        let val = cpu.registers.pc;
        assert_eq!(val, old_pc);
    }

}
