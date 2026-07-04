use crate::arch::mappers::mapper::MapperBox;

use crate::arch::common::replace;
use crate::arch::ppu::Ppu;

pub trait FetchStore {
    fn fetch(&mut self, addr: u16) -> u8;
    fn store(&mut self, addr: u16, val: u8) -> u8;

    fn fetch_many(&mut self, addr: u16, destination: &mut [u8]) {
        for (addr, (_, v)) in (addr..).zip(destination.iter_mut().enumerate()) {
            *v = self.fetch(addr);
        }
    }

    fn store_many(&mut self, addr: u16, values: &[u8]) {
        for (addr, (_, v)) in (addr..).zip(values.iter().enumerate()) {
            self.store(addr, *v);
        }
    }
}

pub struct Bus {
    nes_ram: [u8; 2048],
    ppu: Ppu,
    apu_registers: [u8; 24],
    mapper: MapperBox,
}

impl Bus {
    pub fn new(mapper: MapperBox, ppu: Ppu) -> Self {
        Self {
            nes_ram: [0; 2048],
            ppu,
            apu_registers: [0; 24],
            mapper,
        }
    }

    pub fn push8(&mut self, sp: u8, val: u8) {
        let sp = sp as u16 + 0x0100;
        self.store(sp, val);
    }

    pub fn peek8(&mut self, sp: u8) -> u8 {
        let sp = sp as u16 + 0x0100;
        self.fetch(sp)
    }

    pub fn ppu(&self) -> &Ppu {
        &self.ppu
    }

    pub fn ppu_tick(&mut self) {
        let mapper = self.mapper.as_mut();
        self.ppu.tick(mapper);
    }
}

impl FetchStore for Bus {
    fn fetch(&mut self, address: u16) -> u8 {
        if address <= 0x1FFF {
            let ind = address & 0x07FF;
            self.nes_ram[ind as usize]
        } else if address <= 0x3FFF {
            let ind = address & 0x0007;
            self.ppu.cpu_read(ind)
        } else if address <= 0x4017 {
            // TODO callback here probably
            let ind = address & 0xFF;
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
            self.ppu.cpu_write(ind, val);
            0
        } else if address <= 0x4017 {
            // TODO callback here probably
            let ind = address & 0xFF;
            replace(&mut self.apu_registers, ind as usize, val)
        } else if address <= 0x401F {
            // ignored
            0
        } else {
            self.mapper.store_chr_rom(address, val)
        }
    }
}
