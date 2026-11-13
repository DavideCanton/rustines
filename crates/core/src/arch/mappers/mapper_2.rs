use crate::arch::{
    mappers::{
        common::{BankedMemory, ChrMemory, skip_trainer},
        mapper::Mapper,
    },
    rom_structs::{INesHeader, MirroringType, PRG_ROM_BANK_SIZE},
};
use rustines_macro::{Memento, Named};

#[derive(Named, Memento)]
pub struct Mapper2 {
    prg_rom: BankedMemory,
    #[memento(fill)]
    chr: ChrMemory,
    mirroring_type: MirroringType,
    /// 16KB bank mapped on $8000-$BFFF; out of range values wrap around.
    #[memento]
    selected_bank: u8,
    /// 16KB bank fixed on $C000-$FFFF.
    #[memento]
    last_bank: usize,
}

impl Mapper2 {
    pub fn new(header: &INesHeader, mut rom: Vec<u8>) -> anyhow::Result<Self> {
        skip_trainer(header, &mut rom);

        let prg_rom = BankedMemory::from_rom(&mut rom, header.prg_rom_size());
        let chr = ChrMemory::new(header, &mut rom);
        let last_bank = prg_rom.banks(PRG_ROM_BANK_SIZE) - 1;

        Ok(Mapper2 {
            prg_rom,
            chr,
            mirroring_type: header.mirroring_type(),
            selected_bank: 0,
            last_bank,
        })
    }
}

impl Mapper for Mapper2 {
    fn reset(&mut self) {
        self.selected_bank = 0;
    }

    fn fetch_cpu(&self, addr: u16) -> u8 {
        match addr {
            0x6000..=0x7FFF => 0,
            0x8000..=0xBFFF => {
                self.prg_rom
                    .read_banked(self.selected_bank as usize, PRG_ROM_BANK_SIZE, addr)
            }
            0xC000..=0xFFFF => self
                .prg_rom
                .read_banked(self.last_bank, PRG_ROM_BANK_SIZE, addr),
            _ => panic!("Invalid value provided to fetch_cpu: {addr}"),
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

    fn store_cpu(&mut self, addr: u16, val: u8) {
        if addr >= 0x8000 {
            self.selected_bank = val;
        }
    }

    fn mirroring_mode(&self) -> MirroringType {
        self.mirroring_type
    }
}

#[cfg(test)]
mod tests {
    use super::Mapper2;
    use crate::arch::{
        mappers::{
            common::test_utils::{
                FLAG_TRAINER, FLAG_VERTICAL, build_rom, chr_byte, header, prg_byte,
            },
            mapper::Mapper,
        },
        rom_structs::{CHR_ROM_BANK_SIZE, MirroringType, PRG_ROM_BANK_SIZE},
    };
    use test_case::test_case;

    const LAST: usize = PRG_ROM_BANK_SIZE - 1;

    fn mapper(prg_banks: u8, chr_banks: u8) -> Mapper2 {
        Mapper2::new(
            &header(prg_banks, chr_banks, 0, 0),
            build_rom(prg_banks as usize, chr_banks as usize, false),
        )
        .unwrap()
    }

    #[test]
    pub fn default_banks() {
        let mapper = mapper(8, 1);

        assert_eq!(mapper.fetch_cpu(0x8000), prg_byte(0, 0));
        assert_eq!(mapper.fetch_cpu(0xBFFF), prg_byte(0, LAST));
        assert_eq!(mapper.fetch_cpu(0xC000), prg_byte(7, 0));
        assert_eq!(mapper.fetch_cpu(0xFFFF), prg_byte(7, LAST));
    }

    #[test]
    pub fn switch_lower_bank() {
        let mut mapper = mapper(8, 1);

        for bank in 0..8u8 {
            mapper.store_cpu(0x8000, bank);
            for offset in [0usize, 1, 0x1234, LAST] {
                assert_eq!(
                    mapper.fetch_cpu(0x8000 + offset as u16),
                    prg_byte(bank as usize, offset)
                );
                // upper bank is always fixed to the last one
                assert_eq!(
                    mapper.fetch_cpu(0xC000 + offset as u16),
                    prg_byte(7, offset)
                );
            }
        }
    }

    #[test_case(0x8000)]
    #[test_case(0x9FFF)]
    #[test_case(0xC000)]
    #[test_case(0xFFFF)]
    pub fn bank_select_from_any_rom_address(addr: u16) {
        let mut mapper = mapper(8, 1);

        mapper.store_cpu(addr, 5);
        assert_eq!(mapper.fetch_cpu(0x8000), prg_byte(5, 0));
    }

    #[test_case(4, 0x05, 1)]
    #[test_case(4, 0xFF, 3)]
    #[test_case(8, 0x0A, 2)]
    #[test_case(16, 0x1F, 15)]
    pub fn bank_select_is_masked(banks: u8, value: u8, expected: usize) {
        let mut mapper = mapper(banks, 1);

        mapper.store_cpu(0x8000, value);
        assert_eq!(mapper.fetch_cpu(0x8000), prg_byte(expected, 0));
    }

    #[test]
    pub fn reset_selects_bank_0() {
        let mut mapper = mapper(8, 1);

        mapper.store_cpu(0x8000, 3);
        mapper.reset();
        assert_eq!(mapper.fetch_cpu(0x8000), prg_byte(0, 0));
        assert_eq!(mapper.fetch_cpu(0xC000), prg_byte(7, 0));
    }

    #[test_case(0x6000)]
    #[test_case(0x7FFF)]
    pub fn no_prg_ram(addr: u16) {
        let mapper = mapper(8, 1);
        assert_eq!(mapper.fetch_cpu(addr), 0);
    }

    #[test]
    pub fn store_outside_rom_does_not_switch_bank() {
        let mut mapper = mapper(8, 1);

        for addr in [0x4020u16, 0x6000, 0x7FFF] {
            mapper.store_cpu(addr, 3);
            assert_eq!(mapper.fetch_cpu(0x8000), prg_byte(0, 0));
        }
    }

    #[test]
    pub fn chr_rom() {
        let mut mapper = mapper(8, 1);

        for offset in [0usize, 1, 0x0FFF, 0x1000, CHR_ROM_BANK_SIZE - 1] {
            let addr = offset as u16;
            assert_eq!(mapper.fetch_ppu(addr), chr_byte(0, offset));
            mapper.store_ppu(addr, !chr_byte(0, offset));
            assert_eq!(mapper.fetch_ppu(addr), chr_byte(0, offset));
        }
    }

    #[test]
    pub fn chr_ram() {
        let mut mapper = mapper(8, 0);

        for addr in 0..=0x1FFFu16 {
            assert_eq!(mapper.fetch_ppu(addr), 0);
            mapper.store_ppu(addr, (addr ^ (addr >> 8)) as u8);
        }
        for addr in 0..=0x1FFFu16 {
            assert_eq!(mapper.fetch_ppu(addr), (addr ^ (addr >> 8)) as u8);
        }
    }

    #[test_case(0x2000)]
    #[test_case(0x3FFF)]
    pub fn ppu_outside_pattern_tables(addr: u16) {
        let mut mapper = mapper(8, 0);

        mapper.store_ppu(addr, 0xAB);
        assert_eq!(mapper.fetch_ppu(addr), 0);
        assert_eq!(mapper.fetch_ppu(addr & 0x1FFF), 0);
        assert_eq!(self::mapper(8, 1).fetch_ppu(addr), 0);
    }

    #[test]
    pub fn trainer_is_skipped() {
        let mapper = Mapper2::new(&header(4, 1, 0, FLAG_TRAINER), build_rom(4, 1, true)).unwrap();

        assert_eq!(mapper.fetch_cpu(0x8000), prg_byte(0, 0));
        assert_eq!(mapper.fetch_cpu(0xFFFF), prg_byte(3, LAST));
        assert_eq!(mapper.fetch_ppu(0x0000), chr_byte(0, 0));
        assert_eq!(mapper.fetch_ppu(0x1FFF), chr_byte(0, CHR_ROM_BANK_SIZE - 1));
    }

    #[test_case(0, MirroringType::Horizontal)]
    #[test_case(FLAG_VERTICAL, MirroringType::Vertical)]
    pub fn mirroring_mode(flags: u16, expected: MirroringType) {
        let mapper = Mapper2::new(&header(2, 1, 0, flags), build_rom(2, 1, false)).unwrap();
        assert_eq!(mapper.mirroring_mode(), expected);
    }
}
