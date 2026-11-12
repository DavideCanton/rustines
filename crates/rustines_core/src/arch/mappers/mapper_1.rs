use rustines_macro::Named;

use crate::{
    arch::{
        mappers::{
            common::{BankedMemory, ChrMemory, skip_trainer},
            mapper::Mapper,
        },
        memento::{
            MementoReadAndFill, MementoReader, MementoResult, MementoWriteable, MementoWriter,
        },
        rom_structs::{
            CHR_ROM_BANK_SIZE, INesHeader, MirroringType, PRG_RAM_BANK_SIZE, PRG_ROM_BANK_SIZE,
        },
    },
    utils::{
        bit_utils::{BitIndex, BitManip},
        named::Named,
    },
};
use bitfield::bitfield;

const CHR_4KB_BANK_SIZE: usize = 0x1000;
const PRG_32KB_BANK_SIZE: usize = 0x8000;

bitfield! {
    #[derive(Clone, Copy)]
    struct ControlRegister(u8);
    impl Debug;
    u8;
    pub nametables, set_nametables: 1, 0;
    pub prg_bank_mode, set_prg_bank_mode: 3, 2;
    pub chr_bank_mode, set_chr_bank_mode: 4;
}

impl Default for ControlRegister {
    fn default() -> Self {
        ControlRegister(0x0C)
    }
}

#[derive(Named, Debug)]
pub struct Mapper1 {
    prg_rom: BankedMemory,
    prg_ram: BankedMemory,
    chr: ChrMemory,
    /// 16KB bank fixed on $C000-$FFFF in PRG mode 3.
    last_prg_bank: usize,
    shift_register: u8,
    count: u8,
    ctrl_register: ControlRegister,
    chr_1_reg: u8,
    chr_2_reg: u8,
    prg_reg: u8,
}

impl Mapper1 {
    pub fn new(header: &INesHeader, mut rom: Vec<u8>) -> anyhow::Result<Self> {
        skip_trainer(header, &mut rom);

        let prg_rom = BankedMemory::from_rom(&mut rom, header.prg_rom_size());
        // TODO PRG RAM banking: only the first 8KB bank is reachable for now
        let prg_ram =
            BankedMemory::zeroed(PRG_RAM_BANK_SIZE * header.prg_ram_banks.max(1) as usize);
        let chr = ChrMemory::new(header, &mut rom);
        let last_prg_bank = prg_rom.banks(PRG_ROM_BANK_SIZE) - 1;

        Ok(Mapper1 {
            prg_rom,
            prg_ram,
            chr,
            last_prg_bank,
            count: 0,
            shift_register: 0,
            ctrl_register: ControlRegister::default(),
            chr_1_reg: 0,
            chr_2_reg: 0,
            prg_reg: 0,
        })
    }

    fn prg_ram_enabled(&self) -> bool {
        !self.prg_reg.get_bit(BitIndex::_4)
    }

    /// Returns the CHR bank and its size for the given PPU address.
    fn chr_bank(&self, addr: u16) -> (usize, usize) {
        if self.ctrl_register.chr_bank_mode() {
            let reg = if addr < 0x1000 {
                self.chr_1_reg
            } else {
                self.chr_2_reg
            };
            ((reg & 0x1F) as usize, CHR_4KB_BANK_SIZE)
        } else {
            (((self.chr_1_reg & 0x1E) >> 1) as usize, CHR_ROM_BANK_SIZE)
        }
    }
}

impl Mapper for Mapper1 {
    fn reset(&mut self) {
        self.count = 0;
        self.shift_register = 0;
        self.ctrl_register = ControlRegister::default();
        self.chr_1_reg = 0;
        self.chr_2_reg = 0;
        self.prg_reg = 0;
    }

    fn fetch_cpu(&self, addr: u16) -> u8 {
        match addr {
            0x6000..=0x7FFF => {
                if self.prg_ram_enabled() {
                    self.prg_ram.read((addr - 0x6000) as usize)
                } else {
                    0
                }
            }
            0x8000..=0xFFFF => {
                let prg_reg = self.prg_reg as usize;
                match (self.ctrl_register.prg_bank_mode(), addr) {
                    // 32KB mode, the low bit of the bank number is ignored
                    (0 | 1, _) => {
                        self.prg_rom
                            .read_banked((prg_reg & 0x0E) >> 1, PRG_32KB_BANK_SIZE, addr)
                    }
                    // first bank fixed on $8000, switchable bank on $C000
                    (2, 0x8000..=0xBFFF) => self.prg_rom.read_banked(0, PRG_ROM_BANK_SIZE, addr),
                    (2, _) => self
                        .prg_rom
                        .read_banked(prg_reg & 0x0F, PRG_ROM_BANK_SIZE, addr),
                    // switchable bank on $8000, last bank fixed on $C000
                    (3, 0x8000..=0xBFFF) => {
                        self.prg_rom
                            .read_banked(prg_reg & 0x0F, PRG_ROM_BANK_SIZE, addr)
                    }
                    (3, _) => self
                        .prg_rom
                        .read_banked(self.last_prg_bank, PRG_ROM_BANK_SIZE, addr),
                    _ => unreachable!(),
                }
            }
            _ => 0,
        }
    }

    fn fetch_ppu(&self, addr: u16) -> u8 {
        if addr <= 0x1FFF {
            let (bank, bank_size) = self.chr_bank(addr);
            self.chr.read_banked(bank, bank_size, addr)
        } else {
            0
        }
    }

    fn store_ppu(&mut self, addr: u16, val: u8) {
        if addr <= 0x1FFF {
            let (bank, bank_size) = self.chr_bank(addr);
            self.chr.write_banked(bank, bank_size, addr, val);
        }
    }

    fn store_cpu(&mut self, addr: u16, value: u8) {
        if addr < 0x6000 {
            return;
        }

        if (0x6000..=0x7FFF).contains(&addr) {
            if self.prg_ram_enabled() {
                self.prg_ram.write((addr - 0x6000) as usize, value);
            }
            return;
        }

        if value.get_bit(BitIndex::_7) {
            self.shift_register = 0x00;
            self.count = 0;
            self.ctrl_register.set_prg_bank_mode(3);
            return;
        }

        self.shift_register |= (value & 0x01) << self.count;
        self.count += 1;

        if self.count == 5 {
            match addr {
                0x8000..=0x9FFF => {
                    self.ctrl_register = ControlRegister(self.shift_register);
                }
                0xA000..=0xBFFF => {
                    self.chr_1_reg = self.shift_register;
                }
                0xC000..=0xDFFF => {
                    self.chr_2_reg = self.shift_register;
                }
                0xE000..=0xFFFF => {
                    self.prg_reg = self.shift_register;
                }
                _ => {}
            }

            self.shift_register = 0;
            self.count = 0;
        }
    }

    fn mirroring_mode(&self) -> MirroringType {
        match self.ctrl_register.nametables() {
            0 => MirroringType::SingleScreenLower,
            1 => MirroringType::SingleScreenUpper,
            2 => MirroringType::Vertical,
            3 => MirroringType::Horizontal,
            _ => unreachable!(),
        }
    }

    fn get_prg_ram(&self) -> Option<&[u8]> {
        Some(self.prg_ram.memory())
    }

    fn set_prg_ram(&mut self, content: &[u8]) {
        self.prg_ram.write_all(content);
    }
}

impl MementoReadAndFill for Mapper1 {
    fn read_and_fill(&mut self, data: &mut MementoReader) -> MementoResult {
        self.prg_ram.read_and_fill(data)?;
        self.chr.read_and_fill(data)?;
        self.last_prg_bank = data.read_memento()?;
        self.shift_register = data.read_memento()?;
        self.count = data.read_memento()?;
        self.ctrl_register.0 = data.read_memento()?;
        self.chr_1_reg = data.read_memento()?;
        self.chr_2_reg = data.read_memento()?;
        self.prg_reg = data.read_memento()?;
        Ok(())
    }
}

impl MementoWriteable for Mapper1 {
    fn write_memento(&self, writer: &mut dyn MementoWriter) -> MementoResult {
        self.prg_ram.write_memento(writer)?;
        self.chr.write_memento(writer)?;
        self.last_prg_bank.write_memento(writer)?;
        self.shift_register.write_memento(writer)?;
        self.count.write_memento(writer)?;
        self.ctrl_register.0.write_memento(writer)?;
        self.chr_1_reg.write_memento(writer)?;
        self.chr_2_reg.write_memento(writer)?;
        self.prg_reg.write_memento(writer)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::Mapper1;

    use crate::arch::{
        mappers::{
            common::test_utils::{FLAG_TRAINER, build_rom, chr_byte, header, prg_byte},
            mapper::Mapper,
        },
        rom_structs::{MirroringType, PRG_ROM_BANK_SIZE},
    };

    const LAST: usize = PRG_ROM_BANK_SIZE - 1;

    const CTRL: u16 = 0x8000;
    const CHR_0: u16 = 0xA000;
    const CHR_1: u16 = 0xC000;
    const PRG: u16 = 0xE000;

    fn mapper(prg_banks: u8, chr_banks: u8) -> Mapper1 {
        Mapper1::new(
            &header(prg_banks, chr_banks, 0, 0),
            build_rom(prg_banks as usize, chr_banks as usize, false),
        )
        .unwrap()
    }

    /// Writes a 5-bit value to an MMC1 register through the serial port, LSB first.
    fn write_reg(mapper: &mut Mapper1, addr: u16, value: u8) {
        for i in 0..5 {
            mapper.store_cpu(addr, (value >> i) & 1);
        }
    }

    #[test]
    pub fn power_on_state() {
        let mapper = mapper(8, 4);

        // PRG mode 3: $8000 switchable (bank 0), $C000 fixed to the last bank
        assert_eq!(mapper.fetch_cpu(0x8000), prg_byte(0, 0));
        assert_eq!(mapper.fetch_cpu(0xBFFF), prg_byte(0, LAST));
        assert_eq!(mapper.fetch_cpu(0xC000), prg_byte(7, 0));
        assert_eq!(mapper.fetch_cpu(0xFFFF), prg_byte(7, LAST));
        // CHR 8KB mode, bank 0
        assert_eq!(mapper.fetch_ppu(0x0000), chr_byte(0, 0));
        assert_eq!(mapper.fetch_ppu(0x1FFF), chr_byte(0, 0x1FFF));
        assert_eq!(mapper.mirroring_mode(), MirroringType::SingleScreenLower);
    }

    #[test_case(0, MirroringType::SingleScreenLower)]
    #[test_case(1, MirroringType::SingleScreenUpper)]
    #[test_case(2, MirroringType::Vertical)]
    #[test_case(3, MirroringType::Horizontal)]
    pub fn mirroring_mode(nametables: u8, expected: MirroringType) {
        let mut mapper = mapper(8, 4);

        write_reg(&mut mapper, CTRL, 0x0C | nametables);
        assert_eq!(mapper.mirroring_mode(), expected);
    }

    #[test]
    pub fn register_is_written_only_on_fifth_write() {
        let mut mapper = mapper(8, 4);

        for i in 0..4 {
            mapper.store_cpu(PRG, (3 >> i) & 1);
            assert_eq!(mapper.fetch_cpu(0x8000), prg_byte(0, 0));
        }
        mapper.store_cpu(PRG, 0);
        assert_eq!(mapper.fetch_cpu(0x8000), prg_byte(3, 0));
    }

    #[test]
    pub fn register_is_selected_by_fifth_write_address() {
        let mut mapper = mapper(8, 4);

        for _ in 0..4 {
            mapper.store_cpu(CTRL, 1);
        }
        mapper.store_cpu(PRG, 0);
        // value 0b01111 went to PRG, control register untouched
        assert_eq!(mapper.fetch_cpu(0x8000), prg_byte(15 % 8, 0));
        assert_eq!(mapper.mirroring_mode(), MirroringType::SingleScreenLower);
    }

    #[test_case(0x8000, 0x9FFF)]
    #[test_case(0xA000, 0xBFFF)]
    #[test_case(0xC000, 0xDFFF)]
    #[test_case(0xE000, 0xFFFF)]
    pub fn register_address_ranges(start: u16, end: u16) {
        let mut a = mapper(8, 8);
        let mut b = mapper(8, 8);

        // CHR in 4KB mode, so every register has a visible effect
        write_reg(&mut a, CTRL, 0x1C);
        write_reg(&mut b, CTRL, 0x1C);
        write_reg(&mut a, start, 0x13);
        write_reg(&mut b, end, 0x13);

        for addr in [0x0000u16, 0x1000] {
            assert_eq!(a.fetch_ppu(addr), b.fetch_ppu(addr));
        }
        for addr in [0x8000u16, 0xC000] {
            assert_eq!(a.fetch_cpu(addr), b.fetch_cpu(addr));
        }
        assert_eq!(a.mirroring_mode(), b.mirroring_mode());
    }

    #[test]
    pub fn reset_bit_clears_shift_register() {
        let mut mapper = mapper(8, 4);

        mapper.store_cpu(PRG, 1);
        mapper.store_cpu(PRG, 1);
        mapper.store_cpu(PRG, 0x80);
        write_reg(&mut mapper, PRG, 2);
        assert_eq!(mapper.fetch_cpu(0x8000), prg_byte(2, 0));
    }

    #[test]
    pub fn reset_bit_sets_prg_mode_3_keeping_other_bits() {
        let mut mapper = mapper(8, 4);

        // vertical mirroring, PRG mode 0 (32KB), CHR 4KB
        write_reg(&mut mapper, CTRL, 0x12);
        write_reg(&mut mapper, PRG, 2);
        assert_eq!(mapper.fetch_cpu(0xC000), prg_byte(3, 0));

        mapper.store_cpu(0x8000, 0x80);
        assert_eq!(mapper.fetch_cpu(0x8000), prg_byte(2, 0));
        assert_eq!(mapper.fetch_cpu(0xC000), prg_byte(7, 0));
        assert_eq!(mapper.mirroring_mode(), MirroringType::Vertical);
    }

    #[test_case(0x00, 2, 2, 3; "mode 0")]
    #[test_case(0x04, 2, 2, 3; "mode 1")]
    #[test_case(0x00, 3, 2, 3; "mode 0 ignores low bit")]
    #[test_case(0x04, 7, 6, 7; "mode 1 ignores low bit")]
    #[test_case(0x08, 3, 0, 3; "mode 2")]
    #[test_case(0x08, 6, 0, 6; "mode 2 other bank")]
    #[test_case(0x0C, 2, 2, 7; "mode 3")]
    #[test_case(0x0C, 5, 5, 7; "mode 3 other bank")]
    pub fn prg_banking(ctrl: u8, prg: u8, low: usize, high: usize) {
        let mut mapper = mapper(8, 4);

        write_reg(&mut mapper, CTRL, ctrl);
        write_reg(&mut mapper, PRG, prg);
        for offset in [0usize, 1, 0x1234, LAST] {
            assert_eq!(
                mapper.fetch_cpu(0x8000 + offset as u16),
                prg_byte(low, offset)
            );
            assert_eq!(
                mapper.fetch_cpu(0xC000 + offset as u16),
                prg_byte(high, offset)
            );
        }
    }

    #[test_case(0x0C, 6, 2, 3; "mode 3 wraps")]
    #[test_case(0x08, 5, 0, 1; "mode 2 wraps")]
    #[test_case(0x00, 6, 2, 3; "mode 0 wraps")]
    pub fn prg_bank_wraps_around(ctrl: u8, prg: u8, low: usize, high: usize) {
        let mut mapper = mapper(4, 4);

        write_reg(&mut mapper, CTRL, ctrl);
        write_reg(&mut mapper, PRG, prg);
        assert_eq!(mapper.fetch_cpu(0x8000), prg_byte(low, 0));
        assert_eq!(mapper.fetch_cpu(0xC000), prg_byte(high, 0));
    }

    #[test]
    pub fn chr_8kb_mode() {
        let mut mapper = mapper(8, 4);

        write_reg(&mut mapper, CTRL, 0x0C);
        for (chr_0, bank) in [(0u8, 0usize), (1, 0), (2, 1), (3, 1), (6, 3)] {
            write_reg(&mut mapper, CHR_0, chr_0);
            // CHR 1 is ignored in 8KB mode
            write_reg(&mut mapper, CHR_1, 4);
            for offset in [0usize, 0x0FFF, 0x1000, 0x1FFF] {
                assert_eq!(mapper.fetch_ppu(offset as u16), chr_byte(bank, offset));
            }
        }
    }

    #[test_case(0, 0)]
    #[test_case(3, 4)]
    #[test_case(4, 3)]
    #[test_case(7, 7)]
    pub fn chr_4kb_mode(chr_0: u8, chr_1: u8) {
        let mut mapper = mapper(8, 4);

        write_reg(&mut mapper, CTRL, 0x1C);
        write_reg(&mut mapper, CHR_0, chr_0);
        write_reg(&mut mapper, CHR_1, chr_1);

        let expected = |bank_4kb: u8, offset: usize| {
            chr_byte(
                bank_4kb as usize / 2,
                (bank_4kb as usize % 2) * 0x1000 + offset,
            )
        };
        for offset in [0usize, 1, 0x0FFF] {
            assert_eq!(mapper.fetch_ppu(offset as u16), expected(chr_0, offset));
            assert_eq!(
                mapper.fetch_ppu(0x1000 + offset as u16),
                expected(chr_1, offset)
            );
        }
    }

    #[test]
    pub fn chr_bank_wraps_around() {
        let mut mapper = mapper(8, 2);

        write_reg(&mut mapper, CTRL, 0x0C);
        write_reg(&mut mapper, CHR_0, 6);
        assert_eq!(mapper.fetch_ppu(0), chr_byte(1, 0));

        write_reg(&mut mapper, CTRL, 0x1C);
        write_reg(&mut mapper, CHR_0, 5);
        write_reg(&mut mapper, CHR_1, 7);
        // 4KB bank 5 -> 8KB bank 2 (wraps to 0), upper half
        assert_eq!(mapper.fetch_ppu(0x0000), chr_byte(0, 0x1000));
        // 4KB bank 7 -> 8KB bank 3 (wraps to 1), upper half
        assert_eq!(mapper.fetch_ppu(0x1000), chr_byte(1, 0x1000));
    }

    #[test]
    pub fn chr_rom_is_read_only() {
        let mut mapper = mapper(8, 4);

        for addr in [0x0000u16, 0x1000, 0x1FFF] {
            let before = mapper.fetch_ppu(addr);
            mapper.store_ppu(addr, !before);
            assert_eq!(mapper.fetch_ppu(addr), before);
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

    #[test]
    pub fn chr_ram_4kb_banking() {
        let mut mapper = mapper(8, 0);

        write_reg(&mut mapper, CTRL, 0x1C);
        // both windows point to the upper 4KB of the 8KB CHR RAM
        write_reg(&mut mapper, CHR_0, 1);
        write_reg(&mut mapper, CHR_1, 1);
        mapper.store_ppu(0x0010, 0x42);
        assert_eq!(mapper.fetch_ppu(0x1010), 0x42);

        // swapped windows
        write_reg(&mut mapper, CHR_0, 0);
        write_reg(&mut mapper, CHR_1, 1);
        mapper.store_ppu(0x0020, 0x24);
        assert_eq!(mapper.fetch_ppu(0x0020), 0x24);
        assert_eq!(mapper.fetch_ppu(0x1010), 0x42);
        write_reg(&mut mapper, CHR_0, 1);
        write_reg(&mut mapper, CHR_1, 0);
        assert_eq!(mapper.fetch_ppu(0x0010), 0x42);
        assert_eq!(mapper.fetch_ppu(0x1020), 0x24);
    }

    #[test_case(0x2000)]
    #[test_case(0x3FFF)]
    pub fn ppu_outside_pattern_tables(addr: u16) {
        let mut ram_mapper = mapper(8, 0);
        ram_mapper.store_ppu(addr, 0xAB);
        assert_eq!(ram_mapper.fetch_ppu(addr), 0);
        assert_eq!(ram_mapper.fetch_ppu(addr & 0x1FFF), 0);

        assert_eq!(mapper(8, 4).fetch_ppu(addr), 0);
    }

    #[test]
    pub fn prg_ram_read_write() {
        let mut mapper = mapper(8, 4);

        for addr in [0x6000u16, 0x6001, 0x7000, 0x7FFF] {
            assert_eq!(mapper.fetch_cpu(addr), 0);
            mapper.store_cpu(addr, (addr >> 4) as u8);
        }
        for addr in [0x6000u16, 0x6001, 0x7000, 0x7FFF] {
            assert_eq!(mapper.fetch_cpu(addr), (addr >> 4) as u8);
        }
    }

    #[test]
    pub fn prg_ram_writes_do_not_touch_shift_register() {
        let mut mapper = mapper(8, 4);

        mapper.store_cpu(PRG, 1);
        mapper.store_cpu(0x6000, 0xFF);
        mapper.store_cpu(0x6001, 0x80);
        for i in 1..5 {
            mapper.store_cpu(PRG, (1 >> i) & 1);
        }
        assert_eq!(mapper.fetch_cpu(0x8000), prg_byte(1, 0));
    }

    #[test]
    pub fn prg_ram_disable() {
        let mut mapper = mapper(8, 4);

        mapper.store_cpu(0x6000, 0x42);

        write_reg(&mut mapper, PRG, 0x10);
        assert_eq!(mapper.fetch_cpu(0x6000), 0);
        mapper.store_cpu(0x6000, 0x24);
        mapper.store_cpu(0x6001, 0x24);

        write_reg(&mut mapper, PRG, 0x00);
        assert_eq!(mapper.fetch_cpu(0x6000), 0x42);
        assert_eq!(mapper.fetch_cpu(0x6001), 0);
    }

    #[test]
    pub fn prg_ram_disable_bit_does_not_select_bank() {
        let mut mapper = mapper(8, 4);

        write_reg(&mut mapper, PRG, 0x13);
        assert_eq!(mapper.fetch_cpu(0x8000), prg_byte(3, 0));
        assert_eq!(mapper.fetch_cpu(0xC000), prg_byte(7, 0));
    }

    #[test]
    pub fn store_below_prg_ram_is_ignored() {
        let mut mapper = mapper(8, 4);

        for _ in 0..4 {
            mapper.store_cpu(0x5000, 1);
        }
        write_reg(&mut mapper, PRG, 2);
        assert_eq!(mapper.fetch_cpu(0x8000), prg_byte(2, 0));
    }

    #[test]
    pub fn reset_restores_power_on_state() {
        let mut mapper = mapper(8, 4);

        write_reg(&mut mapper, CTRL, 0x13);
        write_reg(&mut mapper, CHR_0, 3);
        write_reg(&mut mapper, CHR_1, 5);
        write_reg(&mut mapper, PRG, 2);
        mapper.store_cpu(PRG, 1);

        mapper.reset();
        assert_eq!(mapper.fetch_cpu(0x8000), prg_byte(0, 0));
        assert_eq!(mapper.fetch_cpu(0xC000), prg_byte(7, 0));
        assert_eq!(mapper.fetch_ppu(0x0000), chr_byte(0, 0));
        assert_eq!(mapper.fetch_ppu(0x1000), chr_byte(0, 0x1000));
        assert_eq!(mapper.mirroring_mode(), MirroringType::SingleScreenLower);

        // pending shift register bits are discarded
        write_reg(&mut mapper, PRG, 4);
        assert_eq!(mapper.fetch_cpu(0x8000), prg_byte(4, 0));
    }

    #[test]
    pub fn trainer_is_skipped() {
        let mapper = Mapper1::new(&header(4, 2, 0, FLAG_TRAINER), build_rom(4, 2, true)).unwrap();

        assert_eq!(mapper.fetch_cpu(0x8000), prg_byte(0, 0));
        assert_eq!(mapper.fetch_cpu(0xFFFF), prg_byte(3, LAST));
        assert_eq!(mapper.fetch_ppu(0x0000), chr_byte(0, 0));
        assert_eq!(mapper.fetch_ppu(0x1FFF), chr_byte(0, 0x1FFF));
    }
}
