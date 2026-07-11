use crate::arch::controller::NesController;
use crate::arch::mappers::mapper::{Mapper, MapperBox};

use crate::arch::common::replace;
use crate::arch::ppu::Ppu;

pub trait FetchStore {
    fn fetch(&mut self, addr: u16) -> u8;
    fn store(&mut self, addr: u16, val: u8);

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
    mapper: MapperBox,
    controller1: NesController,
    controller2: NesController,
}

impl Bus {
    pub fn new(mapper: MapperBox, ppu: Ppu) -> Self {
        Self {
            nes_ram: [0; 2048],
            ppu,
            mapper,
            controller1: NesController::new(),
            controller2: NesController::new(),
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

    pub fn ppu_mut(&mut self) -> &mut Ppu {
        &mut self.ppu
    }

    pub fn mapper(&self) -> &dyn Mapper {
        self.mapper.as_ref()
    }

    pub fn ppu_tick(&mut self) {
        let mapper = self.mapper.as_mut();
        self.ppu.tick(mapper);
    }

    pub fn controller1_mut(&mut self) -> &mut NesController {
        &mut self.controller1
    }

    pub fn controller2_mut(&mut self) -> &mut NesController {
        &mut self.controller2
    }
}

impl FetchStore for Bus {
    fn fetch(&mut self, address: u16) -> u8 {
        if address <= 0x1FFF {
            let ind = address & 0x07FF;
            self.nes_ram[ind as usize]
        } else if address <= 0x3FFF {
            let ind = address & 0x0007;
            self.ppu.cpu_read(ind, self.mapper.as_ref())
        } else if address <= 0x4017 {
            if address == 0x4016 {
                self.controller1.read()
            } else if address == 0x4017 {
                self.controller2.read()
            } else {
                // TODO APU
                0
            }
        } else if address <= 0x401F {
            0
        } else if address <= 0x7FFF {
            self.mapper.fetch_prg_ram(address)
        } else {
            self.mapper.fetch_prg_rom(address)
        }
    }

    fn store(&mut self, address: u16, val: u8) {
        if address <= 0x1FFF {
            let ind = address & 0x07FF;
            replace(&mut self.nes_ram, ind as usize, val);
        } else if address <= 0x3FFF {
            let ind = address & 0x0007;
            self.ppu.cpu_write(ind as u8, val, self.mapper.as_ref());
        } else if address <= 0x4017 {
            if address == 0x4016 {
                self.controller1.write(val);
            } else if address == 0x4017 {
                self.controller2.write(val);
            } else {
                // TODO stall?
                let mut buf = vec![0; 256];
                let start = (val as u16) << 8;
                self.fetch_many(start, &mut buf);
                self.ppu_mut().dma_copy(&buf);
            }
        } else if address <= 0x401F {
        } else if address <= 0x7FFF {
            self.mapper.store_prg_ram(address, val);
        } else {
            self.mapper.store_prg_rom(address, val);
        }
    }
}
