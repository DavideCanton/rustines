use arch::cpu::CPU;
use arch::instrs::instr_table::decode_zeropage;

pub fn bcc(cpu: &mut CPU) -> (u8, u8)
{
    if !cpu.registers.getC()
    {
        let addr = decode_zeropage(cpu).0 as u16;
        let new_pc = cpu.registers.PC.wrapping_add(addr);
        cpu.registers.PC = new_pc;

        (3, 0)
        // TODO add 1 if destination address on different page
    }
    else
    {
        (2, 2)
    }
}

pub fn bcs(cpu: &mut CPU) -> (u8, u8)
{
    if cpu.registers.getC()
    {
        let addr = decode_zeropage(cpu).0 as u16;
        let new_pc = cpu.registers.PC.wrapping_add(addr);
        cpu.registers.PC = new_pc;

        (3, 0)
        // TODO add 1 if destination address on different page
    }
    else
    {
        (2, 2)
    }
}

pub fn beq(cpu: &mut CPU) -> (u8, u8)
{
    if !cpu.registers.getZ()
    {
        let addr = decode_zeropage(cpu).0 as u16;
        let new_pc = cpu.registers.PC.wrapping_add(addr);
        cpu.registers.PC = new_pc;

        (3, 0)
        // TODO add 1 if destination address on different page
    }
    else
    {
        (2, 2)
    }
}

pub fn bne(cpu: &mut CPU) -> (u8, u8)
{
    if cpu.registers.getZ()
    {
        let addr = decode_zeropage(cpu).0 as u16;
        let new_pc = cpu.registers.PC.wrapping_add(addr);
        cpu.registers.PC = new_pc;

        (3, 0)
        // TODO add 1 if destination address on different page
    }
    else
    {
        (2, 2)
    }
}

pub fn bmi(cpu: &mut CPU) -> (u8, u8)
{
    if !cpu.registers.getN()
    {
        let addr = decode_zeropage(cpu).0 as u16;
        let new_pc = cpu.registers.PC.wrapping_add(addr);
        cpu.registers.PC = new_pc;

        (3, 0)
        // TODO add 1 if destination address on different page
    }
    else
    {
        (2, 2)
    }
}

pub fn bpl(cpu: &mut CPU) -> (u8, u8)
{
    if cpu.registers.getN()
    {
        let addr = decode_zeropage(cpu).0 as u16;
        let new_pc = cpu.registers.PC.wrapping_add(addr);
        cpu.registers.PC = new_pc;

        (3, 0)
        // TODO add 1 if destination address on different page
    }
    else
    {
        (2, 2)
    }
}

pub fn bvs(cpu: &mut CPU) -> (u8, u8)
{
    if !cpu.registers.getV()
    {
        let addr = decode_zeropage(cpu).0 as u16;
        let new_pc = cpu.registers.PC.wrapping_add(addr);
        cpu.registers.PC = new_pc;

        (3, 0)
        // TODO add 1 if destination address on different page
    }
    else
    {
        (2, 2)
    }
}

pub fn bvc(cpu: &mut CPU) -> (u8, u8)
{
    if cpu.registers.getV()
    {
        let addr = decode_zeropage(cpu).0 as u16;
        let new_pc = cpu.registers.PC.wrapping_add(addr);
        cpu.registers.PC = new_pc;

        (3, 0)
        // TODO add 1 if destination address on different page
    }
    else
    {
        (2, 2)
    }
}
