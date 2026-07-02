#[cfg(test)]
mod tests {
    use crate::arch::arch_tests::test_common::tests::setup_tests;
    use crate::arch::bus::FetchStore;
    use crate::arch::instrs::branches;

    #[test]
    fn test_bcc_ok() {
        let (mut cpu, mut bus) = setup_tests();

        {
            bus.store(cpu.registers.pc, 0x90);
            bus.store(cpu.registers.pc + 1, 0xFF);

            cpu.registers.clear_c();
        }

        cpu.registers.pc += 2;
        let old_pc = cpu.registers.pc;

        let cycles = branches::bcc(&mut cpu, &mut bus);

        assert_eq!(3, cycles);

        let val = cpu.registers.pc;
        assert_eq!(val, old_pc - 1);
    }

    #[test]
    fn test_bcc_no() {
        let (mut cpu, mut bus) = setup_tests();

        {
            bus.store(cpu.registers.pc, 0x90);
            bus.store(cpu.registers.pc + 1, 0xFF);

            cpu.registers.set_c();
        }

        cpu.registers.pc += 2;
        let old_pc = cpu.registers.pc;

        let cycles = branches::bcc(&mut cpu, &mut bus);

        assert_eq!(2, cycles);

        let val = cpu.registers.pc;
        assert_eq!(val, old_pc);
    }

    #[test]
    fn test_bcs_ok() {
        let (mut cpu, mut bus) = setup_tests();

        {
            bus.store(cpu.registers.pc, 0x90);
            bus.store(cpu.registers.pc + 1, 0xFF);

            cpu.registers.set_c();
        }

        cpu.registers.pc += 2;
        let old_pc = cpu.registers.pc;

        let cycles = branches::bcs(&mut cpu, &mut bus);

        assert_eq!(3, cycles);

        let val = cpu.registers.pc;
        assert_eq!(val, old_pc - 1);
    }

    #[test]
    fn test_bcs_no() {
        let (mut cpu, mut bus) = setup_tests();

        {
            bus.store(cpu.registers.pc, 0x90);
            bus.store(cpu.registers.pc + 1, 0xFF);

            cpu.registers.clear_c();
        }

        cpu.registers.pc += 2;
        let old_pc = cpu.registers.pc;

        let cycles = branches::bcs(&mut cpu, &mut bus);

        assert_eq!(2, cycles);

        let val = cpu.registers.pc;
        assert_eq!(val, old_pc);
    }
}
