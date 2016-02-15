use arch::instrs::*;
use arch::cpu::CPU;


pub static INSTR_TABLE: [Instr; 256] = [(others::brk, "brk::implied"), // 00
                                        (ora::indirect_x, "ora::indirect_x"), // 01
                                        (error_fn, "error_fn"), // 02
                                        (error_fn, "error_fn"), // 03
                                        (error_fn, "error_fn"), // 04
                                        (ora::zeropage, "ora::zeropage"), // 05
                                        (asl::zeropage, "asl::zeropage"), // 06
                                        (error_fn, "error_fn"), // 07
                                        (pushpop::php, "php::implied"), // 08
                                        (ora::immediate, "ora::immediate"), // 09
                                        (asl::accumulator, "asl::accumulator"), // 0a
                                        (error_fn, "error_fn"), // 0b
                                        (error_fn, "error_fn"), // 0c
                                        (ora::absolute, "ora::absolute"), // 0d
                                        (asl::absolute, "asl::absolute"), // 0e
                                        (error_fn, "error_fn"), // 0f
                                        (branches::bpl, "branches::bpl"), // 10
                                        (ora::indirect_y, "ora::indirect_y"), // 11
                                        (error_fn, "error_fn"), // 12
                                        (error_fn, "error_fn"), // 13
                                        (error_fn, "error_fn"), // 14
                                        (ora::zeropage_x, "ora::zeropage_x"), // 15
                                        (asl::zeropage_x, "asl::zeropage_x"), // 16
                                        (error_fn, "error_fn"), // 17
                                        (flags::clc, "clc::implied"), // 18
                                        (ora::absolute_y, "ora::absolute_y"), // 19
                                        (error_fn, "error_fn"), // 1a
                                        (error_fn, "error_fn"), // 1b
                                        (error_fn, "error_fn"), // 1c
                                        (ora::absolute_x, "ora::absolute_x"), // 1d
                                        (asl::absolute_x, "asl::absolute_x"), // 1e
                                        (error_fn, "error_fn"), // 1f
                                        (subroutines::jsr, "jsr::absolute"), // 20
                                        (and::indirect_x, "and::indirect_x"), // 21
                                        (error_fn, "error_fn"), // 22
                                        (error_fn, "error_fn"), // 23
                                        (bit::zeropage, "bit::zeropage"), // 24
                                        (and::zeropage, "and::zeropage"), // 25
                                        (rol::zeropage, "rol::zeropage"), // 26
                                        (error_fn, "error_fn"), // 27
                                        (pushpop::plp, "plp::implied"), // 28
                                        (and::immediate, "and::immediate"), // 29
                                        (rol::accumulator, "rol::accumulator"), // 2a
                                        (error_fn, "error_fn"), // 2b
                                        (bit::absolute, "bit::absolute"), // 2c
                                        (and::absolute, "and::absolute"), // 2d
                                        (rol::absolute, "rol::absolue"), // 2e
                                        (error_fn, "error_fn"), // 2f
                                        (branches::bmi, "branches::bmi"), // 30
                                        (and::indirect_y, "and::indirect_y"), // 31
                                        (error_fn, "error_fn"), // 32
                                        (error_fn, "error_fn"), // 33
                                        (error_fn, "error_fn"), // 34
                                        (and::zeropage_x, "and::zeropage_x"), // 35
                                        (rol::zeropage_x, "rol::zeropage_x"), // 36
                                        (error_fn, "error_fn"), // 37
                                        (flags::sec, "sec::implied"), // 38
                                        (and::absolute_y, "and::absolute_y"), // 39
                                        (error_fn, "error_fn"), // 3a
                                        (error_fn, "error_fn"), // 3b
                                        (error_fn, "error_fn"), // 3c
                                        (and::absolute_x, "and::absolute_x"), // 3d
                                        (rol::absolute_x, "rol::absolute_x"), // 3e
                                        (error_fn, "error_fn"), // 3f
                                        (subroutines::rti, "rti::absolute"), // 40
                                        (eor::indirect_x, "eor::indirect_x"), // 41
                                        (error_fn, "error_fn"), // 42
                                        (error_fn, "error_fn"), // 43
                                        (error_fn, "error_fn"), // 44
                                        (eor::zeropage, "eor::zeropage"), // 45
                                        (lsr::zeropage, "lsr::zeropage"), // 46
                                        (error_fn, "error_fn"), // 47
                                        (pushpop::pha, "pha::implied"), // 48
                                        (eor::immediate, "eor::immediate"), // 49
                                        (lsr::accumulator, "lsr::accumulator"), // 4a
                                        (error_fn, "error_fn"), // 4b
                                        (jmp::absolute, "jmp::absolute"), // 4c
                                        (eor::absolute, "eor::absolute"), // 4d
                                        (lsr::absolute, "lsr::absolute"), // 4e
                                        (error_fn, "error_fn"), // 4f
                                        (branches::bvc, "branches::bvc"), // 50
                                        (eor::indirect_y, "eor::indirect_y"), // 51
                                        (error_fn, "error_fn"), // 52
                                        (error_fn, "error_fn"), // 53
                                        (error_fn, "error_fn"), // 54
                                        (eor::zeropage_x, "eor::zeropage_x"), // 55
                                        (lsr::zeropage_x, "lsr::zeropage_x"), // 56
                                        (error_fn, "error_fn"), // 57
                                        (flags::cli, "cli::implied"), // 58
                                        (eor::absolute_y, "eor::absolute_y"), // 59
                                        (error_fn, "error_fn"), // 5a
                                        (error_fn, "error_fn"), // 5b
                                        (error_fn, "error_fn"), // 5c
                                        (eor::absolute_x, "eor::absolute_x"), // 5d
                                        (lsr::absolute_x, "lsr::absolute_x"), // 5e
                                        (error_fn, "error_fn"), // 5f
                                        (subroutines::rts, "rts::absolute"), // 60
                                        (adc::indirect_x, "adc::indirect_x"), // 61
                                        (error_fn, "error_fn"), // 62
                                        (error_fn, "error_fn"), // 63
                                        (error_fn, "error_fn"), // 64
                                        (adc::zeropage, "adc::zeropage"), // 65
                                        (ror::zeropage, "ror::zeropage"), // 66
                                        (error_fn, "error_fn"), // 67
                                        (pushpop::pla, "pla::implied"), // 68
                                        (adc::immediate, "adc::immediate"), // 69
                                        (ror::accumulator, "ror::accumulator"), // 6a
                                        (error_fn, "error_fn"), // 6b
                                        (jmp::indirect_absolute, "jmp::indirect_absolute"), // 6c
                                        (adc::absolute, "adc::absolute"), // 6d
                                        (ror::absolute, "ror::absolute"), // 6e
                                        (error_fn, "error_fn"), // 6f
                                        (branches::bvs, "branches::bvs"), // 70
                                        (adc::indirect_y, "adc::indirect_y"), // 71
                                        (error_fn, "error_fn"), // 72
                                        (error_fn, "error_fn"), // 73
                                        (error_fn, "error_fn"), // 74
                                        (adc::zeropage_x, "adc::zeropage_x"), // 75
                                        (ror::zeropage_x, "ror::zeropage_x"), // 76
                                        (error_fn, "error_fn"), // 77
                                        (flags::sei, "sei::implied"), // 78
                                        (adc::absolute_y, "adc::absolute_y"), // 79
                                        (error_fn, "error_fn"), // 7a
                                        (error_fn, "error_fn"), // 7b
                                        (error_fn, "error_fn"), // 7c
                                        (adc::absolute_x, "adc::absolute_x"), // 7d
                                        (ror::absolute_x, "ror::absolute_x"), // 7e
                                        (error_fn, "error_fn"), // 7f
                                        (error_fn, "error_fn"), // 80
                                        (sta::indirect_x, "sta::indirect_x"), // 81
                                        (error_fn, "error_fn"), // 82
                                        (error_fn, "error_fn"), // 83
                                        (sty::zeropage, "sty::zeropage"), // 84
                                        (sta::zeropage, "sta::zeropage"), // 85
                                        (stx::zeropage, "stx::zeropage"), // 86
                                        (error_fn, "error_fn"), // 87
                                        (dey::implied, "dey::implied"), // 88
                                        (error_fn, "error_fn"), // 89
                                        (transfers::txa, "txa::implied"), // 8a
                                        (error_fn, "error_fn"), // 8b
                                        (sty::absolute, "sty::absolute"), // 8c
                                        (sta::absolute, "sta::absolute"), // 8d
                                        (stx::absolute, "stx::absolute"), // 8e
                                        (error_fn, "error_fn"), // 8f
                                        (branches::bcc, "branches::bcc"), // 90
                                        (sta::indirect_y, "sta::indirect_y"), // 91
                                        (error_fn, "error_fn"), // 92
                                        (error_fn, "error_fn"), // 93
                                        (sty::zeropage_x, "sty::zeropage_x"), // 94
                                        (sta::zeropage_x, "sta::zeropage_x"), // 95
                                        (stx::zeropage_y, "stx::zeropage_y"), // 96
                                        (error_fn, "error_fn"), // 97
                                        (transfers::tya, "tya::implied"), // 98
                                        (sta::absolute_y, "sta::absolute_y"), // 99
                                        (transfers::txs, "txs::implied"), // 9a
                                        (error_fn, "error_fn"), // 9b
                                        (error_fn, "error_fn"), // 9c
                                        (sta::absolute_x, "sta::absolute_x"), // 9d
                                        (error_fn, "error_fn"), // 9e
                                        (error_fn, "error_fn"), // 9f
                                        (ldy::immediate, "ldy::immediate"), // a0
                                        (lda::indirect_x, "lda::indirect_x"), // a1
                                        (ldx::immediate, "ldx::immediate"), // a2
                                        (error_fn, "error_fn"), // a3
                                        (ldy::zeropage, "ldy::zeropage"), // a4
                                        (lda::zeropage, "lda::zeropage"), // a5
                                        (ldx::zeropage, "ldx::zeropage"), // a6
                                        (error_fn, "error_fn"), // a7
                                        (transfers::tay, "tay::implied"), // a8
                                        (lda::immediate, "lda::immediate"), // a9
                                        (transfers::tax, "tax::implied"), // aa
                                        (error_fn, "error_fn"), // ab
                                        (ldy::absolute, "ldy::absolute"), // ac
                                        (lda::absolute, "lda::absolute"), // ad
                                        (ldx::absolute, "ldx::absolute"), // ae
                                        (error_fn, "error_fn"), // af
                                        (branches::bcs, "branches::bcs"), // b0
                                        (lda::indirect_y, "lda::indirect_y"), // b1
                                        (error_fn, "error_fn"), // b2
                                        (error_fn, "error_fn"), // b3
                                        (ldy::zeropage_x, "ldy::zeropage_x"), // b4
                                        (lda::zeropage_x, "lda::zeropage_x"), // b5
                                        (ldx::zeropage_y, "ldx::zeropage_y"), // b6
                                        (error_fn, "error_fn"), // b7
                                        (flags::clv, "clv::implied"), // b8
                                        (lda::absolute_y, "lda::absolute_y"), // b9
                                        (transfers::tsx, "tsx::implied"), // ba
                                        (error_fn, "error_fn"), // bb
                                        (ldy::absolute_x, "ldy::absolute_x"), // bc
                                        (lda::absolute_x, "lda::absolute_x"), // bd
                                        (ldx::absolute_y, "ldx::absolute_y"), // be
                                        (error_fn, "error_fn"), // bf
                                        (cpy::immediate, "cpy::immediate"), // c0
                                        (cmp::indirect_x, "cmp::indirect_x"), // c1
                                        (error_fn, "error_fn"), // c2
                                        (error_fn, "error_fn"), // c3
                                        (cpy::zeropage, "cpy::zeropage"), // c4
                                        (cmp::zeropage, "cmp::zeropage"), // c5
                                        (dec::zeropage, "dec::zeropage"), // c6
                                        (error_fn, "error_fn"), // c7
                                        (iny::implied, "iny::implied"), // c8
                                        (cmp::immediate, "cmp::immediate"), // c9
                                        (dex::implied, "dex::implied"), // ca
                                        (error_fn, "error_fn"), // cb
                                        (cpy::absolute, "cpy::absolute"), // cc
                                        (cmp::absolute, "cmp::absolute"), // cd
                                        (dec::absolute, "dec::absolute"), // ce
                                        (error_fn, "error_fn"), // cf
                                        (branches::bne, "branches::bne"), // d0
                                        (cmp::indirect_y, "cmp::indirect_y"), // d1
                                        (error_fn, "error_fn"), // d2
                                        (error_fn, "error_fn"), // d3
                                        (error_fn, "error_fn"), // d4
                                        (cmp::zeropage_x, "cmp::zeropage_x"), // d5
                                        (dec::zeropage_x, "dec::zeropage_x"), // d6
                                        (error_fn, "error_fn"), // d7
                                        (flags::cld, "cld::implied"), // d8
                                        (cmp::absolute_y, "cmp::absolute_y"), // d9
                                        (error_fn, "error_fn"), // da
                                        (error_fn, "error_fn"), // db
                                        (error_fn, "error_fn"), // dc
                                        (cmp::absolute_x, "cmp::absolute_x"), // dd
                                        (dec::absolute_x, "dec::absolute_x"), // de
                                        (error_fn, "error_fn"), // df
                                        (cpx::immediate, "cpx::immediate"), // e0
                                        (sbc::indirect_x, "sbc::indirect_x"), // e1
                                        (error_fn, "error_fn"), // e2
                                        (error_fn, "error_fn"), // e3
                                        (cpx::zeropage, "cpx::zeropage"), // e4
                                        (sbc::zeropage, "sbc::zeropage"), // e5
                                        (inc::zeropage, "inc::zeropage"), // e6
                                        (error_fn, "error_fn"), // e7
                                        (inx::implied, "inx::implied"), // e8
                                        (sbc::immediate, "sbc::immediate"), // e9
                                        (others::nop, "nop::implied"), // ea
                                        (error_fn, "error_fn"), // eb
                                        (cpx::absolute, "cpx::absolute"), // ec
                                        (sbc::absolute, "sbc::absolute"), // ed
                                        (inc::absolute, "inc::absolute"), // ee
                                        (error_fn, "error_fn"), // ef
                                        (branches::beq, "branches::beq"), // f0
                                        (sbc::indirect_y, "sbc::indirect_y"), // f1
                                        (error_fn, "error_fn"), // f2
                                        (error_fn, "error_fn"), // f3
                                        (error_fn, "error_fn"), // f4
                                        (sbc::zeropage_x, "sbc::zeropage_x"), // f5
                                        (inc::zeropage_x, "inc::zeropage_x"), // f6
                                        (error_fn, "error_fn"), // f7
                                        (flags::sed, "sed::implied"), // f8
                                        (sbc::absolute_y, "sbc::absolute_y"), // f9
                                        (error_fn, "error_fn"), // fa
                                        (error_fn, "error_fn"), // fb
                                        (error_fn, "error_fn"), // fc
                                        (sbc::absolute_x, "sbc::absolute_x"), // fd
                                        (inc::absolute_x, "inc::absolute_x"), // fe
                                        (error_fn, "error_fn") /* ff */];

pub type Instr = (fn(&mut CPU) -> (u8, u8), &'static str);

pub fn error_fn(_cpu: &mut CPU) -> (u8, u8) {
    // panic!("Invalid opcode!");
    (0xFF, 0xFF)
}

// decode functions

#[macro_export]
macro_rules! decode_absolute {
    ( $cpu:expr ) => {{      
        let low = $cpu.memory.fetch($cpu.registers.PC + 1);
        let high = $cpu.memory.fetch($cpu.registers.PC + 2);
        (to_u16(low, high), 3)
    }};
}

#[macro_export]
macro_rules! decode_immediate {
    ( $cpu:expr ) => {{
        ($cpu.memory.fetch($cpu.registers.PC + 1), 2)
    }};
}

#[macro_export]
macro_rules! decode_zeropage {
    ( $cpu:expr ) => {{
        ($cpu.memory.fetch($cpu.registers.PC + 1), 2)
    }};
}

#[macro_export]
macro_rules! decode_absolute_indexed {
    ( $cpu:expr, $offset:expr ) => {{
        let low = $cpu.memory.fetch($cpu.registers.PC + 1);
        let high = $cpu.memory.fetch($cpu.registers.PC + 2);
        (to_u16(low, high).wrapping_add($offset as u16), 3)
    }};
}

#[macro_export]
macro_rules! decode_zeropage_indexed {
    ( $cpu:expr, $offset:expr ) => {{
        let addr = $cpu.memory.fetch($cpu.registers.PC + 1);
        (addr.wrapping_add($offset), 2)
    }};
}

#[macro_export]
macro_rules! decode_indexed_indirect {
    ( $cpu:expr ) => {{
        let op = ($cpu.memory.fetch($cpu.registers.PC + 1).wrapping_add($cpu.registers.X)) as u16 & 0xFF;
        let low = $cpu.memory.fetch(op);
        let high = $cpu.memory.fetch((op + 1) & 0xFF);

        (to_u16(low, high), 2)
    }};
}

#[macro_export]
macro_rules! decode_indirect_indexed {
    ( $cpu:expr ) => {{
        let op = $cpu.memory.fetch($cpu.registers.PC + 1) as u16;
        let low = $cpu.memory.fetch(op);
        let high = $cpu.memory.fetch((op + 1) & 0xFF);

        (to_u16(low, high).wrapping_add($cpu.registers.Y as u16), 2)
    }};
}
