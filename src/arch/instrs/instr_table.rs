use crate::arch::cpu::Cpu;
use crate::arch::instrs::*;
use crate::utils::tls::Syncify;
use lazy_static::*;
use log::debug;

lazy_static! {
pub static ref INSTR_TABLE: Syncify<[Instr; 256]> = {
    unsafe { Syncify::new ([
        Instr::new(Box::new(others::brk), "brk::implied", 0 ), // 00
        Instr::new(Box::new(ora::indirect_x), "ora::indirect_x", 2 ), // 01
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 02
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 03
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 04
        Instr::new(Box::new(ora::zeropage), "ora::zeropage", 2 ), // 05
        Instr::new(Box::new(asl::zeropage), "asl::zeropage", 2 ), // 06
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 07
        Instr::new(Box::new(pushpop::php), "php::implied", 1 ), // 08
        Instr::new(Box::new(ora::immediate), "ora::immediate", 2 ), // 09
        Instr::new(Box::new(asl::accumulator), "asl::accumulator", 1 ), // 0a
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 0b
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 0c
        Instr::new(Box::new(ora::absolute), "ora::absolute", 3 ), // 0d
        Instr::new(Box::new(asl::absolute), "asl::absolute", 3 ), // 0e
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 0f
        Instr::new(Box::new(branches::bpl), "bpl", 2 ), // 10
        Instr::new(Box::new(ora::indirect_y), "ora::indirect_y", 2 ), // 11
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 12
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 13
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 14
        Instr::new(Box::new(ora::zeropage_x), "ora::zeropage_x", 2 ), // 15
        Instr::new(Box::new(asl::zeropage_x), "asl::zeropage_x", 2 ), // 16
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 17
        Instr::new(Box::new(flags::clc), "clc::implied", 1 ), // 18
        Instr::new(Box::new(ora::absolute_y), "ora::absolute_y", 3 ), // 19
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 1a
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 1b
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 1c
        Instr::new(Box::new(ora::absolute_x), "ora::absolute_x", 3 ), // 1d
        Instr::new(Box::new(asl::absolute_x), "asl::absolute_x", 3 ), // 1e
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 1f
        Instr::new(Box::new(subroutines::jsr), "jsr::absolute", 0 ), // 20
        Instr::new(Box::new(and::indirect_x), "and::indirect_x", 2 ), // 21
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 22
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 23
        Instr::new(Box::new(bit::zeropage), "bit::zeropage", 2 ), // 24
        Instr::new(Box::new(and::zeropage), "and::zeropage", 2 ), // 25
        Instr::new(Box::new(rol::zeropage), "rol::zeropage", 2 ), // 26
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 27
        Instr::new(Box::new(pushpop::plp), "plp::implied", 1 ), // 28
        Instr::new(Box::new(and::immediate), "and::immediate", 2 ), // 29
        Instr::new(Box::new(rol::accumulator), "rol::accumulator", 1 ), // 2a
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 2b
        Instr::new(Box::new(bit::absolute), "bit::absolute", 3 ), // 2c
        Instr::new(Box::new(and::absolute), "and::absolute", 3 ), // 2d
        Instr::new(Box::new(rol::absolute), "rol::absolute", 3 ), // 2e
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 2f
        Instr::new(Box::new(branches::bmi), "bmi", 0 ), // 30
        Instr::new(Box::new(and::indirect_y), "and::indirect_y", 2 ), // 31
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 32
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 33
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 34
        Instr::new(Box::new(and::zeropage_x), "and::zeropage_x", 2 ), // 35
        Instr::new(Box::new(rol::zeropage_x), "rol::zeropage_x", 2 ), // 36
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 37
        Instr::new(Box::new(flags::sec), "sec::implied", 1 ), // 38
        Instr::new(Box::new(and::absolute_y), "and::absolute_y", 3 ), // 39
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 3a
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 3b
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 3c
        Instr::new(Box::new(and::absolute_x), "and::absolute_x", 3 ), // 3d
        Instr::new(Box::new(rol::absolute_x), "rol::absolute_x", 3 ), // 3e
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 3f
        Instr::new(Box::new(subroutines::rti), "rti::absolute", 0 ), // 40
        Instr::new(Box::new(eor::indirect_x), "eor::indirect_x", 2 ), // 41
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 42
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 43
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 44
        Instr::new(Box::new(eor::zeropage), "eor::zeropage", 2 ), // 45
        Instr::new(Box::new(lsr::zeropage), "lsr::zeropage", 2 ), // 46
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 47
        Instr::new(Box::new(pushpop::pha), "pha::implied", 1 ), // 48
        Instr::new(Box::new(eor::immediate), "eor::immediate", 2 ), // 49
        Instr::new(Box::new(lsr::accumulator), "lsr::accumulator", 1 ), // 4a
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 4b
        Instr::new(Box::new(jmp::absolute), "jmp::absolute", 0 ), // 4c
        Instr::new(Box::new(eor::absolute), "eor::absolute", 3 ), // 4d
        Instr::new(Box::new(lsr::absolute), "lsr::absolute", 3 ), // 4e
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 4f
        Instr::new(Box::new(branches::bvc), "bvc", 2 ), // 50
        Instr::new(Box::new(eor::indirect_y), "eor::indirect_y", 2 ), // 51
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 52
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 53
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 54
        Instr::new(Box::new(eor::zeropage_x), "eor::zeropage_x", 2 ), // 55
        Instr::new(Box::new(lsr::zeropage_x), "lsr::zeropage_x", 2 ), // 56
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 57
        Instr::new(Box::new(flags::cli), "cli::implied", 1 ), // 58
        Instr::new(Box::new(eor::absolute_y), "eor::absolute_y", 3 ), // 59
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 5a
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 5b
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 5c
        Instr::new(Box::new(eor::absolute_x), "eor::absolute_x", 3 ), // 5d
        Instr::new(Box::new(lsr::absolute_x), "lsr::absolute_x", 3 ), // 5e
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 5f
        Instr::new(Box::new(subroutines::rts), "rts::absolute", 1 ), // 60
        Instr::new(Box::new(adc::indirect_x), "adc::indirect_x", 2 ), // 61
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 62
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 63
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 64
        Instr::new(Box::new(adc::zeropage), "adc::zeropage", 2 ), // 65
        Instr::new(Box::new(ror::zeropage), "ror::zeropage", 2 ), // 66
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 67
        Instr::new(Box::new(pushpop::pla), "pla::implied", 1 ), // 68
        Instr::new(Box::new(adc::immediate), "adc::immediate", 2 ), // 69
        Instr::new(Box::new(ror::accumulator), "ror::accumulator", 1 ), // 6a
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 6b
        Instr::new(Box::new(jmp::indirect_absolute), "jmp::indirect_absolute", 0 ), // 6c
        Instr::new(Box::new(adc::absolute), "adc::absolute", 3 ), // 6d
        Instr::new(Box::new(ror::absolute), "ror::absolute", 3 ), // 6e
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 6f
        Instr::new(Box::new(branches::bvs), "bvs", 0 ), // 70
        Instr::new(Box::new(adc::indirect_y), "adc::indirect_y", 2 ), // 71
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 72
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 73
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 74
        Instr::new(Box::new(adc::zeropage_x), "adc::zeropage_x", 2 ), // 75
        Instr::new(Box::new(ror::zeropage_x), "ror::zeropage_x", 2 ), // 76
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 77
        Instr::new(Box::new(flags::sei), "sei::implied", 1 ), // 78
        Instr::new(Box::new(adc::absolute_y), "adc::absolute_y", 3 ), // 79
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 7a
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 7b
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 7c
        Instr::new(Box::new(adc::absolute_x), "adc::absolute_x", 3 ), // 7d
        Instr::new(Box::new(ror::absolute_x), "ror::absolute_x", 3 ), // 7e
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 7f
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 80
        Instr::new(Box::new(sta::indirect_x), "sta::indirect_x", 2 ), // 81
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 82
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 83
        Instr::new(Box::new(sty::zeropage), "sty::zeropage", 2 ), // 84
        Instr::new(Box::new(sta::zeropage), "sta::zeropage", 2 ), // 85
        Instr::new(Box::new(stx::zeropage), "stx::zeropage", 2 ), // 86
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 87
        Instr::new(Box::new(dey::implied), "dey::implied", 1 ), // 88
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 89
        Instr::new(Box::new(transfers::txa), "txa::implied", 1 ), // 8a
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 8b
        Instr::new(Box::new(sty::absolute), "sty::absolute", 3 ), // 8c
        Instr::new(Box::new(sta::absolute), "sta::absolute", 3 ), // 8d
        Instr::new(Box::new(stx::absolute), "stx::absolute", 3 ), // 8e
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 8f
        Instr::new(Box::new(branches::bcc), "bcc", 0 ), // 90
        Instr::new(Box::new(sta::indirect_y), "sta::indirect_y", 2 ), // 91
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 92
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 93
        Instr::new(Box::new(sty::zeropage_x), "sty::zeropage_x", 2 ), // 94
        Instr::new(Box::new(sta::zeropage_x), "sta::zeropage_x", 2 ), // 95
        Instr::new(Box::new(stx::zeropage_y), "stx::zeropage_y", 2 ), // 96
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 97
        Instr::new(Box::new(transfers::tya), "tya::implied", 1 ), // 98
        Instr::new(Box::new(sta::absolute_y), "sta::absolute_y", 3 ), // 99
        Instr::new(Box::new(transfers::txs), "txs::implied", 1 ), // 9a
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 9b
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 9c
        Instr::new(Box::new(sta::absolute_x), "sta::absolute_x", 3 ), // 9d
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 9e
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // 9f
        Instr::new(Box::new(ldy::immediate), "ldy::immediate", 2 ), // a0
        Instr::new(Box::new(lda::indirect_x), "lda::indirect_x", 2 ), // a1
        Instr::new(Box::new(ldx::immediate), "ldx::immediate", 2 ), // a2
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // a3
        Instr::new(Box::new(ldy::zeropage), "ldy::zeropage", 2 ), // a4
        Instr::new(Box::new(lda::zeropage), "lda::zeropage", 2 ), // a5
        Instr::new(Box::new(ldx::zeropage), "ldx::zeropage", 2 ), // a6
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // a7
        Instr::new(Box::new(transfers::tay), "tay::implied", 1 ), // a8
        Instr::new(Box::new(lda::immediate), "lda::immediate", 2 ), // a9
        Instr::new(Box::new(transfers::tax), "tax::implied", 1 ), // aa
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // ab
        Instr::new(Box::new(ldy::absolute), "ldy::absolute", 3 ), // ac
        Instr::new(Box::new(lda::absolute), "lda::absolute", 3 ), // ad
        Instr::new(Box::new(ldx::absolute), "ldx::absolute", 3 ), // ae
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // af
        Instr::new(Box::new(branches::bcs), "bcs", 2 ), // b0
        Instr::new(Box::new(lda::indirect_y), "lda::indirect_y", 2 ), // b1
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // b2
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // b3
        Instr::new(Box::new(ldy::zeropage_x), "ldy::zeropage_x", 2 ), // b4
        Instr::new(Box::new(lda::zeropage_x), "lda::zeropage_x", 2 ), // b5
        Instr::new(Box::new(ldx::zeropage_y), "ldx::zeropage_y", 2 ), // b6
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // b7
        Instr::new(Box::new(flags::clv), "clv::implied", 1 ), // b8
        Instr::new(Box::new(lda::absolute_y), "lda::absolute_y", 3 ), // b9
        Instr::new(Box::new(transfers::tsx), "tsx::implied", 1 ), // ba
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // bb
        Instr::new(Box::new(ldy::absolute_x), "ldy::absolute_x", 3 ), // bc
        Instr::new(Box::new(lda::absolute_x), "lda::absolute_x", 3 ), // bd
        Instr::new(Box::new(ldx::absolute_y), "ldx::absolute_y", 3 ), // be
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // bf
        Instr::new(Box::new(cpy::immediate), "cpy::immediate", 2 ), // c0
        Instr::new(Box::new(cmp::indirect_x), "cmp::indirect_x", 2 ), // c1
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // c2
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // c3
        Instr::new(Box::new(cpy::zeropage), "cpy::zeropage", 2 ), // c4
        Instr::new(Box::new(cmp::zeropage), "cmp::zeropage", 2 ), // c5
        Instr::new(Box::new(dec::zeropage), "dec::zeropage", 2 ), // c6
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // c7
        Instr::new(Box::new(iny::implied), "iny::implied", 1 ), // c8
        Instr::new(Box::new(cmp::immediate), "cmp::immediate", 2 ), // c9
        Instr::new(Box::new(dex::implied), "dex::implied", 1 ), // ca
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // cb
        Instr::new(Box::new(cpy::absolute), "cpy::absolute", 3 ), // cc
        Instr::new(Box::new(cmp::absolute), "cmp::absolute", 3 ), // cd
        Instr::new(Box::new(dec::absolute), "dec::absolute", 3 ), // ce
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // cf
        Instr::new(Box::new(branches::bne), "bne", 2 ), // d0
        Instr::new(Box::new(cmp::indirect_y), "cmp::indirect_y", 2 ), // d1
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // d2
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // d3
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // d4
        Instr::new(Box::new(cmp::zeropage_x), "cmp::zeropage_x", 2 ), // d5
        Instr::new(Box::new(dec::zeropage_x), "dec::zeropage_x", 2 ), // d6
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // d7
        Instr::new(Box::new(flags::cld), "cld::implied", 1 ), // d8
        Instr::new(Box::new(cmp::absolute_y), "cmp::absolute_y", 3 ), // d9
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // da
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // db
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // dc
        Instr::new(Box::new(cmp::absolute_x), "cmp::absolute_x", 3 ), // dd
        Instr::new(Box::new(dec::absolute_x), "dec::absolute_x", 3 ), // de
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // df
        Instr::new(Box::new(cpx::immediate), "cpx::immediate", 2 ), // e0
        Instr::new(Box::new(sbc::indirect_x), "sbc::indirect_x", 2 ), // e1
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // e2
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // e3
        Instr::new(Box::new(cpx::zeropage), "cpx::zeropage", 2 ), // e4
        Instr::new(Box::new(sbc::zeropage), "sbc::zeropage", 2 ), // e5
        Instr::new(Box::new(inc::zeropage), "inc::zeropage", 2 ), // e6
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // e7
        Instr::new(Box::new(inx::implied), "inx::implied", 1 ), // e8
        Instr::new(Box::new(sbc::immediate), "sbc::immediate", 2 ), // e9
        Instr::new(Box::new(others::nop), "nop::implied", 1 ), // ea
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // eb
        Instr::new(Box::new(cpx::absolute), "cpx::absolute", 3 ), // ec
        Instr::new(Box::new(sbc::absolute), "sbc::absolute", 3 ), // ed
        Instr::new(Box::new(inc::absolute), "inc::absolute", 3 ), // ee
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // ef
        Instr::new(Box::new(branches::beq), "beq", 0 ), // f0
        Instr::new(Box::new(sbc::indirect_y), "sbc::indirect_y", 2 ), // f1
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // f2
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // f3
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // f4
        Instr::new(Box::new(sbc::zeropage_x), "sbc::zeropage_x", 2 ), // f5
        Instr::new(Box::new(inc::zeropage_x), "inc::zeropage_x", 2 ), // f6
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // f7
        Instr::new(Box::new(flags::sed), "sed::implied", 1 ), // f8
        Instr::new(Box::new(sbc::absolute_y), "sbc::absolute_y", 3 ), // f9
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // fa
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // fb
        Instr::new(Box::new(error_fn), "error_fn", 255 ), // fc
        Instr::new(Box::new(sbc::absolute_x), "sbc::absolute_x", 3 ), // fd
        Instr::new(Box::new(inc::absolute_x), "inc::absolute_x", 3 ), // fe
        Instr::new(Box::new(error_fn), "error_fn", 255) /* ff */,
    ])}};
}

pub struct Instr {
    pub fun: Box<dyn Fn(&mut Cpu) -> (u8, u8)>,
    pub fname: String,
    pub ilen: usize,
}

impl Instr {
    fn new<S: Into<String>>(fun: Box<dyn Fn(&mut Cpu) -> (u8, u8)>, fname: S, ilen: usize) -> Self {
        Instr {
            fun,
            fname: fname.into(),
            ilen,
        }
    }
}

pub fn error_fn(_cpu: &mut Cpu) -> (u8, u8) {
    // panic!("Invalid opcode!");
    (0xFF, 0xFF)
}

fn format_hex(data: &[u8]) -> String {
    let hexes: Vec<_> = data.iter().map(|v| format!("{:02X}", v)).collect();
    hexes.join("")
}

fn get_fname_for_print(fname: &str, arg: &str) -> String {
    let pieces: Vec<&str> = fname.split("::").collect();

    let instr_name = pieces.get(0).unwrap();
    let address = pieces.get(1);

    match address {
        Some(&"implied") => instr_name.to_string(),
        Some(&"zeropage_x") => format!("{} {}+x", instr_name, arg),
        Some(&"zeropage") => format!("{} {}", instr_name, arg),
        Some(&"immediate") => format!("{} #{}", instr_name, arg),
        Some(&"absolute_x") => format!("{} [{}+x]", instr_name, arg),
        Some(&"absolute_y") => format!("{} [{}+y]", instr_name, arg),
        Some(&"absolute") => format!("{} [{}]", instr_name, arg),
        Some(&"indirect_x") => format!("{} x({})", instr_name, arg),
        Some(&"indirect_y") => format!("{} y({})", instr_name, arg),
        _ => instr_name.to_string(),
    }
}

pub fn disassemble_instr(prg: &[u8], current: usize) -> (String, usize) {
    let opcode: u8 = prg[current];

    let Instr {
        ref fname,
        mut ilen,
        ..
    } = INSTR_TABLE[opcode as usize];

    let is_error = ilen == 0xFF;

    if ilen == 0 || ilen == 0xFF {
        // branches or error
        ilen = 1;
    }

    let a = if is_error {
        format!("{} ({:02X})", fname, opcode)
    } else {
        let codes = &format_hex(&prg[current + 1..current + ilen]);
        debug!(
            "{:02X}> Found function {}, opcode: {:02X}, {}, bytes: {:?}",
            current + 16,
            fname,
            opcode,
            ilen,
            codes
        );
        get_fname_for_print(&fname, codes)
    };

    (a.to_owned(), current + ilen)
}

// decode functions

#[macro_export]
macro_rules! decode_absolute {
    ( $cpu:expr ) => {{
        let low = $cpu.memory.fetch($cpu.registers.pc + 1);
        let high = $cpu.memory.fetch($cpu.registers.pc + 2);
        (to_u16(low, high), 3)
    }};
}

#[macro_export]
macro_rules! decode_immediate {
    ( $cpu:expr ) => {{
        ($cpu.memory.fetch($cpu.registers.pc + 1), 2)
    }};
}

#[macro_export]
macro_rules! decode_zeropage {
    ( $cpu:expr ) => {{
        ($cpu.memory.fetch($cpu.registers.pc + 1), 2)
    }};
}

#[macro_export]
macro_rules! decode_absolute_indexed {
    ( $cpu:expr, $offset:expr ) => {{
        let low = $cpu.memory.fetch($cpu.registers.pc + 1);
        let high = $cpu.memory.fetch($cpu.registers.pc + 2);
        (to_u16(low, high).wrapping_add($offset as u16), 3)
    }};
}

#[macro_export]
macro_rules! decode_zeropage_indexed {
    ( $cpu:expr, $offset:expr ) => {{
        let addr = $cpu.memory.fetch($cpu.registers.pc + 1);
        (addr.wrapping_add($offset), 2)
    }};
}

#[macro_export]
macro_rules! decode_indexed_indirect {
    ( $cpu:expr ) => {{
        let op = ($cpu
            .memory
            .fetch($cpu.registers.pc + 1)
            .wrapping_add($cpu.registers.x_reg)) as u16
            & 0xFF;
        let low = $cpu.memory.fetch(op);
        let high = $cpu.memory.fetch((op + 1) & 0xFF);

        (to_u16(low, high), 2)
    }};
}

#[macro_export]
macro_rules! decode_indirect_indexed {
    ( $cpu:expr ) => {{
        let op = $cpu.memory.fetch($cpu.registers.pc + 1) as u16;
        let low = $cpu.memory.fetch(op);
        let high = $cpu.memory.fetch((op + 1) & 0xFF);

        (
            to_u16(low, high).wrapping_add($cpu.registers.y_reg as u16),
            2,
        )
    }};
}
