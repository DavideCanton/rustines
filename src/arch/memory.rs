use crate::utils::bit_utils::to_u16;

use super::rom_structs::NesRom;

pub struct Memory {
    mem: Vec<u8>,
}

macro_rules! switch_addr {
    ( $self:ident, $addr:expr, $fn_ram:expr, $fn_ppu:expr, $fn_io:expr, $fn_exp:expr, $fn_sram:expr, $fn_prg:expr ) => {{
        if $self._is_ram_addr($addr) {
            $fn_ram
        } else if $self._is_ppu_register($addr) {
            $fn_ppu
        } else if $self._is_io_register($addr) {
            $fn_io
        } else if $self._is_exp_rom($addr) {
            $fn_exp
        } else if $self._is_sram($addr) {
            $fn_sram
        } else if $self._is_prg_rom($addr) {
            $fn_prg
        } else {
            0
        }
    }};
}

impl Memory {
    pub fn new(rom: NesRom) -> Self {
        let mut mem = vec![0; 0x10000];
        unsafe {
            let ptr = mem.as_mut_ptr();
            if rom.prg_rom_banks.len() == 1 {
                // 1 prg rom of 32k
                let prg = &rom.prg_rom_banks[0];
                std::ptr::copy_nonoverlapping(prg.data.as_ptr(), ptr.offset(0x8000), 0x8000);
            } else {
                // 2 prg rom of 16k
                let prg = &rom.prg_rom_banks[0];
                std::ptr::copy_nonoverlapping(prg.data.as_ptr(), ptr.offset(0x8000), 0x4000);
                let prg = &rom.prg_rom_banks[1];
                std::ptr::copy_nonoverlapping(prg.data.as_ptr(), ptr.offset(0xC000), 0x4000);
            }
        }
        Memory { mem }
    }

    pub fn fetch(&self, addr: u16) -> u8 {
        let addr = addr as usize;

        switch_addr!(
            self,
            addr,
            self._fetch_from_ram(addr),
            self._fetch_from_ppu_register(addr),
            self._fetch_from_io_register(addr),
            self._fetch_from_exp_rom(addr),
            self._fetch_from_sram(addr),
            self._fetch_from_prg_rom(addr)
        )
    }

    pub fn store(&mut self, addr: u16, val: u8) -> u8 {
        let addr = addr as usize;

        switch_addr!(
            self,
            addr,
            self._store_ram(addr, val),
            self._store_ppu_register(addr, val),
            self._store_io_register(addr, val),
            self._store_exp_rom(addr, val),
            self._store_sram(addr, val),
            self._store_prg_rom(addr, val)
        )
    }

    fn _is_ram_addr(&self, addr: usize) -> bool {
        addr <= 0x1FFF
    }

    fn _fetch_from_ram(&self, addr: usize) -> u8 {
        let addr = addr & 0x7FF;
        self.mem[addr]
    }

    fn _store_ram(&mut self, addr: usize, val: u8) -> u8 {
        let addr = addr & 0x7FF;
        let old = self.mem[addr];
        self.mem[addr] = val;
        old
    }

    fn _is_ppu_register(&self, addr: usize) -> bool {
        addr <= 0x3FFF
    }

    fn _fetch_from_ppu_register(&self, addr: usize) -> u8 {
        let addr = addr & 0x7;
        0
    }

    fn _store_ppu_register(&mut self, addr: usize, val: u8) -> u8 {
        // let addr = addr & 0x7;
        unimplemented!();
    }

    fn _is_io_register(&self, addr: usize) -> bool {
        addr <= 0x401F
    }

    fn _fetch_from_io_register(&self, addr: usize) -> u8 {
        unimplemented!();
    }

    fn _store_io_register(&mut self, addr: usize, val: u8) -> u8 {
        unimplemented!();
    }

    fn _is_exp_rom(&self, addr: usize) -> bool {
        addr <= 0x6000
    }

    fn _fetch_from_exp_rom(&self, addr: usize) -> u8 {
        unimplemented!();
    }

    fn _store_exp_rom(&mut self, addr: usize, val: u8) -> u8 {
        unimplemented!();
    }

    fn _is_sram(&self, addr: usize) -> bool {
        addr <= 0x8000
    }

    fn _fetch_from_sram(&self, addr: usize) -> u8 {
        unimplemented!();
    }

    fn _store_sram(&mut self, addr: usize, val: u8) -> u8 {
        unimplemented!();
    }

    fn _is_prg_rom(&self, addr: usize) -> bool {
        addr <= 0xFFFF
    }

    fn _fetch_from_prg_rom(&self, addr: usize) -> u8 {
        unimplemented!();
    }

    fn _store_prg_rom(&mut self, addr: usize, val: u8) -> u8 {
        unimplemented!();
    }

    pub fn push8(&mut self, sp: u8, val: u8) {
        let sp = sp as u16 + 0x0100;
        self.store(sp, val);
    }

    pub fn peek8(&self, sp: u8) -> u8 {
        let sp = sp as u16 + 0x0100;
        self.fetch(sp)
    }

    pub fn get_reset(&self) -> u16 {
        to_u16(self.mem[0xFFFC], self.mem[0xFFFD])
    }
}
