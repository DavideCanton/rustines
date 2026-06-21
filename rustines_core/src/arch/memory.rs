use crate::arch::mappers::mapper::MapperBox;

use crate::arch::common::replace;

pub trait FetchStore {
    fn fetch(&self, addr: u16) -> u8;
    fn store(&mut self, addr: u16, val: u8) -> u8;

    fn fetch_many(&self, addr: u16, destination: &mut [u8]) {
        for (addr, (i, v)) in (addr..).zip(destination.iter_mut().enumerate()) {
            *v = self.fetch(addr);
        }
    }

    fn store_many(&mut self, addr: u16, values: &[u8]) {
        for (addr, (i, v)) in (addr..).zip(values.iter().enumerate()) {
            self.store(addr, *v);
        }
    }
}

pub struct Memory {
    nes_ram: [u8; 2048],
    ppu_registers: [u8; 8],
    apu_registers: [u8; 24],
    mapper: MapperBox,
}

impl Memory {
    pub fn new(mapper: MapperBox) -> Self {
        Self {
            nes_ram: [0; 2048],
            ppu_registers: [0; 8],
            apu_registers: [0; 24],
            mapper,
        }
    }

    pub fn push8(&mut self, sp: u8, val: u8) {
        let sp = sp as u16 + 0x0100;
        self.store(sp, val);
    }

    pub fn peek8(&self, sp: u8) -> u8 {
        let sp = sp as u16 + 0x0100;
        self.fetch(sp)
    }
}

impl FetchStore for Memory {
    fn fetch(&self, address: u16) -> u8 {
        if address <= 0x1FFF {
            let ind = address & 0x07FF;
            self.nes_ram[ind as usize]
        } else if address <= 0x3FFF {
            let ind = address & 0x0007;
            self.ppu_registers[ind as usize]
        } else if address <= 0x4017 {
            // TODO callback here probably
            let ind = address;
            self.apu_registers[ind as usize]
        } else if address <= 0x401F {
            // ignored
            0
        } else {
            self.mapper.fetch_prg_rom(address)
        }
    }

    fn store(&mut self, address: u16, val: u8) -> u8 {
        if address <= 0x1FFF {
            let ind = address & 0x07FF;
            replace(&mut self.nes_ram, ind as usize, val)
        } else if address <= 0x3FFF {
            let ind = address & 0x0007;
            replace(&mut self.ppu_registers, ind as usize, val)
        } else if address <= 0x4017 {
            // TODO callback here probably
            let ind = address;
            replace(&mut self.apu_registers, ind as usize, val)
        } else if address <= 0x401F {
            // ignored
            0
        } else {
            self.mapper.store_chr_rom(address, val)
        }
    }
}
