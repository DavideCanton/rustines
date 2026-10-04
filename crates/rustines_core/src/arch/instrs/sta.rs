use crate::arch::{bus::Bus, cpu::Cpu};

pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage(bus);
    bus.write(addr as u16, cpu.registers.a_reg);
    3
}

pub fn zeropage_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_zeropage_indexed(bus, cpu.registers.x_reg);
    bus.write(addr as u16, cpu.registers.a_reg);
    4
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_absolute(bus);
    bus.write(addr, cpu.registers.a_reg);
    4
}

pub fn absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu
        .decode_absolute_indexed(bus, cpu.registers.x_reg, true)
        .address();
    bus.write(addr, cpu.registers.a_reg);
    5
}

pub fn absolute_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu
        .decode_absolute_indexed(bus, cpu.registers.y_reg, true)
        .address();
    bus.write(addr, cpu.registers.a_reg);
    5
}

pub fn indirect_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_indexed_indirect(bus);
    bus.write(addr, cpu.registers.a_reg);
    6
}

pub fn indirect_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = cpu.decode_indirect_indexed(bus, true).address();
    bus.write(addr, cpu.registers.a_reg);
    6
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arch::arch_tests::setup_tests;

    fn assert_sta(
        bus: &mut Bus,
        address: u16,
        expected: u8,
        expected_cycles: u8,
        actual_cycles: u8,
    ) {
        assert_eq!(actual_cycles, expected_cycles);
        assert_eq!(bus.read(address), expected);
    }

    #[test]
    fn test_sta_zeropage_stores_a_reg() {
        let (mut cpu, mut bus) = setup_tests();

        cpu.registers.a_reg = 0xAB;
        bus.write(cpu.registers.pc, 0x85);
        bus.write(cpu.registers.pc + 1, 0xDE);
        cpu.registers.pc += 1;

        let cycles = zeropage(&mut cpu, &mut bus);

        assert_sta(&mut bus, 0x00DE, 0xAB, 3, cycles);
    }

    #[test]
    fn test_sta_zeropage_x_stores_a_reg_with_offset() {
        let (mut cpu, mut bus) = setup_tests();

        cpu.registers.a_reg = 0xAB;
        cpu.registers.x_reg = 0x10;
        bus.write(cpu.registers.pc, 0x95);
        bus.write(cpu.registers.pc + 1, 0xDE);
        cpu.registers.pc += 1;

        let cycles = zeropage_x(&mut cpu, &mut bus);

        assert_sta(&mut bus, 0x00EE, 0xAB, 4, cycles);
    }

    #[test]
    fn test_sta_zeropage_x_wraps_zero_page_address() {
        let (mut cpu, mut bus) = setup_tests();

        cpu.registers.a_reg = 0xAB;
        cpu.registers.x_reg = 0x01;
        bus.write(cpu.registers.pc, 0x95);
        bus.write(cpu.registers.pc + 1, 0xFF);
        cpu.registers.pc += 1;

        let cycles = zeropage_x(&mut cpu, &mut bus);

        assert_sta(&mut bus, 0x0000, 0xAB, 4, cycles);
    }

    #[test]
    fn test_sta_absolute_stores_a_reg() {
        let (mut cpu, mut bus) = setup_tests();

        cpu.registers.a_reg = 0xAB;
        bus.write(cpu.registers.pc, 0x8D);
        bus.write(cpu.registers.pc + 1, 0x34);
        bus.write(cpu.registers.pc + 2, 0x12);
        cpu.registers.pc += 1;

        let cycles = absolute(&mut cpu, &mut bus);

        assert_sta(&mut bus, 0x1234, 0xAB, 4, cycles);
    }

    #[test]
    fn test_sta_absolute_x_stores_a_reg() {
        let (mut cpu, mut bus) = setup_tests();

        cpu.registers.a_reg = 0xAB;
        cpu.registers.x_reg = 0x10;
        bus.write(cpu.registers.pc, 0x9D);
        bus.write(cpu.registers.pc + 1, 0x34);
        bus.write(cpu.registers.pc + 2, 0x12);
        cpu.registers.pc += 1;

        let cycles = absolute_x(&mut cpu, &mut bus);

        assert_sta(&mut bus, 0x1244, 0xAB, 5, cycles);
    }

    #[test]
    fn test_sta_absolute_x_wraps_page_when_indexed() {
        let (mut cpu, mut bus) = setup_tests();

        cpu.registers.a_reg = 0xAB;
        cpu.registers.x_reg = 0x03;
        bus.write(cpu.registers.pc, 0x9D);
        bus.write(cpu.registers.pc + 1, 0xFE);
        bus.write(cpu.registers.pc + 2, 0xFF);
        cpu.registers.pc += 1;

        let cycles = absolute_x(&mut cpu, &mut bus);

        assert_sta(&mut bus, 0x0001, 0xAB, 5, cycles);
    }

    #[test]
    fn test_sta_absolute_y_stores_a_reg() {
        let (mut cpu, mut bus) = setup_tests();

        cpu.registers.a_reg = 0xAB;
        cpu.registers.y_reg = 0x10;
        bus.write(cpu.registers.pc, 0x99);
        bus.write(cpu.registers.pc + 1, 0x34);
        bus.write(cpu.registers.pc + 2, 0x12);
        cpu.registers.pc += 1;

        let cycles = absolute_y(&mut cpu, &mut bus);

        assert_sta(&mut bus, 0x1244, 0xAB, 5, cycles);
    }

    #[test]
    fn test_sta_absolute_y_wraps_page_when_indexed() {
        let (mut cpu, mut bus) = setup_tests();

        cpu.registers.a_reg = 0xAB;
        cpu.registers.y_reg = 0x03;
        bus.write(cpu.registers.pc, 0x99);
        bus.write(cpu.registers.pc + 1, 0xFE);
        bus.write(cpu.registers.pc + 2, 0xFF);
        cpu.registers.pc += 1;

        let cycles = absolute_y(&mut cpu, &mut bus);

        assert_sta(&mut bus, 0x0001, 0xAB, 5, cycles);
    }

    #[test]
    fn test_sta_indirect_x_stores_a_reg() {
        let (mut cpu, mut bus) = setup_tests();

        cpu.registers.a_reg = 0xAB;
        cpu.registers.x_reg = 0x10;
        bus.write(cpu.registers.pc, 0x81);
        bus.write(cpu.registers.pc + 1, 0x34);
        cpu.registers.pc += 1;

        bus.write(0x44, 0x10);
        bus.write(0x45, 0x11);

        let cycles = indirect_x(&mut cpu, &mut bus);

        assert_sta(&mut bus, 0x1110, 0xAB, 6, cycles);
    }

    #[test]
    fn test_sta_indirect_x_wraps_zero_page_address() {
        let (mut cpu, mut bus) = setup_tests();

        cpu.registers.a_reg = 0xAB;
        cpu.registers.x_reg = 0x10;
        bus.write(cpu.registers.pc, 0x81);
        bus.write(cpu.registers.pc + 1, 0xFE);
        cpu.registers.pc += 1;

        bus.write(0x0E, 0x10);
        bus.write(0x0F, 0x11);

        let cycles = indirect_x(&mut cpu, &mut bus);

        assert_sta(&mut bus, 0x1110, 0xAB, 6, cycles);
    }

    #[test]
    fn test_sta_indirect_y_stores_a_reg() {
        let (mut cpu, mut bus) = setup_tests();

        cpu.registers.a_reg = 0xAB;
        cpu.registers.y_reg = 0x10;
        bus.write(cpu.registers.pc, 0x91);
        bus.write(cpu.registers.pc + 1, 0x34);
        cpu.registers.pc += 1;

        bus.write(0x34, 0x10);
        bus.write(0x35, 0x11);

        let cycles = indirect_y(&mut cpu, &mut bus);

        assert_sta(&mut bus, 0x1120, 0xAB, 6, cycles);
    }

    #[test]
    fn test_sta_indirect_y_wraps_page_when_indexed() {
        let (mut cpu, mut bus) = setup_tests();

        cpu.registers.a_reg = 0xAB;
        cpu.registers.y_reg = 0x20;
        bus.write(cpu.registers.pc, 0x91);
        bus.write(cpu.registers.pc + 1, 0x0E);
        cpu.registers.pc += 1;

        bus.write(0x0E, 0xFE);
        bus.write(0x0F, 0xFF);

        let cycles = indirect_y(&mut cpu, &mut bus);

        assert_sta(&mut bus, 0x001E, 0xAB, 6, cycles);
    }
}
