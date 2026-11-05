use anyhow::bail;
use rustines_macro::Named;

use crate::arch::mappers::common::{ChrRomBank, PrgRomBank, get_bank_coordinates, get_rom_banks};
use crate::arch::mappers::mapper::Mapper;
use crate::arch::rom_structs::{
    CHR_ROM_BANK_SIZE, INesHeader, MirroringType, PRG_ROM_BANK_SIZE, TRAINER_SIZE,
};
use crate::utils::named::Named;

const CHR_RAM_SIZE: usize = 8192;

#[derive(Named, Debug)]
pub struct Mapper0 {
    prg_rom: Vec<PrgRomBank>,
    chr_rom: Option<Vec<ChrRomBank>>,
    chr_ram: Option<[u8; CHR_RAM_SIZE]>,
    mirroring_type: MirroringType,
}

impl Mapper0 {
    pub fn new(header: &INesHeader, mut rom: Vec<u8>) -> anyhow::Result<Self> {
        if header.has_trainer() {
            let _ = rom.drain(0..TRAINER_SIZE).collect::<Vec<_>>();
        }

        let banks = header.prg_rom_banks();
        if banks != 1 && banks != 2 {
            bail!("Invalid number of PRG banks, expected 1 or 2, found {banks}")
        }

        let mut prg_rom = get_rom_banks(&mut rom, header.prg_rom_size());
        if banks == 1 {
            prg_rom.push(prg_rom[0]);
        }

        let (chr_rom, chr_ram) = if header.uses_chr_ram() {
            (None, Some([0; CHR_RAM_SIZE]))
        } else {
            (Some(get_rom_banks(&mut rom, header.chr_rom_size())), None)
        };

        Ok(Mapper0 {
            prg_rom,
            chr_rom,
            chr_ram,
            mirroring_type: header.mirroring_type(),
        })
    }
}

impl Mapper for Mapper0 {
    fn fetch_cpu(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0xFFFF => {
                let (index, offset) = get_bank_coordinates(addr - 0x8000, PRG_ROM_BANK_SIZE as u16);
                self.prg_rom[index as usize][offset as usize]
            }
            _ => 0,
        }
    }

    fn fetch_ppu(&self, addr: u16) -> u8 {
        if addr <= 0x1FFF {
            if let Some(chr_ram) = self.chr_ram {
                chr_ram[addr as usize]
            } else {
                let (index, offset) = get_bank_coordinates(addr, CHR_ROM_BANK_SIZE as u16);
                debug_assert_eq!(index, 0);
                self.chr_rom.as_ref().unwrap()[index as usize][offset as usize]
            }
        } else {
            0
        }
    }

    fn store_ppu(&mut self, addr: u16, val: u8) {
        if addr <= 0x1FFF
            && let Some(chr_ram) = self.chr_ram.as_mut()
        {
            chr_ram[addr as usize] = val;
        }
    }

    fn store_cpu(&mut self, _addr: u16, _val: u8) {}

    fn mirroring_mode(&self) -> MirroringType {
        self.mirroring_type
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        INesHeader,
        arch::{mappers::Mapper0, rom_structs::HeaderFlags},
    };

    #[test]
    pub fn new_invalid_banks() {
        let header = INesHeader {
            header: [0; 4],
            flags: HeaderFlags(0),
            chr_rom_banks: 1,
            flags_9: 0,
            flags_10: 0,
            prg_rom_banks: 3,
            prg_ram_banks: 0,
            _padding: [0; 5],
        };
        let mapper = Mapper0::new(&header, vec![0; 1 << 16]);
        assert!(mapper.is_err());
        assert_eq!(
            mapper.unwrap_err().to_string(),
            "Invalid number of PRG banks, expected 1 or 2, found 3"
        )
    }
}
