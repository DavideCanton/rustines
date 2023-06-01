#[allow(non_snake_case)]
#[allow(dead_code)]
#[macro_use]
pub mod instr_table;
pub mod adc;
pub mod and;
pub mod asl;
pub mod bit;
pub mod branches;
pub mod cmp;
pub mod cpx;
pub mod cpy;
pub mod dec;
pub mod dex;
pub mod dey;
pub mod eor;
pub mod flags;
pub mod inc;
pub mod inx;
pub mod iny;
pub mod jmp;
pub mod lda;
pub mod ldx;
pub mod ldy;
pub mod lsr;
pub mod ora;
pub mod others;
pub mod pushpop;
pub mod rol;
pub mod ror;
pub mod sbc;
pub mod sta;
pub mod stx;
pub mod sty;
pub mod subroutines;
pub mod transfers;
