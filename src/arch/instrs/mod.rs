#[allow(non_snake_case)]
#[allow(dead_code)]
#[macro_use]
pub mod instr_table;
pub mod lda;
pub mod ldx;
pub mod ldy;
pub mod sta;
pub mod stx;
pub mod sty;
pub mod inc;
pub mod inx;
pub mod iny;
pub mod dec;
pub mod dex;
pub mod dey;
pub mod adc;
pub mod sbc;
pub mod and;
pub mod ora;
pub mod eor;
pub mod jmp;
pub mod branches;
pub mod cmp;
pub mod cpx;
pub mod cpy;
pub mod bit;
pub mod asl;
pub mod lsr;
pub mod rol;
pub mod ror;
pub mod transfers;
pub mod pushpop;
pub mod subroutines;
pub mod flags;
pub mod others;