use crate::arch::cpu::Cpu;
use crate::arch::instrs::*;
use crate::hex;

macro_rules! instr {
    ( $fn: expr, $name: expr, $op: expr ) => {{ Instr::new($fn, $name, $op) }};
    ( $fn: expr, $op: expr ) => {{ Instr::new($fn, stringify!($fn), $op) }};
}

pub const INSTR_TABLE: [Instr; 256] = [
    instr!(others::brk, "brk::implied", 0),       // 00
    instr!(ora::indirect_x, 2),                   // 01
    Instr::error(),                               // 02
    Instr::error(),                               // 03
    Instr::error(),                               // 04
    instr!(ora::zeropage, 2),                     // 05
    instr!(asl::zeropage, 2),                     // 06
    Instr::error(),                               // 07
    instr!(pushpop::php, "php::implied", 1),      // 08
    instr!(ora::immediate, 2),                    // 09
    instr!(asl::accumulator, 1),                  // 0a
    Instr::error(),                               // 0b
    Instr::error(),                               // 0c
    instr!(ora::absolute, 3),                     // 0d
    instr!(asl::absolute, 3),                     // 0e
    Instr::error(),                               // 0f
    instr!(branches::bpl, "bpl", 2),              // 10
    instr!(ora::indirect_y, 2),                   // 11
    Instr::error(),                               // 12
    Instr::error(),                               // 13
    Instr::error(),                               // 14
    instr!(ora::zeropage_x, 2),                   // 15
    instr!(asl::zeropage_x, 2),                   // 16
    Instr::error(),                               // 17
    instr!(flags::clc, "clc::implied", 1),        // 18
    instr!(ora::absolute_y, 3),                   // 19
    Instr::error(),                               // 1a
    Instr::error(),                               // 1b
    Instr::error(),                               // 1c
    instr!(ora::absolute_x, 3),                   // 1d
    instr!(asl::absolute_x, 3),                   // 1e
    Instr::error(),                               // 1f
    instr!(subroutines::jsr, "jsr::absolute", 3), // 20
    instr!(and::indirect_x, 2),                   // 21
    Instr::error(),                               // 22
    Instr::error(),                               // 23
    instr!(bit::zeropage, 2),                     // 24
    instr!(and::zeropage, 2),                     // 25
    instr!(rol::zeropage, 2),                     // 26
    Instr::error(),                               // 27
    instr!(pushpop::plp, "plp::implied", 1),      // 28
    instr!(and::immediate, 2),                    // 29
    instr!(rol::accumulator, 1),                  // 2a
    Instr::error(),                               // 2b
    instr!(bit::absolute, 3),                     // 2c
    instr!(and::absolute, 3),                     // 2d
    instr!(rol::absolute, 3),                     // 2e
    Instr::error(),                               // 2f
    instr!(branches::bmi, "bmi", 2),              // 30
    instr!(and::indirect_y, 2),                   // 31
    Instr::error(),                               // 32
    Instr::error(),                               // 33
    Instr::error(),                               // 34
    instr!(and::zeropage_x, 2),                   // 35
    instr!(rol::zeropage_x, 2),                   // 36
    Instr::error(),                               // 37
    instr!(flags::sec, "sec::implied", 1),        // 38
    instr!(and::absolute_y, 3),                   // 39
    Instr::error(),                               // 3a
    Instr::error(),                               // 3b
    Instr::error(),                               // 3c
    instr!(and::absolute_x, 3),                   // 3d
    instr!(rol::absolute_x, 3),                   // 3e
    Instr::error(),                               // 3f
    instr!(subroutines::rti, "rti::absolute", 1), // 40
    instr!(eor::indirect_x, 2),                   // 41
    Instr::error(),                               // 42
    Instr::error(),                               // 43
    Instr::error(),                               // 44
    instr!(eor::zeropage, 2),                     // 45
    instr!(lsr::zeropage, 2),                     // 46
    Instr::error(),                               // 47
    instr!(pushpop::pha, "pha::implied", 1),      // 48
    instr!(eor::immediate, 2),                    // 49
    instr!(lsr::accumulator, 1),                  // 4a
    Instr::error(),                               // 4b
    instr!(jmp::absolute, 3),                     // 4c
    instr!(eor::absolute, 3),                     // 4d
    instr!(lsr::absolute, 3),                     // 4e
    Instr::error(),                               // 4f
    instr!(branches::bvc, "bvc", 2),              // 50
    instr!(eor::indirect_y, 2),                   // 51
    Instr::error(),                               // 52
    Instr::error(),                               // 53
    Instr::error(),                               // 54
    instr!(eor::zeropage_x, 2),                   // 55
    instr!(lsr::zeropage_x, 2),                   // 56
    Instr::error(),                               // 57
    instr!(flags::cli, "cli::implied", 1),        // 58
    instr!(eor::absolute_y, 3),                   // 59
    Instr::error(),                               // 5a
    Instr::error(),                               // 5b
    Instr::error(),                               // 5c
    instr!(eor::absolute_x, 3),                   // 5d
    instr!(lsr::absolute_x, 3),                   // 5e
    Instr::error(),                               // 5f
    instr!(subroutines::rts, "rts::absolute", 1), // 60
    instr!(adc::indirect_x, 2),                   // 61
    Instr::error(),                               // 62
    Instr::error(),                               // 63
    Instr::error(),                               // 64
    instr!(adc::zeropage, 2),                     // 65
    instr!(ror::zeropage, 2),                     // 66
    Instr::error(),                               // 67
    instr!(pushpop::pla, "pla::implied", 1),      // 68
    instr!(adc::immediate, 2),                    // 69
    instr!(ror::accumulator, 1),                  // 6a
    Instr::error(),                               // 6b
    instr!(jmp::indirect_absolute, 3),            // 6c
    instr!(adc::absolute, 3),                     // 6d
    instr!(ror::absolute, 3),                     // 6e
    Instr::error(),                               // 6f
    instr!(branches::bvs, "bvs", 2),              // 70
    instr!(adc::indirect_y, 2),                   // 71
    Instr::error(),                               // 72
    Instr::error(),                               // 73
    Instr::error(),                               // 74
    instr!(adc::zeropage_x, 2),                   // 75
    instr!(ror::zeropage_x, 2),                   // 76
    Instr::error(),                               // 77
    instr!(flags::sei, "sei::implied", 1),        // 78
    instr!(adc::absolute_y, 3),                   // 79
    Instr::error(),                               // 7a
    Instr::error(),                               // 7b
    Instr::error(),                               // 7c
    instr!(adc::absolute_x, 3),                   // 7d
    instr!(ror::absolute_x, 3),                   // 7e
    Instr::error(),                               // 7f
    Instr::error(),                               // 80
    instr!(sta::indirect_x, 2),                   // 81
    Instr::error(),                               // 82
    Instr::error(),                               // 83
    instr!(sty::zeropage, 2),                     // 84
    instr!(sta::zeropage, 2),                     // 85
    instr!(stx::zeropage, 2),                     // 86
    Instr::error(),                               // 87
    instr!(dey::implied, 1),                      // 88
    Instr::error(),                               // 89
    instr!(transfers::txa, "txa::implied", 1),    // 8a
    Instr::error(),                               // 8b
    instr!(sty::absolute, 3),                     // 8c
    instr!(sta::absolute, 3),                     // 8d
    instr!(stx::absolute, 3),                     // 8e
    Instr::error(),                               // 8f
    instr!(branches::bcc, "bcc", 2),              // 90
    instr!(sta::indirect_y, 2),                   // 91
    Instr::error(),                               // 92
    Instr::error(),                               // 93
    instr!(sty::zeropage_x, 2),                   // 94
    instr!(sta::zeropage_x, 2),                   // 95
    instr!(stx::zeropage_y, 2),                   // 96
    Instr::error(),                               // 97
    instr!(transfers::tya, "tya::implied", 1),    // 98
    instr!(sta::absolute_y, 3),                   // 99
    instr!(transfers::txs, "txs::implied", 1),    // 9a
    Instr::error(),                               // 9b
    Instr::error(),                               // 9c
    instr!(sta::absolute_x, 3),                   // 9d
    Instr::error(),                               // 9e
    Instr::error(),                               // 9f
    instr!(ldy::immediate, 2),                    // a0
    instr!(lda::indirect_x, 2),                   // a1
    instr!(ldx::immediate, 2),                    // a2
    Instr::error(),                               // a3
    instr!(ldy::zeropage, 2),                     // a4
    instr!(lda::zeropage, 2),                     // a5
    instr!(ldx::zeropage, 2),                     // a6
    Instr::error(),                               // a7
    instr!(transfers::tay, "tay::implied", 1),    // a8
    instr!(lda::immediate, 2),                    // a9
    instr!(transfers::tax, "tax::implied", 1),    // aa
    Instr::error(),                               // ab
    instr!(ldy::absolute, 3),                     // ac
    instr!(lda::absolute, 3),                     // ad
    instr!(ldx::absolute, 3),                     // ae
    Instr::error(),                               // af
    instr!(branches::bcs, "bcs", 2),              // b0
    instr!(lda::indirect_y, 2),                   // b1
    Instr::error(),                               // b2
    Instr::error(),                               // b3
    instr!(ldy::zeropage_x, 2),                   // b4
    instr!(lda::zeropage_x, 2),                   // b5
    instr!(ldx::zeropage_y, 2),                   // b6
    Instr::error(),                               // b7
    instr!(flags::clv, "clv::implied", 1),        // b8
    instr!(lda::absolute_y, 3),                   // b9
    instr!(transfers::tsx, "tsx::implied", 1),    // ba
    Instr::error(),                               // bb
    instr!(ldy::absolute_x, 3),                   // bc
    instr!(lda::absolute_x, 3),                   // bd
    instr!(ldx::absolute_y, 3),                   // be
    Instr::error(),                               // bf
    instr!(cpy::immediate, 2),                    // c0
    instr!(cmp::indirect_x, 2),                   // c1
    Instr::error(),                               // c2
    Instr::error(),                               // c3
    instr!(cpy::zeropage, 2),                     // c4
    instr!(cmp::zeropage, 2),                     // c5
    instr!(dec::zeropage, 2),                     // c6
    Instr::error(),                               // c7
    instr!(iny::implied, 1),                      // c8
    instr!(cmp::immediate, 2),                    // c9
    instr!(dex::implied, 1),                      // ca
    Instr::error(),                               // cb
    instr!(cpy::absolute, 3),                     // cc
    instr!(cmp::absolute, 3),                     // cd
    instr!(dec::absolute, 3),                     // ce
    Instr::error(),                               // cf
    instr!(branches::bne, "bne", 2),              // d0
    instr!(cmp::indirect_y, 2),                   // d1
    Instr::error(),                               // d2
    Instr::error(),                               // d3
    Instr::error(),                               // d4
    instr!(cmp::zeropage_x, 2),                   // d5
    instr!(dec::zeropage_x, 2),                   // d6
    Instr::error(),                               // d7
    instr!(flags::cld, "cld::implied", 1),        // d8
    instr!(cmp::absolute_y, 3),                   // d9
    Instr::error(),                               // da
    Instr::error(),                               // db
    Instr::error(),                               // dc
    instr!(cmp::absolute_x, 3),                   // dd
    instr!(dec::absolute_x, 3),                   // de
    Instr::error(),                               // df
    instr!(cpx::immediate, 2),                    // e0
    instr!(sbc::indirect_x, 2),                   // e1
    Instr::error(),                               // e2
    Instr::error(),                               // e3
    instr!(cpx::zeropage, 2),                     // e4
    instr!(sbc::zeropage, 2),                     // e5
    instr!(inc::zeropage, 2),                     // e6
    Instr::error(),                               // e7
    instr!(inx::implied, 1),                      // e8
    instr!(sbc::immediate, 2),                    // e9
    instr!(others::nop, "nop::implied", 1),       // ea
    Instr::error(),                               // eb
    instr!(cpx::absolute, 3),                     // ec
    instr!(sbc::absolute, 3),                     // ed
    instr!(inc::absolute, 3),                     // ee
    Instr::error(),                               // ef
    instr!(branches::beq, "beq", 2),              // f0
    instr!(sbc::indirect_y, 2),                   // f1
    Instr::error(),                               // f2
    Instr::error(),                               // f3
    Instr::error(),                               // f4
    instr!(sbc::zeropage_x, 2),                   // f5
    instr!(inc::zeropage_x, 2),                   // f6
    Instr::error(),                               // f7
    instr!(flags::sed, "sed::implied", 1),        // f8
    instr!(sbc::absolute_y, 3),                   // f9
    Instr::error(),                               // fa
    Instr::error(),                               // fb
    Instr::error(),                               // fc
    instr!(sbc::absolute_x, 3),                   // fd
    instr!(inc::absolute_x, 3),                   // fe
    instr!(error_fn, 255),                        // ff
];

pub type InstrFn = fn(&mut Cpu) -> (u8, u8);

pub struct Instr {
    pub fun: InstrFn,
    pub fname: &'static str,
    pub ilen: usize,
}

impl Instr {
    const fn new(fun: InstrFn, fname: &'static str, ilen: usize) -> Self {
        Instr { fun, fname, ilen }
    }

    const fn error() -> Self {
        Instr::new(error_fn, "err", 255)
    }

    pub fn get_fname_for_print(&self, data: &[u8]) -> String {
        let codes = data
            .iter()
            .skip(1)
            .map(|v| hex!(v))
            .collect::<Vec<_>>()
            .join(" ");

        let pieces: Vec<&str> = self.fname.split("::").collect();

        let instr_name = pieces.first().unwrap();
        let address = pieces.get(1);

        let ret = match address {
            Some(&"implied") => instr_name.to_string(),
            Some(&"zeropage_x") => format!("{} {}+x", instr_name, codes),
            Some(&"zeropage") => format!("{} {}", instr_name, codes),
            Some(&"immediate") => format!("{} #{}", instr_name, codes),
            Some(&"absolute_x") => format!("{} [{}+x]", instr_name, codes),
            Some(&"absolute_y") => format!("{} [{}+y]", instr_name, codes),
            Some(&"absolute") => format!("{} [{}]", instr_name, codes),
            Some(&"indirect_x") => format!("{} x({})", instr_name, codes),
            Some(&"indirect_y") => format!("{} y({})", instr_name, codes),
            _ => match self.ilen {
                1 => instr_name.to_string(),
                _ => format!("{} {}", instr_name, codes),
            },
        };
        format!("({}) {}", hex!(data[0]), ret)
    }
}

pub fn error_fn(_cpu: &mut Cpu) -> (u8, u8) {
    // panic!("Invalid opcode!");
    (0xFF, 0xFF)
}

pub fn disassemble_instr(prg: &[u8], current: usize) -> (String, usize) {
    let opcode: u8 = prg[current];

    let instr = &INSTR_TABLE[opcode as usize];
    let Instr { fname, ilen, .. } = instr;

    let mut ilen = *ilen;
    let is_error = ilen == 0xFF;

    if is_error {
        ilen = 1;
    }

    let msg = if is_error {
        format!("{} ({})", fname, hex!(opcode))
    } else {
        instr.get_fname_for_print(&prg[current..current + ilen])
    };

    (msg, current + ilen)
}

#[cfg(test)]
#[macro_use]
mod test {
    use crate::arch::cpu::Cpu;

    use super::{Instr, InstrFn};

    fn instr_fn1(_cpu: &mut Cpu) -> (u8, u8) {
        (0, 0)
    }
    fn instr_fn2(_cpu: &mut Cpu) -> (u8, u8) {
        (0, 0)
    }

    mod test_macro {
        use super::Instr;
        use super::{compare_instr_fun, instr_fn1, instr_fn2};

        #[test]
        fn test_instr_macro_2() {
            let instr1 = instr!(instr_fn1, 2);
            assert_eq!(instr1.fname, "instr_fn1");
            compare_instr_fun(&instr1, instr_fn1);
            assert_eq!(instr1.ilen, 2);
        }

        #[test]
        fn test_instr_macro_3() {
            let instr2 = instr!(instr_fn2, "fn2", 2);
            assert_eq!(instr2.fname, "fn2");
            compare_instr_fun(&instr2, instr_fn2);
            assert_eq!(instr2.ilen, 2);
        }
    }

    mod test_instr {
        use super::super::error_fn;
        use super::Instr;
        use super::{compare_instr_fun, instr_fn1};

        #[test]
        fn test_new() {
            let instr = Instr::new(instr_fn1, "foo", 42);
            assert_eq!(instr.fname, "foo");
            assert_eq!(instr.ilen, 42);
            compare_instr_fun(&instr, instr_fn1);
        }

        #[test]
        fn test_error() {
            let instr = Instr::error();
            assert_eq!(instr.fname, "err");
            assert_eq!(instr.ilen, 255);
            compare_instr_fun(&instr, error_fn);
        }
    }

    fn compare_instr_fun(instr: &Instr, exp: InstrFn) {
        assert_eq!(instr.fun as *const (), exp as *const ());
    }
}
