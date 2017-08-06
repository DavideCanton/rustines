#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod registers;
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)] // to remove when memory implemented
pub mod memory;
#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod cpu;
#[allow(non_snake_case)]
#[allow(dead_code)]
#[macro_use]
pub mod instrs;
mod arch_tests;
