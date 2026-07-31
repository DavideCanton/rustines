use crate::arch::controller::NesController;
use crate::arch::mappers::mapper::{Mapper, MapperBox};

use crate::arch::common::replace;
use crate::arch::ppu::Ppu;

pub struct Bus {
    nes_ram: [u8; 2048],
    ppu: Ppu,
    mapper: MapperBox,
    controller1: NesController,
    controller2: NesController,
    open_bus_value: u8,
}

impl Bus {
    pub fn new(mapper: MapperBox, ppu: Ppu) -> Self {
        Self {
            nes_ram: [0; 2048],
            ppu,
            mapper,
            controller1: NesController::new(1),
            controller2: NesController::new(2),
            open_bus_value: 0,
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

    pub fn open_bus_value(&self) -> u8 {
        self.open_bus_value
    }

    pub fn fetch(&mut self, address: u16) -> u8 {
        let mut update_open_bus = true;
        let value = {
            if address <= 0x1FFF {
                update_open_bus = false;
                let ind = address & 0x07FF;
                self.nes_ram[ind as usize]
            } else if address <= 0x3FFF {
                let ind = address & 0x0007;

                self.ppu
                    .cpu_read(ind, self.open_bus_value, self.mapper.as_ref())
            } else if address <= 0x4017 {
                if address == 0x4016 {
                    self.controller1.read()
                } else if address == 0x4017 {
                    self.controller2.read()
                } else {
                    // TODO APU
                    update_open_bus = false;
                    0
                }
            } else if address <= 0x5FFF {
                update_open_bus = false;
                self.open_bus_value
            } else if address <= 0x7FFF {
                if self.mapper.has_prg_ram() {
                    self.mapper.fetch_prg_ram(address)
                } else {
                    self.open_bus_value
                }
            } else {
                self.mapper.fetch_prg_rom(address)
            }
        };
        if update_open_bus {
            self.open_bus_value = value;
        }
        value
    }

    pub fn store(&mut self, address: u16, val: u8) {
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
            } else if address == 0x4014 {
                // DMA implementation
                // TODO stall?
                let mut buf = vec![0; 256];
                let start = (val as u16) << 8;
                self.fetch_many(start, &mut buf);
                self.ppu_mut().dma_copy(&buf);
            } else {
                // do nothing here
            }
        } else if address <= 0x401F {
            // do nothing here
        } else if address <= 0x7FFF {
            self.mapper.store_prg_ram(address, val);
        } else {
            self.mapper.store_prg_rom(address, val);
        }
    }

    pub fn fetch_many(&mut self, addr: u16, destination: &mut [u8]) {
        for (addr, v) in (addr..).zip(destination.iter_mut()) {
            *v = self.fetch(addr);
        }
    }

    pub fn store_many(&mut self, addr: u16, values: &[u8]) {
        for (addr, v) in (addr..).zip(values.iter()) {
            self.store(addr, *v);
        }
    }
}
