use arch::cpu::CPU;

pub fn tax(cpu: &mut CPU) -> (u8, u8)
{
    cpu.registers.X = cpu.registers.A;
    let x = cpu.registers.X;
    cpu.registers.compute_NZ_flags(x);
    (2, 1)
}

pub fn tay(cpu: &mut CPU) -> (u8, u8)
{
    cpu.registers.Y = cpu.registers.A;
    let y = cpu.registers.Y;
    cpu.registers.compute_NZ_flags(y);
    (2, 1)
}

pub fn txa(cpu: &mut CPU) -> (u8, u8)
{
    cpu.registers.A = cpu.registers.X;
    let a = cpu.registers.A;
    cpu.registers.compute_NZ_flags(a);
    (2, 1)
}

pub fn tya(cpu: &mut CPU) -> (u8, u8)
{
    cpu.registers.A = cpu.registers.Y;
    let a = cpu.registers.A;
    cpu.registers.compute_NZ_flags(a);
    (2, 1)
}

pub fn tsx(cpu: &mut CPU) -> (u8, u8)
{
    cpu.registers.X = cpu.registers.SP;
    let x = cpu.registers.X;
    cpu.registers.compute_NZ_flags(x);
    (2, 1)
}

pub fn txs(cpu: &mut CPU) -> (u8, u8)
{
    cpu.registers.SP = cpu.registers.X;
    (2, 1)
}
