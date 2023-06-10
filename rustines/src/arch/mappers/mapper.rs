use log::info;

use crate::arch::memory::FetchStore;
use crate::arch::rom_structs::{Bank, INesHeader, PRG_ROM_BANK_SIZE};
use crate::utils::named::Named;

pub trait Mapper: Named + FetchStore {
    fn load_prg_rom(&self, buf: &[u8], header: &INesHeader) -> Vec<Bank> {
        info!("Rom has trainer? {}", header.has_trainer());

        let trainer_size = if header.has_trainer() { 512 } else { 0 };
        let prg_start = 0x10 + trainer_size as usize;
        let prg_end = prg_start + header.prg_rom_size();

        info!("PRG ROM from {:#x} to {:#x}", prg_start, prg_end);
        info!("Number of PRG banks: {}", header.prg_rom_banks());

        let mut banks = Vec::with_capacity(header.prg_rom_banks());

        for i in 0..header.prg_rom_banks() {
            let start = prg_start + i * PRG_ROM_BANK_SIZE;
            let end = start + PRG_ROM_BANK_SIZE;

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
