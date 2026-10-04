use crate::arch::{bus::Bus, cpu::Cpu};

pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let val = cpu.decode_immediate(bus);
    do_lda(cpu, val);
    2
}

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);
    let val = bus.read(addr as u16);
    do_lda(cpu, val);
    3
}

pub fn zeropage_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage_indexed(bus, cpu.registers.x_reg);
    let val = bus.read(addr as u16);
    do_lda(cpu, val);
    4
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    let val = bus.read(addr);
    do_lda(cpu, val);
    4
}

pub fn absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let result = cpu.decode_absolute_indexed(bus, cpu.registers.x_reg, false);
    let addr = result.address();
    let val = bus.read(addr);
    do_lda(cpu, val);
    4 + result.page_boundary_crossed_as_u8()
}

pub fn absolute_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let result = cpu.decode_absolute_indexed(bus, cpu.registers.y_reg, false);
    let addr = result.address();
    let val = bus.read(addr);
    do_lda(cpu, val);
    4 + result.page_boundary_crossed_as_u8()
}

pub fn indirect_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_indexed_indirect(bus);
    let val = bus.read(addr);
    do_lda(cpu, val);
    6
}

pub fn indirect_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let result = cpu.decode_indirect_indexed(bus, false);
    let addr = result.address();
    let val = bus.read(addr);
    do_lda(cpu, val);
    5 + result.page_boundary_crossed_as_u8()
}

fn do_lda(cpu: &mut Cpu, val: u8) {
    cpu.registers.a_reg = val;
    cpu.registers.update_nz_flags(val);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arch::arch_tests::setup_tests;

    #[test]
    fn test_lda_immediate() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc, 0xa9);
        bus.write(cpu.registers.pc + 1, 0xDE);

        cpu.registers.pc += 1;

        let cycles = immediate(&mut cpu, &mut bus);

        assert_eq!(2, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xDE);
    }

    #[test]
    fn test_lda_zeropage() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc, 0xa5);
        bus.write(cpu.registers.pc + 1, 0xDE);
        bus.write(0xDE, 0xAB);

        cpu.registers.pc += 1;

        let cycles = zeropage(&mut cpu, &mut bus);

        assert_eq!(3, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_zeropage_x() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc, 0xb5);
        bus.write(cpu.registers.pc + 1, 0xDE);
        bus.write(0xEE, 0xAB);

        cpu.registers.pc += 1;

        cpu.registers.x_reg = 0x10;

        let cycles = zeropage_x(&mut cpu, &mut bus);

        assert_eq!(4, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_zeropage_x_flipping() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc, 0xb5);
        bus.write(cpu.registers.pc + 1, 0xFF);
        bus.write(0x0, 0xAB);

        cpu.registers.pc += 1;

        cpu.registers.x_reg = 0x1;

        let cycles = zeropage_x(&mut cpu, &mut bus);

        assert_eq!(4, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_absolute() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc, 0xad);
        bus.write(cpu.registers.pc + 1, 0x34);
        bus.write(cpu.registers.pc + 2, 0x12);
        bus.write(0x1234, 0xAB);

        cpu.registers.pc += 1;

        let cycles = absolute(&mut cpu, &mut bus);

        assert_eq!(4, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_absolute_x() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc, 0xbd);
        bus.write(cpu.registers.pc + 1, 0x34);
        bus.write(cpu.registers.pc + 2, 0x12);
        bus.write(0x1244, 0xAB);

        cpu.registers.pc += 1;

        cpu.registers.x_reg = 0x10;

        let cycles = absolute_x(&mut cpu, &mut bus);

        assert_eq!(4, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_absolute_x_flipping() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc, 0xbd);
        bus.write(cpu.registers.pc + 1, 0xFE);
        bus.write(cpu.registers.pc + 2, 0xFF);

        cpu.registers.pc += 1;

        bus.write(0x0001, 0xAB);

        cpu.registers.x_reg = 0x3;

        let cycles = absolute_x(&mut cpu, &mut bus);

        assert_eq!(5, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_absolute_y() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc, 0xb9);
        bus.write(cpu.registers.pc + 1, 0x34);
        bus.write(cpu.registers.pc + 2, 0x12);

        cpu.registers.pc += 1;

        bus.write(0x1244, 0xAB);

        cpu.registers.y_reg = 0x10;

        let cycles = absolute_y(&mut cpu, &mut bus);

        assert_eq!(4, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_absolute_y_flipping() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc, 0xb9);
        bus.write(cpu.registers.pc + 1, 0xFE);
        bus.write(cpu.registers.pc + 2, 0xFF);

        cpu.registers.pc += 1;

        bus.write(0x0001, 0xAB);

        cpu.registers.y_reg = 0x3;

        let cycles = absolute_y(&mut cpu, &mut bus);

        assert_eq!(5, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_indirect_x() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc, 0xa1);
        bus.write(cpu.registers.pc + 1, 0x34);
        cpu.registers.pc += 1;

        bus.write(0x44, 0x10);
        bus.write(0x45, 0x11);

        bus.write(0x1110, 0xAB);

        cpu.registers.x_reg = 0x10;

        let cycles = indirect_x(&mut cpu, &mut bus);

        assert_eq!(6, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_indirect_x_flipping() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc, 0xa1);
        bus.write(cpu.registers.pc + 1, 0xFE);
        cpu.registers.pc += 1;

        bus.write(0x0E, 0x10);
        bus.write(0x0F, 0x11);

        bus.write(0x1110, 0xAB);

        cpu.registers.x_reg = 0x10;

        let cycles = indirect_x(&mut cpu, &mut bus);

        assert_eq!(6, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_indirect_y() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc, 0xb1);
        bus.write(cpu.registers.pc + 1, 0x34);
        cpu.registers.pc += 1;

        bus.write(0x34, 0x10);
        bus.write(0x35, 0x11);

        bus.write(0x1120, 0xAB);

        cpu.registers.y_reg = 0x10;

        let cycles = indirect_y(&mut cpu, &mut bus);

        assert_eq!(5, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }

    #[test]
    fn test_lda_indirect_y_flipping() {
        let (mut cpu, mut bus) = setup_tests();

        bus.write(cpu.registers.pc, 0xb1);
        bus.write(cpu.registers.pc + 1, 0x0E);
        cpu.registers.pc += 1;

        bus.write(0x0E, 0xFE);
        bus.write(0x0F, 0xFF);

        bus.write(0x001E, 0xAB);

        cpu.registers.y_reg = 0x20;

        let cycles = indirect_y(&mut cpu, &mut bus);

        assert_eq!(6, cycles);

        let val = cpu.registers.a_reg;
        assert_eq!(val, 0xAB);
    }
}
