use anyhow::bail;
use rustines_macro::{Memento, Named};

use crate::{
    arch::{
        mappers::{
            common::{BankedMemory, ChrMemory, skip_trainer},
            mapper::Mapper,
        },
        memento::{
            MementoReadAndFill, MementoReader, MementoResult, MementoWriteable, MementoWriter,
        },
        rom_structs::{INesHeader, MirroringType},
    },
    utils::named::Named,
};

#[derive(Named, Debug, Memento)]
pub struct Mapper0 {
    /// 16KB or 32KB, a 16KB PRG ROM is mirrored on $C000-$FFFF.
    prg_rom: BankedMemory,
    #[memento(fill)]
    chr: ChrMemory,
    mirroring_type: MirroringType,
}

impl Mapper0 {
    pub fn new(header: &INesHeader, mut rom: Vec<u8>) -> anyhow::Result<Self> {
        skip_trainer(header, &mut rom);

        let banks = header.prg_rom_banks();
        if banks != 1 && banks != 2 {
            bail!("Invalid number of PRG banks, expected 1 or 2, found {banks}")
        }

        let prg_rom = BankedMemory::from_rom(&mut rom, header.prg_rom_size());
        let chr = ChrMemory::new(header, &mut rom);

        Ok(Mapper0 {
            prg_rom,
            chr,
            mirroring_type: header.mirroring_type(),
        })
    }
}

impl Mapper for Mapper0 {
    fn fetch_cpu(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0xFFFF => self.prg_rom.read((addr - 0x8000) as usize),
            _ => 0,
        }
    }

    fn fetch_ppu(&self, addr: u16) -> u8 {
        if addr <= 0x1FFF {
            self.chr.read_banked(0, 0x2000, addr)
        } else {
            0
        }
    }

    fn store_ppu(&mut self, addr: u16, val: u8) {
        if addr <= 0x1FFF {
            self.chr.write_banked(0, 0x2000, addr, val);
        }
    }

    fn store_cpu(&mut self, _addr: u16, _val: u8) {}

    fn mirroring_mode(&self) -> MirroringType {
        self.mirroring_type
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::Mapper0;

    use crate::arch::{
        mappers::{
            common::test_utils::{
                FLAG_TRAINER, FLAG_VERTICAL, build_rom, chr_byte, header, prg_byte,
            },
            mapper::Mapper,
        },
        rom_structs::{CHR_ROM_BANK_SIZE, MirroringType, PRG_ROM_BANK_SIZE},
    };

    #[test_case(0)]
    #[test_case(3)]
    #[test_case(255)]
    pub fn new_invalid_banks(banks: u8) {
        let header = header(banks, 1, 0, 0);
        let mapper = Mapper0::new(&header, vec![0; 1 << 16]);
        assert!(mapper.is_err());
        assert_eq!(
            mapper.unwrap_err().to_string(),
            format!("Invalid number of PRG banks, expected 1 or 2, found {banks}")
        )
    }

    #[test_case(1)]
    #[test_case(2)]
    pub fn new_valid_banks(banks: u8) {
        let header = header(banks, 1, 0, 0);
        let rom = build_rom(banks as usize, 1, false);
        assert!(Mapper0::new(&header, rom).is_ok());
    }

    #[test]
    pub fn fetch_cpu_two_banks() {
        let mapper = Mapper0::new(&header(2, 1, 0, 0), build_rom(2, 1, false)).unwrap();

        for offset in [0usize, 1, 0x3F, 0x40, 0x1234, PRG_ROM_BANK_SIZE - 1] {
            assert_eq!(
                mapper.fetch_cpu(0x8000 + offset as u16),
                prg_byte(0, offset)
            );
            assert_eq!(
                mapper.fetch_cpu(0xC000 + offset as u16),
                prg_byte(1, offset)
            );
        }
        assert_eq!(mapper.fetch_cpu(0xFFFF), prg_byte(1, PRG_ROM_BANK_SIZE - 1));
    }

    #[test]
    pub fn fetch_cpu_one_bank_is_mirrored() {
        let mapper = Mapper0::new(&header(1, 1, 0, 0), build_rom(1, 1, false)).unwrap();

        for offset in [0usize, 1, 0x3F, 0x40, 0x1234, PRG_ROM_BANK_SIZE - 1] {
            let low = mapper.fetch_cpu(0x8000 + offset as u16);
            let high = mapper.fetch_cpu(0xC000 + offset as u16);
            assert_eq!(low, prg_byte(0, offset));
            assert_eq!(high, low);
        }
    }

    #[test_case(0x0000)]
    #[test_case(0x2000)]
    #[test_case(0x4020)]
    #[test_case(0x6000)]
    #[test_case(0x7FFF)]
    pub fn fetch_cpu_outside_prg_rom(addr: u16) {
        let mapper = Mapper0::new(&header(2, 1, 0, 0), build_rom(2, 1, false)).unwrap();
        assert_eq!(mapper.fetch_cpu(addr), 0);
    }

    #[test]
    pub fn store_cpu_is_ignored() {
        let mut mapper = Mapper0::new(&header(2, 1, 0, 0), build_rom(2, 1, false)).unwrap();

        for addr in [0x6000u16, 0x8000, 0x8001, 0xC000, 0xFFFF] {
            let before = mapper.fetch_cpu(addr);
            mapper.store_cpu(addr, !before);
            assert_eq!(mapper.fetch_cpu(addr), before);
        }
    }

    #[test]
    pub fn fetch_ppu_chr_rom() {
        let mapper = Mapper0::new(&header(2, 1, 0, 0), build_rom(2, 1, false)).unwrap();

        for offset in [0usize, 1, 0x3F, 0x40, 0x1000, CHR_ROM_BANK_SIZE - 1] {
            assert_eq!(mapper.fetch_ppu(offset as u16), chr_byte(0, offset));
        }
    }

    #[test_case(0x2000)]
    #[test_case(0x2400)]
    #[test_case(0x3F00)]
    #[test_case(0x3FFF)]
    pub fn fetch_ppu_outside_pattern_tables(addr: u16) {
        let rom_mapper = Mapper0::new(&header(2, 1, 0, 0), build_rom(2, 1, false)).unwrap();
        assert_eq!(rom_mapper.fetch_ppu(addr), 0);

        let ram_mapper = Mapper0::new(&header(2, 0, 0, 0), build_rom(2, 0, false)).unwrap();
        assert_eq!(ram_mapper.fetch_ppu(addr), 0);
    }

    #[test]
    pub fn store_ppu_chr_rom_is_read_only() {
        let mut mapper = Mapper0::new(&header(2, 1, 0, 0), build_rom(2, 1, false)).unwrap();

        for addr in [0x0000u16, 0x0FFF, 0x1000, 0x1FFF] {
            let before = mapper.fetch_ppu(addr);
            mapper.store_ppu(addr, !before);
            assert_eq!(mapper.fetch_ppu(addr), before);
        }
    }

    #[test]
    pub fn chr_ram_starts_zeroed() {
        let mapper = Mapper0::new(&header(2, 0, 0, 0), build_rom(2, 0, false)).unwrap();

        for addr in 0..=0x1FFFu16 {
            assert_eq!(mapper.fetch_ppu(addr), 0);
        }
    }

    #[test]
    pub fn chr_ram_read_write() {
        let mut mapper = Mapper0::new(&header(2, 0, 0, 0), build_rom(2, 0, false)).unwrap();

        for addr in 0..=0x1FFFu16 {
            mapper.store_ppu(addr, (addr ^ (addr >> 8)) as u8);
        }
        for addr in 0..=0x1FFFu16 {
            assert_eq!(mapper.fetch_ppu(addr), (addr ^ (addr >> 8)) as u8);
        }
    }

    #[test_case(0x2000)]
    #[test_case(0x3FFF)]
    pub fn chr_ram_store_outside_pattern_tables_is_ignored(addr: u16) {
        let mut mapper = Mapper0::new(&header(2, 0, 0, 0), build_rom(2, 0, false)).unwrap();

        mapper.store_ppu(addr, 0xAB);
        assert_eq!(mapper.fetch_ppu(addr), 0);
        // must not wrap into the pattern tables
        assert_eq!(mapper.fetch_ppu(addr & 0x1FFF), 0);
    }

    #[test]
    pub fn trainer_is_skipped() {
        let header = header(2, 1, 0, FLAG_TRAINER);
        let mapper = Mapper0::new(&header, build_rom(2, 1, true)).unwrap();

        assert_eq!(mapper.fetch_cpu(0x8000), prg_byte(0, 0));
        assert_eq!(mapper.fetch_cpu(0xBFFF), prg_byte(0, PRG_ROM_BANK_SIZE - 1));
        assert_eq!(mapper.fetch_cpu(0xC000), prg_byte(1, 0));
        assert_eq!(mapper.fetch_cpu(0xFFFF), prg_byte(1, PRG_ROM_BANK_SIZE - 1));
        assert_eq!(mapper.fetch_ppu(0x0000), chr_byte(0, 0));
        assert_eq!(mapper.fetch_ppu(0x1FFF), chr_byte(0, CHR_ROM_BANK_SIZE - 1));
    }

    #[test_case(0, MirroringType::Horizontal)]
    #[test_case(FLAG_VERTICAL, MirroringType::Vertical)]
    #[test_case(FLAG_VERTICAL | FLAG_TRAINER, MirroringType::Vertical)]
    pub fn mirroring_mode(flags: u16, expected: MirroringType) {
        let trainer = flags & FLAG_TRAINER != 0;
        let mapper = Mapper0::new(&header(1, 1, 0, flags), build_rom(1, 1, trainer)).unwrap();
        assert_eq!(mapper.mirroring_mode(), expected);
    }
}
