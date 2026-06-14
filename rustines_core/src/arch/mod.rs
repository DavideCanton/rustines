#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod cpu;
#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod instrs;
pub mod mappers;
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)] // to remove when memory implemented
pub mod memory;
#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod registers;
#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod rom_structs;

#[cfg(test)]
mod arch_tests;
