use crate::arch::cpu::Cpu;
use crate::arch::instrs::*;
use crate::hex;
use crate::utils::tls::Syncify;
use lazy_static::*;

macro_rules! instr {
    ( $fn: expr, $name: expr, $op: expr ) => {{
        Instr::new(Box::new($fn), $name, $op)
    }};
}

macro_rules! err {
    () => {{
        instr!(error_fn, "error_fn", 255)
    }};
}

lazy_static! {
pub static ref INSTR_TABLE: Syncify<[Instr; 256]> = {
    unsafe { Syncify::new ([
        instr!(others::brk, "brk::implied", 0 ), // 00
        instr!(ora::indirect_x, "ora::indirect_x", 2 ), // 01
        err!(), // 02
        err!(), // 03
        err!(), // 04
        instr!(ora::zeropage, "ora::zeropage", 2 ), // 05
        instr!(asl::zeropage, "asl::zeropage", 2 ), // 06
        err!(), // 07
        instr!(pushpop::php, "php::implied", 1 ), // 08
        instr!(ora::immediate, "ora::immediate", 2 ), // 09
        instr!(asl::accumulator, "asl::accumulator", 1 ), // 0a
        err!(), // 0b
        err!(), // 0c
        instr!(ora::absolute, "ora::absolute", 3 ), // 0d
        instr!(asl::absolute, "asl::absolute", 3 ), // 0e
        err!(), // 0f
        instr!(branches::bpl, "bpl", 2 ), // 10
        instr!(ora::indirect_y, "ora::indirect_y", 2 ), // 11
        err!(), // 12
        err!(), // 13
        err!(), // 14
        instr!(ora::zeropage_x, "ora::zeropage_x", 2 ), // 15
        instr!(asl::zeropage_x, "asl::zeropage_x", 2 ), // 16
        err!(), // 17
        instr!(flags::clc, "clc::implied", 1 ), // 18
        instr!(ora::absolute_y, "ora::absolute_y", 3 ), // 19
        err!(), // 1a
        err!(), // 1b
        err!(), // 1c
        instr!(ora::absolute_x, "ora::absolute_x", 3 ), // 1d
        instr!(asl::absolute_x, "asl::absolute_x", 3 ), // 1e
        err!(), // 1f
        instr!(subroutines::jsr, "jsr::absolute", 3 ), // 20
        instr!(and::indirect_x, "and::indirect_x", 2 ), // 21
        err!(), // 22
        err!(), // 23
        instr!(bit::zeropage, "bit::zeropage", 2 ), // 24
        instr!(and::zeropage, "and::zeropage", 2 ), // 25
        instr!(rol::zeropage, "rol::zeropage", 2 ), // 26
        err!(), // 27
        instr!(pushpop::plp, "plp::implied", 1 ), // 28
        instr!(and::immediate, "and::immediate", 2 ), // 29
        instr!(rol::accumulator, "rol::accumulator", 1 ), // 2a
        err!(), // 2b
        instr!(bit::absolute, "bit::absolute", 3 ), // 2c
        instr!(and::absolute, "and::absolute", 3 ), // 2d
        instr!(rol::absolute, "rol::absolute", 3 ), // 2e
        err!(), // 2f
        instr!(branches::bmi, "bmi", 2 ), // 30
        instr!(and::indirect_y, "and::indirect_y", 2 ), // 31
        err!(), // 32
        err!(), // 33
        err!(), // 34
        instr!(and::zeropage_x, "and::zeropage_x", 2 ), // 35
        instr!(rol::zeropage_x, "rol::zeropage_x", 2 ), // 36
        err!(), // 37
        instr!(flags::sec, "sec::implied", 1 ), // 38
        instr!(and::absolute_y, "and::absolute_y", 3 ), // 39
        err!(), // 3a
        err!(), // 3b
        err!(), // 3c
        instr!(and::absolute_x, "and::absolute_x", 3 ), // 3d
        instr!(rol::absolute_x, "rol::absolute_x", 3 ), // 3e
        err!(), // 3f
        instr!(subroutines::rti, "rti::absolute", 1 ), // 40
        instr!(eor::indirect_x, "eor::indirect_x", 2 ), // 41
        err!(), // 42
        err!(), // 43
        err!(), // 44
        instr!(eor::zeropage, "eor::zeropage", 2 ), // 45
        instr!(lsr::zeropage, "lsr::zeropage", 2 ), // 46
        err!(), // 47
        instr!(pushpop::pha, "pha::implied", 1 ), // 48
        instr!(eor::immediate, "eor::immediate", 2 ), // 49
        instr!(lsr::accumulator, "lsr::accumulator", 1 ), // 4a
        err!(), // 4b
        instr!(jmp::absolute, "jmp::absolute", 3 ), // 4c
        instr!(eor::absolute, "eor::absolute", 3 ), // 4d
        instr!(lsr::absolute, "lsr::absolute", 3 ), // 4e
        err!(), // 4f
        instr!(branches::bvc, "bvc", 2 ), // 50
        instr!(eor::indirect_y, "eor::indirect_y", 2 ), // 51
        err!(), // 52
        err!(), // 53
        err!(), // 54
        instr!(eor::zeropage_x, "eor::zeropage_x", 2 ), // 55
        instr!(lsr::zeropage_x, "lsr::zeropage_x", 2 ), // 56
        err!(), // 57
        instr!(flags::cli, "cli::implied", 1 ), // 58
        instr!(eor::absolute_y, "eor::absolute_y", 3 ), // 59
        err!(), // 5a
        err!(), // 5b
        err!(), // 5c
        instr!(eor::absolute_x, "eor::absolute_x", 3 ), // 5d
        instr!(lsr::absolute_x, "lsr::absolute_x", 3 ), // 5e
        err!(), // 5f
        instr!(subroutines::rts, "rts::absolute", 1 ), // 60
        instr!(adc::indirect_x, "adc::indirect_x", 2 ), // 61
        err!(), // 62
        err!(), // 63
        err!(), // 64
        instr!(adc::zeropage, "adc::zeropage", 2 ), // 65
        instr!(ror::zeropage, "ror::zeropage", 2 ), // 66
        err!(), // 67
        instr!(pushpop::pla, "pla::implied", 1 ), // 68
        instr!(adc::immediate, "adc::immediate", 2 ), // 69
        instr!(ror::accumulator, "ror::accumulator", 1 ), // 6a
        err!(), // 6b
        instr!(jmp::indirect_absolute, "jmp::indirect_absolute", 3 ), // 6c
        instr!(adc::absolute, "adc::absolute", 3 ), // 6d
        instr!(ror::absolute, "ror::absolute", 3 ), // 6e
        err!(), // 6f
        instr!(branches::bvs, "bvs", 2 ), // 70
        instr!(adc::indirect_y, "adc::indirect_y", 2 ), // 71
        err!(), // 72
        err!(), // 73
        err!(), // 74
        instr!(adc::zeropage_x, "adc::zeropage_x", 2 ), // 75
        instr!(ror::zeropage_x, "ror::zeropage_x", 2 ), // 76
        err!(), // 77
        instr!(flags::sei, "sei::implied", 1 ), // 78
        instr!(adc::absolute_y, "adc::absolute_y", 3 ), // 79
        err!(), // 7a
        err!(), // 7b
        err!(), // 7c
        instr!(adc::absolute_x, "adc::absolute_x", 3 ), // 7d
        instr!(ror::absolute_x, "ror::absolute_x", 3 ), // 7e
        err!(), // 7f
        err!(), // 80
        instr!(sta::indirect_x, "sta::indirect_x", 2 ), // 81
        err!(), // 82
        err!(), // 83
        instr!(sty::zeropage, "sty::zeropage", 2 ), // 84
        instr!(sta::zeropage, "sta::zeropage", 2 ), // 85
        instr!(stx::zeropage, "stx::zeropage", 2 ), // 86
        err!(), // 87
        instr!(dey::implied, "dey::implied", 1 ), // 88
        err!(), // 89
        instr!(transfers::txa, "txa::implied", 1 ), // 8a
        err!(), // 8b
        instr!(sty::absolute, "sty::absolute", 3 ), // 8c
        instr!(sta::absolute, "sta::absolute", 3 ), // 8d
        instr!(stx::absolute, "stx::absolute", 3 ), // 8e
        err!(), // 8f
        instr!(branches::bcc, "bcc", 2 ), // 90
        instr!(sta::indirect_y, "sta::indirect_y", 2 ), // 91
        err!(), // 92
        err!(), // 93
        instr!(sty::zeropage_x, "sty::zeropage_x", 2 ), // 94
        instr!(sta::zeropage_x, "sta::zeropage_x", 2 ), // 95
        instr!(stx::zeropage_y, "stx::zeropage_y", 2 ), // 96
        err!(), // 97
        instr!(transfers::tya, "tya::implied", 1 ), // 98
        instr!(sta::absolute_y, "sta::absolute_y", 3 ), // 99
        instr!(transfers::txs, "txs::implied", 1 ), // 9a
        err!(), // 9b
        err!(), // 9c
        instr!(sta::absolute_x, "sta::absolute_x", 3 ), // 9d
        err!(), // 9e
        err!(), // 9f
        instr!(ldy::immediate, "ldy::immediate", 2 ), // a0
        instr!(lda::indirect_x, "lda::indirect_x", 2 ), // a1
        instr!(ldx::immediate, "ldx::immediate", 2 ), // a2
        err!(), // a3
        instr!(ldy::zeropage, "ldy::zeropage", 2 ), // a4
        instr!(lda::zeropage, "lda::zeropage", 2 ), // a5
        instr!(ldx::zeropage, "ldx::zeropage", 2 ), // a6
        err!(), // a7
        instr!(transfers::tay, "tay::implied", 1 ), // a8
        instr!(lda::immediate, "lda::immediate", 2 ), // a9
        instr!(transfers::tax, "tax::implied", 1 ), // aa
        err!(), // ab
        instr!(ldy::absolute, "ldy::absolute", 3 ), // ac
        instr!(lda::absolute, "lda::absolute", 3 ), // ad
        instr!(ldx::absolute, "ldx::absolute", 3 ), // ae
        err!(), // af
        instr!(branches::bcs, "bcs", 2 ), // b0
        instr!(lda::indirect_y, "lda::indirect_y", 2 ), // b1
        err!(), // b2
        err!(), // b3
        instr!(ldy::zeropage_x, "ldy::zeropage_x", 2 ), // b4
        instr!(lda::zeropage_x, "lda::zeropage_x", 2 ), // b5
        instr!(ldx::zeropage_y, "ldx::zeropage_y", 2 ), // b6
        err!(), // b7
        instr!(flags::clv, "clv::implied", 1 ), // b8
        instr!(lda::absolute_y, "lda::absolute_y", 3 ), // b9
        instr!(transfers::tsx, "tsx::implied", 1 ), // ba
        err!(), // bb
        instr!(ldy::absolute_x, "ldy::absolute_x", 3 ), // bc
        instr!(lda::absolute_x, "lda::absolute_x", 3 ), // bd
        instr!(ldx::absolute_y, "ldx::absolute_y", 3 ), // be
        err!(), // bf
        instr!(cpy::immediate, "cpy::immediate", 2 ), // c0
        instr!(cmp::indirect_x, "cmp::indirect_x", 2 ), // c1
        err!(), // c2
        err!(), // c3
        instr!(cpy::zeropage, "cpy::zeropage", 2 ), // c4
        instr!(cmp::zeropage, "cmp::zeropage", 2 ), // c5
        instr!(dec::zeropage, "dec::zeropage", 2 ), // c6
        err!(), // c7
        instr!(iny::implied, "iny::implied", 1 ), // c8
        instr!(cmp::immediate, "cmp::immediate", 2 ), // c9
        instr!(dex::implied, "dex::implied", 1 ), // ca
        err!(), // cb
        instr!(cpy::absolute, "cpy::absolute", 3 ), // cc
        instr!(cmp::absolute, "cmp::absolute", 3 ), // cd
        instr!(dec::absolute, "dec::absolute", 3 ), // ce
        err!(), // cf
        instr!(branches::bne, "bne", 2 ), // d0
        instr!(cmp::indirect_y, "cmp::indirect_y", 2 ), // d1
        err!(), // d2
        err!(), // d3
        err!(), // d4
        instr!(cmp::zeropage_x, "cmp::zeropage_x", 2 ), // d5
        instr!(dec::zeropage_x, "dec::zeropage_x", 2 ), // d6
        err!(), // d7
        instr!(flags::cld, "cld::implied", 1 ), // d8
        instr!(cmp::absolute_y, "cmp::absolute_y", 3 ), // d9
        err!(), // da
        err!(), // db
        err!(), // dc
        instr!(cmp::absolute_x, "cmp::absolute_x", 3 ), // dd
        instr!(dec::absolute_x, "dec::absolute_x", 3 ), // de
        err!(), // df
        instr!(cpx::immediate, "cpx::immediate", 2 ), // e0
        instr!(sbc::indirect_x, "sbc::indirect_x", 2 ), // e1
        err!(), // e2
        err!(), // e3
        instr!(cpx::zeropage, "cpx::zeropage", 2 ), // e4
        instr!(sbc::zeropage, "sbc::zeropage", 2 ), // e5
        instr!(inc::zeropage, "inc::zeropage", 2 ), // e6
        err!(), // e7
        instr!(inx::implied, "inx::implied", 1 ), // e8
        instr!(sbc::immediate, "sbc::immediate", 2 ), // e9
        instr!(others::nop, "nop::implied", 1 ), // ea
        err!(), // eb
        instr!(cpx::absolute, "cpx::absolute", 3 ), // ec
        instr!(sbc::absolute, "sbc::absolute", 3 ), // ed
        instr!(inc::absolute, "inc::absolute", 3 ), // ee
        err!(), // ef
        instr!(branches::beq, "beq", 2 ), // f0
        instr!(sbc::indirect_y, "sbc::indirect_y", 2 ), // f1
        err!(), // f2
        err!(), // f3
        err!(), // f4
        instr!(sbc::zeropage_x, "sbc::zeropage_x", 2 ), // f5
        instr!(inc::zeropage_x, "inc::zeropage_x", 2 ), // f6
        err!(), // f7
        instr!(flags::sed, "sed::implied", 1 ), // f8
        instr!(sbc::absolute_y, "sbc::absolute_y", 3 ), // f9
        err!(), // fa
        err!(), // fb
        err!(), // fc
        instr!(sbc::absolute_x, "sbc::absolute_x", 3 ), // fd
        instr!(inc::absolute_x, "inc::absolute_x", 3 ), // fe
        instr!(error_fn, "error_fn", 255) // ff
    ])}};
}

pub type InstrFn = Box<dyn Fn(&mut Cpu) -> (u8, u8)>;

pub struct Instr {
    pub fun: InstrFn,
    pub fname: String,
    pub ilen: usize,
}

impl Instr {
    fn new(fun: InstrFn, fname: &str, ilen: usize) -> Self {
        Instr {
            fun,
            fname: fname.to_string(),
            ilen,
        }
    }
}

pub fn error_fn(_cpu: &mut Cpu) -> (u8, u8) {
    // panic!("Invalid opcode!");
    (0xFF, 0xFF)
}

pub fn get_fname_for_print(instr: &Instr, data: &[u8]) -> String {
    let codes = data
        .iter()
        .skip(1)
        .map(|v| hex!(v))
        .collect::<Vec<_>>()
        .join(" ");

    let pieces: Vec<&str> = instr.fname.split("::").collect();

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
        _ => match instr.ilen {
            1 => instr_name.to_string(),
            _ => format!("{} {}", instr_name, codes),
        },
    };
    format!("({}) {}", hex!(data[0]), ret)
}

pub fn disassemble_instr(prg: &[u8], current: usize) -> (String, usize) {
    let opcode: u8 = prg[current];

    let instr = &INSTR_TABLE[opcode as usize];
    let Instr {
        fname, ilen, ..
    } = instr;

    let mut ilen = *ilen;
    let is_error = ilen == 0xFF;

    if is_error {
        ilen = 1;
    }

    let msg = if is_error {
        format!("{} ({})", fname, hex!(opcode))
    } else {
        get_fname_for_print(instr, &prg[current..current + ilen])
    };

    (msg, current + ilen)
}
