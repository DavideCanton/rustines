use arch::instrs::*;
use arch::cpu::CPU;
use utils::tls::Syncify;

lazy_static! {
pub static ref INSTR_TABLE: Syncify<[Instr<'static>; 256]> = {
    unsafe { Syncify::new ([
        Instr { fun: Box::new(others::brk), fname: "brk::implied", ilen: 0 }, // 00
        Instr { fun: Box::new(ora::indirect_x), fname: "ora::indirect_x", ilen: 2 }, // 01
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 02
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 03
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 04
        Instr { fun: Box::new(ora::zeropage), fname: "ora::zeropage", ilen: 2 }, // 05
        Instr { fun: Box::new(asl::zeropage), fname: "asl::zeropage", ilen: 2 }, // 06
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 07
        Instr { fun: Box::new(pushpop::php), fname: "php::implied", ilen: 1 }, // 08
        Instr { fun: Box::new(ora::immediate), fname: "ora::immediate", ilen: 2 }, // 09
        Instr { fun: Box::new(asl::accumulator), fname: "asl::accumulator", ilen: 1 }, // 0a
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 0b
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 0c
        Instr { fun: Box::new(ora::absolute), fname: "ora::absolute", ilen: 3 }, // 0d
        Instr { fun: Box::new(asl::absolute), fname: "asl::absolute", ilen: 3 }, // 0e
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 0f
        Instr { fun: Box::new(branches::bpl), fname: "bpl", ilen: 2 }, // 10
        Instr { fun: Box::new(ora::indirect_y), fname: "ora::indirect_y", ilen: 2 }, // 11
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 12
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 13
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 14
        Instr { fun: Box::new(ora::zeropage_x), fname: "ora::zeropage_x", ilen: 2 }, // 15
        Instr { fun: Box::new(asl::zeropage_x), fname: "asl::zeropage_x", ilen: 2 }, // 16
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 17
        Instr { fun: Box::new(flags::clc), fname: "clc::implied", ilen: 1 }, // 18
        Instr { fun: Box::new(ora::absolute_y), fname: "ora::absolute_y", ilen: 3 }, // 19
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 1a
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 1b
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 1c
        Instr { fun: Box::new(ora::absolute_x), fname: "ora::absolute_x", ilen: 3 }, // 1d
        Instr { fun: Box::new(asl::absolute_x), fname: "asl::absolute_x", ilen: 3 }, // 1e
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 1f
        Instr { fun: Box::new(subroutines::jsr), fname: "jsr::absolute", ilen: 0 }, // 20
        Instr { fun: Box::new(and::indirect_x), fname: "and::indirect_x", ilen: 2 }, // 21
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 22
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 23
        Instr { fun: Box::new(bit::zeropage), fname: "bit::zeropage", ilen: 2 }, // 24
        Instr { fun: Box::new(and::zeropage), fname: "and::zeropage", ilen: 2 }, // 25
        Instr { fun: Box::new(rol::zeropage), fname: "rol::zeropage", ilen: 2 }, // 26
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 27
        Instr { fun: Box::new(pushpop::plp), fname: "plp::implied", ilen: 1 }, // 28
        Instr { fun: Box::new(and::immediate), fname: "and::immediate", ilen: 2 }, // 29
        Instr { fun: Box::new(rol::accumulator), fname: "rol::accumulator", ilen: 1 }, // 2a
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 2b
        Instr { fun: Box::new(bit::absolute), fname: "bit::absolute", ilen: 3 }, // 2c
        Instr { fun: Box::new(and::absolute), fname: "and::absolute", ilen: 3 }, // 2d
        Instr { fun: Box::new(rol::absolute), fname: "rol::absolue", ilen: 3 }, // 2e
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 2f
        Instr { fun: Box::new(branches::bmi), fname: "bmi", ilen: 0 }, // 30
        Instr { fun: Box::new(and::indirect_y), fname: "and::indirect_y", ilen: 2 }, // 31
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 32
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 33
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 34
        Instr { fun: Box::new(and::zeropage_x), fname: "and::zeropage_x", ilen: 2 }, // 35
        Instr { fun: Box::new(rol::zeropage_x), fname: "rol::zeropage_x", ilen: 2 }, // 36
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 37
        Instr { fun: Box::new(flags::sec), fname: "sec::implied", ilen: 1 }, // 38
        Instr { fun: Box::new(and::absolute_y), fname: "and::absolute_y", ilen: 3 }, // 39
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 3a
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 3b
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 3c
        Instr { fun: Box::new(and::absolute_x), fname: "and::absolute_x", ilen: 3 }, // 3d
        Instr { fun: Box::new(rol::absolute_x), fname: "rol::absolute_x", ilen: 3 }, // 3e
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 3f
        Instr { fun: Box::new(subroutines::rti), fname: "rti::absolute", ilen: 0 }, // 40
        Instr { fun: Box::new(eor::indirect_x), fname: "eor::indirect_x", ilen: 2 }, // 41
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 42
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 43
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 44
        Instr { fun: Box::new(eor::zeropage), fname: "eor::zeropage", ilen: 2 }, // 45
        Instr { fun: Box::new(lsr::zeropage), fname: "lsr::zeropage", ilen: 2 }, // 46
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 47
        Instr { fun: Box::new(pushpop::pha), fname: "pha::implied", ilen: 1 }, // 48
        Instr { fun: Box::new(eor::immediate), fname: "eor::immediate", ilen: 2 }, // 49
        Instr { fun: Box::new(lsr::accumulator), fname: "lsr::accumulator", ilen: 1 }, // 4a
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 4b
        Instr { fun: Box::new(jmp::absolute), fname: "jmp::absolute", ilen: 0 }, // 4c
        Instr { fun: Box::new(eor::absolute), fname: "eor::absolute", ilen: 3 }, // 4d
        Instr { fun: Box::new(lsr::absolute), fname: "lsr::absolute", ilen: 3 }, // 4e
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 4f
        Instr { fun: Box::new(branches::bvc), fname: "bvc", ilen: 2 }, // 50
        Instr { fun: Box::new(eor::indirect_y), fname: "eor::indirect_y", ilen: 2 }, // 51
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 52
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 53
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 54
        Instr { fun: Box::new(eor::zeropage_x), fname: "eor::zeropage_x", ilen: 2 }, // 55
        Instr { fun: Box::new(lsr::zeropage_x), fname: "lsr::zeropage_x", ilen: 2 }, // 56
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 57
        Instr { fun: Box::new(flags::cli), fname: "cli::implied", ilen: 1 }, // 58
        Instr { fun: Box::new(eor::absolute_y), fname: "eor::absolute_y", ilen: 3 }, // 59
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 5a
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 5b
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 5c
        Instr { fun: Box::new(eor::absolute_x), fname: "eor::absolute_x", ilen: 3 }, // 5d
        Instr { fun: Box::new(lsr::absolute_x), fname: "lsr::absolute_x", ilen: 3 }, // 5e
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 5f
        Instr { fun: Box::new(subroutines::rts), fname: "rts::absolute", ilen: 1 }, // 60
        Instr { fun: Box::new(adc::indirect_x), fname: "adc::indirect_x", ilen: 2 }, // 61
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 62
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 63
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 64
        Instr { fun: Box::new(adc::zeropage), fname: "adc::zeropage", ilen: 2 }, // 65
        Instr { fun: Box::new(ror::zeropage), fname: "ror::zeropage", ilen: 2 }, // 66
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 67
        Instr { fun: Box::new(pushpop::pla), fname: "pla::implied", ilen: 1 }, // 68
        Instr { fun: Box::new(adc::immediate), fname: "adc::immediate", ilen: 2 }, // 69
        Instr { fun: Box::new(ror::accumulator), fname: "ror::accumulator", ilen: 1 }, // 6a
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 6b
        Instr { fun: Box::new(jmp::indirect_absolute), fname: "jmp::indirect_absolute", ilen: 0 }, // 6c
        Instr { fun: Box::new(adc::absolute), fname: "adc::absolute", ilen: 3 }, // 6d
        Instr { fun: Box::new(ror::absolute), fname: "ror::absolute", ilen: 3 }, // 6e
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 6f
        Instr { fun: Box::new(branches::bvs), fname: "bvs", ilen: 0 }, // 70
        Instr { fun: Box::new(adc::indirect_y), fname: "adc::indirect_y", ilen: 2 }, // 71
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 72
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 73
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 74
        Instr { fun: Box::new(adc::zeropage_x), fname: "adc::zeropage_x", ilen: 2 }, // 75
        Instr { fun: Box::new(ror::zeropage_x), fname: "ror::zeropage_x", ilen: 2 }, // 76
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 77
        Instr { fun: Box::new(flags::sei), fname: "sei::implied", ilen: 1 }, // 78
        Instr { fun: Box::new(adc::absolute_y), fname: "adc::absolute_y", ilen: 3 }, // 79
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 7a
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 7b
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 7c
        Instr { fun: Box::new(adc::absolute_x), fname: "adc::absolute_x", ilen: 3 }, // 7d
        Instr { fun: Box::new(ror::absolute_x), fname: "ror::absolute_x", ilen: 3 }, // 7e
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 7f
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 80
        Instr { fun: Box::new(sta::indirect_x), fname: "sta::indirect_x", ilen: 2 }, // 81
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 82
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 83
        Instr { fun: Box::new(sty::zeropage), fname: "sty::zeropage", ilen: 2 }, // 84
        Instr { fun: Box::new(sta::zeropage), fname: "sta::zeropage", ilen: 2 }, // 85
        Instr { fun: Box::new(stx::zeropage), fname: "stx::zeropage", ilen: 2 }, // 86
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 87
        Instr { fun: Box::new(dey::implied), fname: "dey::implied", ilen: 1 }, // 88
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 89
        Instr { fun: Box::new(transfers::txa), fname: "txa::implied", ilen: 1 }, // 8a
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 8b
        Instr { fun: Box::new(sty::absolute), fname: "sty::absolute", ilen: 3 }, // 8c
        Instr { fun: Box::new(sta::absolute), fname: "sta::absolute", ilen: 3 }, // 8d
        Instr { fun: Box::new(stx::absolute), fname: "stx::absolute", ilen: 3 }, // 8e
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 8f
        Instr { fun: Box::new(branches::bcc), fname: "bcc", ilen: 0 }, // 90
        Instr { fun: Box::new(sta::indirect_y), fname: "sta::indirect_y", ilen: 2 }, // 91
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 92
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 93
        Instr { fun: Box::new(sty::zeropage_x), fname: "sty::zeropage_x", ilen: 2 }, // 94
        Instr { fun: Box::new(sta::zeropage_x), fname: "sta::zeropage_x", ilen: 2 }, // 95
        Instr { fun: Box::new(stx::zeropage_y), fname: "stx::zeropage_y", ilen: 2 }, // 96
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 97
        Instr { fun: Box::new(transfers::tya), fname: "tya::implied", ilen: 1 }, // 98
        Instr { fun: Box::new(sta::absolute_y), fname: "sta::absolute_y", ilen: 3 }, // 99
        Instr { fun: Box::new(transfers::txs), fname: "txs::implied", ilen: 1 }, // 9a
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 9b
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 9c
        Instr { fun: Box::new(sta::absolute_x), fname: "sta::absolute_x", ilen: 3 }, // 9d
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 9e
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // 9f
        Instr { fun: Box::new(ldy::immediate), fname: "ldy::immediate", ilen: 2 }, // a0
        Instr { fun: Box::new(lda::indirect_x), fname: "lda::indirect_x", ilen: 2 }, // a1
        Instr { fun: Box::new(ldx::immediate), fname: "ldx::immediate", ilen: 2 }, // a2
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // a3
        Instr { fun: Box::new(ldy::zeropage), fname: "ldy::zeropage", ilen: 2 }, // a4
        Instr { fun: Box::new(lda::zeropage), fname: "lda::zeropage", ilen: 2 }, // a5
        Instr { fun: Box::new(ldx::zeropage), fname: "ldx::zeropage", ilen: 2 }, // a6
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // a7
        Instr { fun: Box::new(transfers::tay), fname: "tay::implied", ilen: 1 }, // a8
        Instr { fun: Box::new(lda::immediate), fname: "lda::immediate", ilen: 2 }, // a9
        Instr { fun: Box::new(transfers::tax), fname: "tax::implied", ilen: 1 }, // aa
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // ab
        Instr { fun: Box::new(ldy::absolute), fname: "ldy::absolute", ilen: 3 }, // ac
        Instr { fun: Box::new(lda::absolute), fname: "lda::absolute", ilen: 3 }, // ad
        Instr { fun: Box::new(ldx::absolute), fname: "ldx::absolute", ilen: 3 }, // ae
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // af
        Instr { fun: Box::new(branches::bcs), fname: "bcs", ilen: 2 }, // b0
        Instr { fun: Box::new(lda::indirect_y), fname: "lda::indirect_y", ilen: 2 }, // b1
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // b2
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // b3
        Instr { fun: Box::new(ldy::zeropage_x), fname: "ldy::zeropage_x", ilen: 2 }, // b4
        Instr { fun: Box::new(lda::zeropage_x), fname: "lda::zeropage_x", ilen: 2 }, // b5
        Instr { fun: Box::new(ldx::zeropage_y), fname: "ldx::zeropage_y", ilen: 2 }, // b6
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // b7
        Instr { fun: Box::new(flags::clv), fname: "clv::implied", ilen: 1 }, // b8
        Instr { fun: Box::new(lda::absolute_y), fname: "lda::absolute_y", ilen: 3 }, // b9
        Instr { fun: Box::new(transfers::tsx), fname: "tsx::implied", ilen: 1 }, // ba
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // bb
        Instr { fun: Box::new(ldy::absolute_x), fname: "ldy::absolute_x", ilen: 3 }, // bc
        Instr { fun: Box::new(lda::absolute_x), fname: "lda::absolute_x", ilen: 3 }, // bd
        Instr { fun: Box::new(ldx::absolute_y), fname: "ldx::absolute_y", ilen: 3 }, // be
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // bf
        Instr { fun: Box::new(cpy::immediate), fname: "cpy::immediate", ilen: 2 }, // c0
        Instr { fun: Box::new(cmp::indirect_x), fname: "cmp::indirect_x", ilen: 2 }, // c1
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // c2
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // c3
        Instr { fun: Box::new(cpy::zeropage), fname: "cpy::zeropage", ilen: 2 }, // c4
        Instr { fun: Box::new(cmp::zeropage), fname: "cmp::zeropage", ilen: 2 }, // c5
        Instr { fun: Box::new(dec::zeropage), fname: "dec::zeropage", ilen: 2 }, // c6
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // c7
        Instr { fun: Box::new(iny::implied), fname: "iny::implied", ilen: 1 }, // c8
        Instr { fun: Box::new(cmp::immediate), fname: "cmp::immediate", ilen: 2 }, // c9
        Instr { fun: Box::new(dex::implied), fname: "dex::implied", ilen: 1 }, // ca
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // cb
        Instr { fun: Box::new(cpy::absolute), fname: "cpy::absolute", ilen: 3 }, // cc
        Instr { fun: Box::new(cmp::absolute), fname: "cmp::absolute", ilen: 3 }, // cd
        Instr { fun: Box::new(dec::absolute), fname: "dec::absolute", ilen: 3 }, // ce
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // cf
        Instr { fun: Box::new(branches::bne), fname: "bne", ilen: 2 }, // d0
        Instr { fun: Box::new(cmp::indirect_y), fname: "cmp::indirect_y", ilen: 2 }, // d1
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // d2
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // d3
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // d4
        Instr { fun: Box::new(cmp::zeropage_x), fname: "cmp::zeropage_x", ilen: 2 }, // d5
        Instr { fun: Box::new(dec::zeropage_x), fname: "dec::zeropage_x", ilen: 2 }, // d6
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // d7
        Instr { fun: Box::new(flags::cld), fname: "cld::implied", ilen: 1 }, // d8
        Instr { fun: Box::new(cmp::absolute_y), fname: "cmp::absolute_y", ilen: 3 }, // d9
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // da
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // db
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // dc
        Instr { fun: Box::new(cmp::absolute_x), fname: "cmp::absolute_x", ilen: 3 }, // dd
        Instr { fun: Box::new(dec::absolute_x), fname: "dec::absolute_x", ilen: 3 }, // de
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // df
        Instr { fun: Box::new(cpx::immediate), fname: "cpx::immediate", ilen: 2 }, // e0
        Instr { fun: Box::new(sbc::indirect_x), fname: "sbc::indirect_x", ilen: 2 }, // e1
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // e2
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // e3
        Instr { fun: Box::new(cpx::zeropage), fname: "cpx::zeropage", ilen: 2 }, // e4
        Instr { fun: Box::new(sbc::zeropage), fname: "sbc::zeropage", ilen: 2 }, // e5
        Instr { fun: Box::new(inc::zeropage), fname: "inc::zeropage", ilen: 2 }, // e6
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // e7
        Instr { fun: Box::new(inx::implied), fname: "inx::implied", ilen: 1 }, // e8
        Instr { fun: Box::new(sbc::immediate), fname: "sbc::immediate", ilen: 2 }, // e9
        Instr { fun: Box::new(others::nop), fname: "nop::implied", ilen: 1 }, // ea
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // eb
        Instr { fun: Box::new(cpx::absolute), fname: "cpx::absolute", ilen: 3 }, // ec
        Instr { fun: Box::new(sbc::absolute), fname: "sbc::absolute", ilen: 3 }, // ed
        Instr { fun: Box::new(inc::absolute), fname: "inc::absolute", ilen: 3 }, // ee
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // ef
        Instr { fun: Box::new(branches::beq), fname: "beq", ilen: 0 }, // f0
        Instr { fun: Box::new(sbc::indirect_y), fname: "sbc::indirect_y", ilen: 2 }, // f1
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // f2
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // f3
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // f4
        Instr { fun: Box::new(sbc::zeropage_x), fname: "sbc::zeropage_x", ilen: 2 }, // f5
        Instr { fun: Box::new(inc::zeropage_x), fname: "inc::zeropage_x", ilen: 2 }, // f6
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // f7
        Instr { fun: Box::new(flags::sed), fname: "sed::implied", ilen: 1 }, // f8
        Instr { fun: Box::new(sbc::absolute_y), fname: "sbc::absolute_y", ilen: 3 }, // f9
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // fa
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // fb
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 }, // fc
        Instr { fun: Box::new(sbc::absolute_x), fname: "sbc::absolute_x", ilen: 3 }, // fd
        Instr { fun: Box::new(inc::absolute_x), fname: "inc::absolute_x", ilen: 3 }, // fe
        Instr { fun: Box::new(error_fn), fname: "error_fn", ilen: 255 } /* ff */,
    ])}};
}

pub struct Instr<'a> {
    pub fun: Box<Fn(&mut CPU) -> (u8, u8)>,
    pub fname: &'a str,
    pub ilen: usize,
}

pub fn error_fn(_cpu: &mut CPU) -> (u8, u8) {
    // panic!("Invalid opcode!");
    (0xFF, 0xFF)
}

fn format_hex(data: &[u8]) -> String {
    let hexes: Vec<_> = data.iter().map(|v| format!("{:02X}", v)).collect();
    hexes.join("")
}

fn get_fname_for_print(fname: &str, arg: &str) -> String {
    let instr_name = fname.split("::").nth(0).unwrap();

    let fname_for_print = if fname.contains("implied") {
        instr_name.to_string()
    } else if fname.contains("zeropage_x") {
        format!("{} {}+x", instr_name, arg)
    } else if fname.contains("zeropage") {
        format!("{} {}", instr_name, arg)
    } else if fname.contains("immediate") {
        format!("{} #{}", instr_name, arg)
    } else if fname.contains("absolute_x") {
        format!("{} [{}+x]", instr_name, arg)
    } else if fname.contains("absolute_y") {
        format!("{} [{}+y]", instr_name, arg)
    } else if fname.contains("absolute") {
        format!("{} [{}]", instr_name, arg)
    } else if fname.contains("indirect_x") {
        format!("{} x({})", instr_name, arg)
    } else if fname.contains("indirect_y") {
        format!("{} y({})", instr_name, arg)
    } else {
        instr_name.to_string()
    };

  fname_for_print
}

pub fn disassemble_instr(prg: &[u8], current: usize) -> (String, usize) {
    let opcode: u8 = prg[current];

    let Instr { fname, mut ilen, .. } = INSTR_TABLE[opcode as usize];    
    let is_error = ilen == 0xFF;

    if ilen == 0 || ilen == 0xFF {
        // branches or error
        ilen = 1;
    }

    let a = if is_error {
        format!("{} ({:02X})", fname, opcode)
    } else {
        let codes = &format_hex(&prg[current + 1..current + ilen]);
        debug!("{:02X}> Found function {}, opcode: {:02X}, ilen: {}, bytes: {:?}", current+16, fname, opcode, ilen, codes);
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
        let op = ($cpu.memory.fetch($cpu.registers.pc + 1).wrapping_add($cpu.registers.x_reg)) as u16 & 0xFF;
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

        (to_u16(low, high).wrapping_add($cpu.registers.y_reg as u16), 2)
    }};
}
