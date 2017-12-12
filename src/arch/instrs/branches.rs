use arch::cpu::CPU;

pub fn bcc(cpu: &mut CPU) -> (u8, u8) {
    if !cpu.registers.get_c() {
        let addr = decode_zeropage!(cpu).0 as i8;
        let new_pc = cpu.registers.pc.wrapping_add(addr as u16);
        cpu.registers.pc = new_pc;

        (3, 0)
        // TODO add 1 if destination address on different page
    } else {
        (2, 2)
    }
}

pub fn bcs(cpu: &mut CPU) -> (u8, u8) {
    if cpu.registers.get_c() {
        let addr = decode_zeropage!(cpu).0 as i8;
        let new_pc = cpu.registers.pc.wrapping_add(addr as u16);
        cpu.registers.pc = new_pc;

        (3, 0)
        // TODO add 1 if destination address on different page
    } else {
        (2, 2)
    }
}

pub fn beq(cpu: &mut CPU) -> (u8, u8) {
    if !cpu.registers.get_z() {
        let addr = decode_zeropage!(cpu).0 as i8;
        let new_pc = cpu.registers.pc.wrapping_add(addr as u16);
        cpu.registers.pc = new_pc;

        (3, 0)
        // TODO add 1 if destination address on different page
    } else {
        (2, 2)
    }
}

pub fn bne(cpu: &mut CPU) -> (u8, u8) {
    if cpu.registers.get_z() {
        let addr = decode_zeropage!(cpu).0 as i8;
        let new_pc = cpu.registers.pc.wrapping_add(addr as u16);
        cpu.registers.pc = new_pc;

        (3, 0)
        // TODO add 1 if destination address on different page
    } else {
        (2, 2)
    }
}

pub fn bmi(cpu: &mut CPU) -> (u8, u8) {
    if !cpu.registers.get_n() {
        let addr = decode_zeropage!(cpu).0 as i8;
        let new_pc = cpu.registers.pc.wrapping_add(addr as u16);
        cpu.registers.pc = new_pc;

        (3, 0)
        // TODO add 1 if destination address on different page
    } else {
        (2, 2)
    }
}

pub fn bpl(cpu: &mut CPU) -> (u8, u8) {
    if cpu.registers.get_n() {
        let addr = decode_zeropage!(cpu).0 as i8;
        let new_pc = cpu.registers.pc.wrapping_add(addr as u16);
        cpu.registers.pc = new_pc;

        (3, 0)
        // TODO add 1 if destination address on different page
    } else {
        (2, 2)
    }
}

pub fn bvs(cpu: &mut CPU) -> (u8, u8) {
    if !cpu.registers.get_v() {
        let addr = decode_zeropage!(cpu).0 as i8;
        let new_pc = cpu.registers.pc.wrapping_add(addr as u16);
        cpu.registers.pc = new_pc;

        (3, 0)
        // TODO add 1 if destination address on different page
    } else {
        (2, 2)
    }
}

pub fn bvc(cpu: &mut CPU) -> (u8, u8) {
    if cpu.registers.get_v() {
        let addr = decode_zeropage!(cpu).0 as i8;
        let new_pc = cpu.registers.pc.wrapping_add(addr as u16);
        cpu.registers.pc = new_pc;

        (3, 0)
        // TODO add 1 if destination address on different page
    } else {
        (2, 2)
    }
}
