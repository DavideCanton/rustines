use rustines_macro::Named;

use crate::arch::mappers::common::{ChrRomBank, PrgRamBank, PrgRomBank, get_rom_banks};
use crate::arch::mappers::mapper::Mapper;
use crate::arch::rom_structs::{INesHeader, MirroringType, PRG_RAM_BANK_SIZE, TRAINER_SIZE};
use crate::utils::bit_utils::{BitIndex, BitManip};
use crate::utils::named::Named;
use bitfield::bitfield;

const CHR_RAM_SIZE: usize = 8192;

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
    prg_rom: Vec<PrgRomBank>,
    prg_ram: Vec<PrgRamBank>,
    chr_rom: Option<Vec<ChrRomBank>>,
    chr_ram: Option<[u8; CHR_RAM_SIZE]>,
    shift_register: u8,
    count: u8,
    ctrl_register: ControlRegister,
    chr_1_reg: u8,
    chr_2_reg: u8,
    prg_reg: u8,
}

impl Mapper1 {
    pub fn new(header: &INesHeader, mut rom: Vec<u8>) -> anyhow::Result<Self> {
        if header.has_trainer() {
            let _ = rom.drain(0..TRAINER_SIZE).collect::<Vec<_>>();
        }

        let prg_rom = get_rom_banks(&mut rom, header.prg_rom_size());
        let prg_ram = (0..header.prg_ram_banks.max(1))
            .map(|_| [0; PRG_RAM_BANK_SIZE])
            .collect();

        let (chr_rom, chr_ram) = if header.uses_chr_ram() {
            (None, Some([0; CHR_RAM_SIZE]))
        } else {
            (Some(get_rom_banks(&mut rom, header.chr_rom_size())), None)
        };

        Ok(Mapper1 {
            prg_rom,
            prg_ram,
            chr_rom,
            chr_ram,
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
        if (0x6000..=0x7FFF).contains(&addr) {
            if self.prg_ram_enabled() {
                // TODO improve
                return self.prg_ram[0][(addr & 0x1FFF) as usize];
            }
            return 0;
        }

        if addr >= 0x8000 {
            let num_banks = self.prg_rom.len();

            let (bank, offset) = match self.ctrl_register.prg_bank_mode() {
                0 | 1 => {
                    let base_bank = (self.prg_reg & 0x0E) as usize;
                    if addr < 0xC000 {
                        (base_bank, (addr & 0x3FFF) as usize)
                    } else {
                        (base_bank + 1, (addr & 0x3FFF) as usize)
                    }
                }

                2 => {
                    if addr < 0xC000 {
                        (0, (addr & 0x3FFF) as usize)
                    } else {
                        ((self.prg_reg & 0x0F) as usize, (addr & 0x3FFF) as usize)
                    }
                }

                3 => {
                    if addr < 0xC000 {
                        ((self.prg_reg & 0x0F) as usize, (addr & 0x3FFF) as usize)
                    } else {
                        (num_banks - 1, (addr & 0x3FFF) as usize)
                    }
                }

                _ => unreachable!(),
            };

            let safe_bank = bank % num_banks;
            return self.prg_rom[safe_bank][offset];
        }

        0
    }

    fn fetch_ppu(&self, addr: u16) -> u8 {
        if addr <= 0x1FFF {
            if let Some(chr_ram) = self.chr_ram {
                chr_ram[addr as usize]
            } else {
                let (index, offset) = if self.ctrl_register.chr_bank_mode() {
                    let index = match addr {
                        0x0000..=0x0FFF => self.chr_1_reg,
                        0x1000..=0x1FFF => self.chr_2_reg,
                        _ => unreachable!(),
                    };
                    let index = index & 0x1F;
                    let bank_8kb = (index >> 1) as usize;
                    let sub_offset = ((index & 0x01) as usize * 0x1000) + (addr & 0x0FFF) as usize;
                    (bank_8kb, sub_offset)
                } else {
                    let bank_8kb = ((self.chr_1_reg & 0x1E) >> 1) as usize;
                    let offset_8kb = (addr & 0x1FFF) as usize;
                    (bank_8kb, offset_8kb)
                };

                let rom_banks = self.chr_rom.as_ref().unwrap();
                let safe_index = index % rom_banks.len();

                self.chr_rom.as_ref().unwrap()[safe_index][offset]
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

    fn store_cpu(&mut self, addr: u16, value: u8) {
        if (0x6000..=0x7FFF).contains(&addr) {
            if self.prg_ram_enabled() {
                // TODO improve
                self.prg_ram[0][(addr & 0x1FFF) as usize] = value;
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
}
