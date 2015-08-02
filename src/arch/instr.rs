use arch::cpu::CPU;
use arch::bit_utils::to_u16;

pub const INSTR_TABLE: [Instr; 1] = [jmp_direct];

pub type Instr = fn(&mut CPU) -> u8;

fn jmp_direct(cpu: &mut CPU) -> u8
{
    let pc = cpu.registers.PC + 1; // fetch operand
    let low_pc = cpu.memory.fetch(pc);
    let high_pc = cpu.memory.fetch(pc + 1);

    cpu.registers.PC = to_u16(low_pc, high_pc);

    3
}
