use anyhow::bail;
use rustines_macro::{Memento, Named};

use crate::arch::{
    mappers::{
        common::{BankedMemory, skip_trainer},
        mapper::Mapper,
    },
    rom_structs::{CHR_ROM_BANK_SIZE, INesHeader, MirroringType},
};

#[derive(Named, Memento)]
pub struct Mapper3 {
    /// 16KB or 32KB, a 16KB PRG ROM is mirrored on $C000-$FFFF.
    prg_rom: BankedMemory,
    /// Mirrored over $6000-$7FFF when smaller than 8KB.
    /// not battery backed, so no need to implement get/set prg ram
    #[memento(fill)]
    prg_ram: Option<BankedMemory>,
    chr_rom: BankedMemory,
    mirroring_type: MirroringType,
    /// 8KB CHR bank; out of range values wrap around.
    #[memento]
    selected_bank: u8,
}

impl Mapper3 {
    pub fn new(header: &INesHeader, mut rom: Vec<u8>) -> anyhow::Result<Self> {
        skip_trainer(header, &mut rom);

        let banks = header.prg_rom_banks();
        if banks != 1 && banks != 2 {
            bail!("Invalid number of PRG banks, expected 1 or 2, found {banks}")
        }

        let prg_rom = BankedMemory::from_rom(&mut rom, header.prg_rom_size());

        if header.uses_chr_ram() {
            bail!("Unexpected CHR RAM for mapper 3");
        }
        let chr_rom = BankedMemory::from_rom(&mut rom, header.chr_rom_size());

        let prg_ram = header
            .uses_prg_ram()
            .then(|| BankedMemory::zeroed(header.prg_ram_size()));

        Ok(Mapper3 {
            prg_rom,
            prg_ram,
            chr_rom,
            mirroring_type: header.mirroring_type(),
            selected_bank: 0,
        })
    }
}

impl Mapper for Mapper3 {
    fn reset(&mut self) {
        self.selected_bank = 0;
    }

    fn fetch_cpu(&self, addr: u16) -> u8 {
        match addr {
            0x6000..=0x7FFF => self
                .prg_ram
                .as_ref()
                .map_or(0, |prg_ram| prg_ram.read((addr - 0x6000) as usize)),
            0x8000..=0xFFFF => self.prg_rom.read((addr - 0x8000) as usize),
            _ => panic!("Invalid value provided to fetch_cpu: {addr}"),
        }
    }

    fn fetch_ppu(&self, addr: u16) -> u8 {
        if addr <= 0x1FFF {
            self.chr_rom
                .read_banked(self.selected_bank as usize, CHR_ROM_BANK_SIZE, addr)
        } else {
            0
        }
    }

    fn store_ppu(&mut self, _addr: u16, _val: u8) {}

    fn store_cpu(&mut self, addr: u16, val: u8) {
        match addr {
            0x6000..=0x7FFF => {
                if let Some(prg_ram) = self.prg_ram.as_mut() {
                    prg_ram.write((addr - 0x6000) as usize, val);
                }
            }
            0x8000..=0xFFFF => self.selected_bank = val,
            _ => {}
        }
    }

    fn mirroring_mode(&self) -> MirroringType {
        self.mirroring_type
    }
}

#[cfg(test)]
mod tests {
    use super::Mapper3;
    use test_case::test_case;

    use crate::arch::{
        mappers::{
            common::test_utils::{
                FLAG_TRAINER, FLAG_VERTICAL, build_rom, chr_byte, header, prg_byte,
            },
            mapper::Mapper,
        },
        rom_structs::{CHR_ROM_BANK_SIZE, MirroringType, PRG_ROM_BANK_SIZE},
    };

    const LAST: usize = PRG_ROM_BANK_SIZE - 1;

    fn mapper(chr_banks: u8, prg_ram_banks: u8) -> Mapper3 {
        Mapper3::new(
            &header(2, chr_banks, prg_ram_banks, 0),
            build_rom(2, chr_banks as usize, false),
        )
        .unwrap()
    }

    #[test_case(0)]
    #[test_case(3)]
    #[test_case(4)]
    #[test_case(255)]
    pub fn new_invalid_prg_banks(banks: u8) {
        let result = Mapper3::new(&header(banks, 1, 0, 0), vec![0; 1 << 17]);
        assert_eq!(
            result.err().unwrap().to_string(),
            format!("Invalid number of PRG banks, expected 1 or 2, found {banks}")
        );
    }

    #[test]
    pub fn fetch_cpu_one_prg_bank_is_mirrored() {
        let mut mapper = Mapper3::new(&header(1, 4, 0, 0), build_rom(1, 4, false)).unwrap();

        for offset in [0usize, 1, 0x1234, LAST] {
            let low = mapper.fetch_cpu(0x8000 + offset as u16);
            assert_eq!(low, prg_byte(0, offset));
            assert_eq!(mapper.fetch_cpu(0xC000 + offset as u16), low);
        }

        // CHR banking is unaffected
        mapper.store_cpu(0x8000, 3);
        assert_eq!(mapper.fetch_ppu(0), chr_byte(3, 0));
    }

    #[test]
    pub fn new_chr_ram_is_rejected() {
        let result = Mapper3::new(&header(2, 0, 0, 0), build_rom(2, 0, false));
        assert_eq!(
            result.err().unwrap().to_string(),
            "Unexpected CHR RAM for mapper 3"
        );
    }

    #[test]
    pub fn fetch_cpu_prg_rom() {
        let mapper = mapper(4, 0);

        for offset in [0usize, 1, 0x1234, LAST] {
            assert_eq!(
                mapper.fetch_cpu(0x8000 + offset as u16),
                prg_byte(0, offset)
            );
            assert_eq!(
                mapper.fetch_cpu(0xC000 + offset as u16),
                prg_byte(1, offset)
            );
        }
    }

    #[test]
    pub fn prg_rom_is_not_banked() {
        let mut mapper = mapper(4, 0);

        mapper.store_cpu(0x8000, 3);
        assert_eq!(mapper.fetch_cpu(0x8000), prg_byte(0, 0));
        assert_eq!(mapper.fetch_cpu(0xC000), prg_byte(1, 0));
    }

    #[test]
    pub fn default_chr_bank() {
        let mapper = mapper(4, 0);

        for offset in [0usize, 0x0FFF, 0x1000, CHR_ROM_BANK_SIZE - 1] {
            assert_eq!(mapper.fetch_ppu(offset as u16), chr_byte(0, offset));
        }
    }

    #[test]
    pub fn switch_chr_bank() {
        let mut mapper = mapper(4, 0);

        for bank in 0..4u8 {
            mapper.store_cpu(0x8000, bank);
            for offset in [0usize, 0x0FFF, 0x1000, CHR_ROM_BANK_SIZE - 1] {
                assert_eq!(
                    mapper.fetch_ppu(offset as u16),
                    chr_byte(bank as usize, offset)
                );
            }
        }
    }

    #[test_case(0x8000)]
    #[test_case(0xBFFF)]
    #[test_case(0xC000)]
    #[test_case(0xFFFF)]
    pub fn chr_bank_select_from_any_rom_address(addr: u16) {
        let mut mapper = mapper(4, 0);

        mapper.store_cpu(addr, 2);
        assert_eq!(mapper.fetch_ppu(0), chr_byte(2, 0));
    }

    #[test_case(2, 0x03, 1)]
    #[test_case(4, 0x06, 2)]
    #[test_case(4, 0xFF, 3)]
    pub fn chr_bank_select_is_masked(banks: u8, value: u8, expected: usize) {
        let mut mapper = mapper(banks, 0);

        mapper.store_cpu(0x8000, value);
        assert_eq!(mapper.fetch_ppu(0), chr_byte(expected, 0));
    }

    #[test]
    pub fn reset_selects_chr_bank_0() {
        let mut mapper = mapper(4, 0);

        mapper.store_cpu(0x8000, 3);
        mapper.reset();
        assert_eq!(mapper.fetch_ppu(0), chr_byte(0, 0));
    }

    #[test]
    pub fn chr_rom_is_read_only() {
        let mut mapper = mapper(4, 0);

        for addr in [0x0000u16, 0x1000, 0x1FFF] {
            let before = mapper.fetch_ppu(addr);
            mapper.store_ppu(addr, !before);
            assert_eq!(mapper.fetch_ppu(addr), before);
        }
    }

    #[test_case(0x2000)]
    #[test_case(0x3FFF)]
    pub fn ppu_outside_pattern_tables(addr: u16) {
        assert_eq!(mapper(4, 0).fetch_ppu(addr), 0);
    }

    #[test_case(0x6000)]
    #[test_case(0x7FFF)]
    pub fn no_prg_ram(addr: u16) {
        assert_eq!(mapper(4, 0).fetch_cpu(addr), 0);
    }

    #[test_case(0x6000)]
    #[test_case(0x7FFF)]
    pub fn prg_ram_starts_zeroed(addr: u16) {
        assert_eq!(mapper(4, 1).fetch_cpu(addr), 0);
    }

    #[test]
    pub fn prg_ram_read_write() {
        let mut mapper = mapper(4, 1);

        mapper.store_cpu(0x6000, 0x42);
        mapper.store_cpu(0x6123, 0x24);
        assert_eq!(mapper.fetch_cpu(0x6000), 0x42);
        assert_eq!(mapper.fetch_cpu(0x6123), 0x24);
    }

    #[test]
    pub fn store_outside_rom_does_not_switch_bank() {
        let mut mapper = mapper(4, 0);

        for addr in [0x4020u16, 0x6000, 0x7FFF] {
            mapper.store_cpu(addr, 3);
            assert_eq!(mapper.fetch_ppu(0), chr_byte(0, 0));
        }
    }

    #[test]
    pub fn trainer_is_skipped() {
        let mapper = Mapper3::new(&header(2, 2, 0, FLAG_TRAINER), build_rom(2, 2, true)).unwrap();

        assert_eq!(mapper.fetch_cpu(0x8000), prg_byte(0, 0));
        assert_eq!(mapper.fetch_cpu(0xFFFF), prg_byte(1, LAST));
        assert_eq!(mapper.fetch_ppu(0x0000), chr_byte(0, 0));
        assert_eq!(mapper.fetch_ppu(0x1FFF), chr_byte(0, CHR_ROM_BANK_SIZE - 1));
    }

    #[test_case(0, MirroringType::Horizontal)]
    #[test_case(FLAG_VERTICAL, MirroringType::Vertical)]
    pub fn mirroring_mode(flags: u16, expected: MirroringType) {
        let mapper = Mapper3::new(&header(2, 1, 0, flags), build_rom(2, 1, false)).unwrap();
        assert_eq!(mapper.mirroring_mode(), expected);
    }
}
