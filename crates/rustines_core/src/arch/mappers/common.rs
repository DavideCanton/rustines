use rustines_macro::Memento;

use crate::{
    MementoWriter,
    arch::{
        memento::{MementoReadAndFill, MementoReader, MementoResult, MementoWriteable},
        rom_structs::{INesHeader, TRAINER_SIZE},
    },
};

const CHR_RAM_SIZE: usize = 0x2000;

/// Linear memory (ROM or RAM) accessed through banks of arbitrary size.
///
/// Every access wraps around the memory size, so out of range banks are mirrored like on
/// real hardware (e.g. a 16KB PRG ROM seen through a 32KB window).
#[derive(Debug, Clone, Memento)]
pub struct BankedMemory {
    #[memento]
    data: Vec<u8>,
    /// `len - 1` when the size is a power of two, so wrapping is a cheap mask.
    #[memento]
    mask: Option<usize>,
}

impl BankedMemory {
    pub fn new(data: Vec<u8>) -> Self {
        assert!(!data.is_empty(), "BankedMemory can't be empty");
        let mask = data.len().is_power_of_two().then(|| data.len() - 1);
        BankedMemory { data, mask }
    }

    pub fn zeroed(size: usize) -> Self {
        Self::new(vec![0; size])
    }

    /// Takes the first `size` bytes out of `rom`.
    pub fn from_rom(rom: &mut Vec<u8>, size: usize) -> Self {
        Self::new(rom.drain(0..size).collect())
    }

    /// Number of banks of the given size.
    pub fn banks(&self, bank_size: usize) -> usize {
        self.data.len() / bank_size
    }

    #[inline(always)]
    fn wrap(&self, offset: usize) -> usize {
        match self.mask {
            Some(mask) => offset & mask,
            None => offset % self.data.len(),
        }
    }

    #[inline(always)]
    fn banked_offset(bank: usize, bank_size: usize, addr: u16) -> usize {
        debug_assert!(bank_size.is_power_of_two());
        bank * bank_size + (addr as usize & (bank_size - 1))
    }

    #[inline(always)]
    pub fn read(&self, offset: usize) -> u8 {
        self.data[self.wrap(offset)]
    }

    #[inline(always)]
    pub fn write(&mut self, offset: usize, val: u8) {
        let offset = self.wrap(offset);
        self.data[offset] = val;
    }

    /// Reads `addr` (only the bits inside the bank are used) from bank `bank`.
    #[inline(always)]
    pub fn read_banked(&self, bank: usize, bank_size: usize, addr: u16) -> u8 {
        self.read(Self::banked_offset(bank, bank_size, addr))
    }

    #[inline(always)]
    pub fn write_banked(&mut self, bank: usize, bank_size: usize, addr: u16, val: u8) {
        self.write(Self::banked_offset(bank, bank_size, addr), val)
    }

    pub fn memory(&self) -> &[u8] {
        &self.data
    }

    pub(crate) fn write_all(&mut self, content: &[u8]) {
        self.data.copy_from_slice(content);
    }
}

/// CHR memory: CHR ROM from the cartridge, or 8KB of CHR RAM when the header declares no
/// CHR ROM. Writes are ignored for CHR ROM.
#[derive(Debug, Clone)]
pub struct ChrMemory {
    memory: BankedMemory,
    is_ram: bool,
}

impl ChrMemory {
    pub fn new(header: &INesHeader, rom: &mut Vec<u8>) -> Self {
        if header.uses_chr_ram() {
            ChrMemory {
                memory: BankedMemory::zeroed(CHR_RAM_SIZE),
                is_ram: true,
            }
        } else {
            ChrMemory {
                memory: BankedMemory::from_rom(rom, header.chr_rom_size()),
                is_ram: false,
            }
        }
    }

    #[inline(always)]
    pub fn read_banked(&self, bank: usize, bank_size: usize, addr: u16) -> u8 {
        self.memory.read_banked(bank, bank_size, addr)
    }

    #[inline(always)]
    pub fn write_banked(&mut self, bank: usize, bank_size: usize, addr: u16, val: u8) {
        if self.is_ram {
            self.memory.write_banked(bank, bank_size, addr, val)
        }
    }
}

impl MementoReadAndFill for ChrMemory {
    fn read_and_fill(&mut self, data: &mut MementoReader) -> MementoResult {
        if self.is_ram {
            self.memory.read_and_fill(data)?;
        }
        Ok(())
    }
}

impl MementoWriteable for ChrMemory {
    fn write_memento(&self, writer: &mut dyn MementoWriter) -> MementoResult {
        if self.is_ram {
            self.memory.write_memento(writer)?;
        }
        Ok(())
    }
}

/// Removes the trainer, if any, from the beginning of the ROM.
pub fn skip_trainer(header: &INesHeader, rom: &mut Vec<u8>) {
    if header.has_trainer() {
        rom.drain(0..TRAINER_SIZE);
    }
}

#[cfg(test)]
pub(crate) mod test_utils {
    use crate::arch::rom_structs::{
        CHR_ROM_BANK_SIZE, HeaderFlags, INesHeader, PRG_ROM_BANK_SIZE, TRAINER_SIZE,
    };

    pub const FLAG_VERTICAL: u16 = 1 << 0;
    pub const FLAG_TRAINER: u16 = 1 << 2;

    pub fn header(
        prg_rom_banks: u8,
        chr_rom_banks: u8,
        prg_ram_banks: u8,
        flags: u16,
    ) -> INesHeader {
        INesHeader {
            header: [0; 4],
            flags: HeaderFlags(flags),
            chr_rom_banks,
            flags_9: 0,
            flags_10: 0,
            prg_rom_banks,
            prg_ram_banks,
            _padding: [0; 5],
        }
    }

    /// Expected PRG byte: the high nibble identifies the 16KB bank, the low nibble the offset.
    pub fn prg_byte(bank: usize, offset: usize) -> u8 {
        ((bank << 4) | (offset & 0x0F)) as u8
    }

    /// Expected CHR byte for an 8KB bank and an offset in `0..0x2000`. Each 4KB half has a
    /// distinct value, so 4KB banking can be verified too.
    pub fn chr_byte(bank: usize, offset: usize) -> u8 {
        let bank_4kb = bank * 2 + offset / 0x1000;
        (0x80 ^ (bank_4kb << 3) ^ (offset & 0x07)) as u8
    }

    /// Builds a ROM (without the iNES header) with recognizable PRG and CHR patterns, optionally
    /// preceded by a trainer filled with garbage.
    pub fn build_rom(prg_banks: usize, chr_banks: usize, trainer: bool) -> Vec<u8> {
        let mut rom = Vec::new();
        if trainer {
            rom.extend(std::iter::repeat_n(0xEE, TRAINER_SIZE));
        }
        for bank in 0..prg_banks {
            rom.extend((0..PRG_ROM_BANK_SIZE).map(|i| prg_byte(bank, i)));
        }
        for bank in 0..chr_banks {
            rom.extend((0..CHR_ROM_BANK_SIZE).map(|i| chr_byte(bank, i)));
        }
        rom
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::{BankedMemory, ChrMemory, test_utils::header};

    fn memory(size: usize) -> BankedMemory {
        BankedMemory::new((0..size).map(|i| i as u8).collect())
    }

    #[test_case(0x100)]
    #[test_case(0x0C0)]
    fn read_wraps_around(size: usize) {
        let memory = memory(size);

        assert_eq!(memory.read(0), 0);
        assert_eq!(memory.read(size - 1), (size - 1) as u8);
        assert_eq!(memory.read(size), 0);
        assert_eq!(memory.read(size * 3 + 5), 5);
    }

    #[test_case(0x100)]
    #[test_case(0x0C0)]
    fn write_wraps_around(size: usize) {
        let mut memory = BankedMemory::zeroed(size);

        memory.write(size + 7, 0xAB);
        assert_eq!(memory.read(7), 0xAB);
    }

    #[test]
    fn read_banked() {
        let memory = memory(0x100);

        // only the bits inside the bank are taken from the address
        assert_eq!(memory.read_banked(0, 0x40, 0x8005), 0x05);
        assert_eq!(memory.read_banked(2, 0x40, 0x8005), 0x85);
        assert_eq!(memory.read_banked(3, 0x40, 0xFFFF), 0xFF);
        // out of range banks wrap around
        assert_eq!(memory.read_banked(5, 0x40, 0x0001), 0x41);
    }

    #[test]
    fn write_banked() {
        let mut memory = BankedMemory::zeroed(0x100);

        memory.write_banked(1, 0x80, 0x1234, 0xCD);
        assert_eq!(memory.read(0x80 + 0x34), 0xCD);
    }

    #[test_case(0x40, 4)]
    #[test_case(0x80, 2)]
    #[test_case(0x100, 1)]
    fn banks(bank_size: usize, expected: usize) {
        assert_eq!(memory(0x100).banks(bank_size), expected);
    }

    #[test]
    #[should_panic(expected = "BankedMemory can't be empty")]
    fn empty_memory() {
        BankedMemory::new(vec![]);
    }

    #[test]
    fn chr_rom_is_read_only() {
        let mut rom = vec![0x11; 0x2000];
        let mut chr = ChrMemory::new(&header(1, 1, 0, 0), &mut rom);

        chr.write_banked(0, 0x2000, 0x10, 0x22);
        assert_eq!(chr.read_banked(0, 0x2000, 0x10), 0x11);
        assert!(rom.is_empty());
    }

    #[test]
    fn chr_ram_is_writable() {
        let mut rom = vec![0x11; 0x10];
        let mut chr = ChrMemory::new(&header(1, 0, 0, 0), &mut rom);

        assert_eq!(chr.read_banked(0, 0x2000, 0x1FFF), 0);
        chr.write_banked(0, 0x2000, 0x1FFF, 0x22);
        assert_eq!(chr.read_banked(0, 0x2000, 0x1FFF), 0x22);
        // CHR RAM doesn't consume the ROM
        assert_eq!(rom.len(), 0x10);
    }
}
