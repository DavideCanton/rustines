use crate::arch::rom_structs::{Bank, Header};
use log::{info, log};
use std::any::Any;

pub trait Mapper: Any {
    fn as_any(&self) -> &Any;

    fn load_prg_rom(&self, buf: &[u8], header: &Header) -> Vec<Bank> {
        info!("Rom has trainer? {}", header.has_trainer());

        let trainer_size = if header.has_trainer() { 512 } else { 0 };
        let prg_start = 16 + trainer_size as usize;
        let prg_end = prg_start + header.prg_rom_size();

        info!("PRG ROM from {:#x} to {:#x}", prg_start, prg_end);
        info!("Number of PRG banks: {}", header.prg_rom_banks());

        let mut banks = Vec::with_capacity(header.prg_rom_banks());

        for i in 0..header.prg_rom_banks() {
            let start = prg_start + i * 0x4000;
            let end = start + 0x4000;

            info!("Bank {}", i);
            info!("{:#x}-{:#x}", start, end);

            let bank = Bank {
                ram: false,
                enabled: true,
                writable: false,
                battery: false,
                id: i.to_string(),
                data: Vec::from(&buf[start..end]),
            };

            banks.push(bank)
        }

        banks
    }
}
