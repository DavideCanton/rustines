use log::trace;

use crate::arch::apu::Apu;
use crate::arch::controller::NesController;
use crate::arch::mappers::mapper::{Mapper, MapperBox};

use crate::arch::common::replace;
use crate::arch::ppu::Ppu;

pub struct Bus {
    nes_ram: [u8; 2048],
    ppu: Ppu,
    apu: Apu,
    mapper: MapperBox,
    controller1: NesController,
    controller2: NesController,
    open_bus_value: u8,
    cycles_cnt: u8,
    trace: bool,
}

impl Bus {
    pub fn new(mapper: MapperBox, ppu: Ppu, apu: Apu) -> Self {
        Self {
            nes_ram: [0; 2048],
            ppu,
            apu,
            mapper,
            controller1: NesController::new(1),
            controller2: NesController::new(2),
            open_bus_value: 0,
            cycles_cnt: 0,
            trace: false,
        }
    }

    pub fn set_trace(&mut self, trace: bool) {
        self.trace = trace;
    }

    pub fn start_tick(&mut self) {
        self.cycles_cnt = 0;
    }

    pub fn check(&self, exp: u8) -> Option<u8> {
        if exp == 0xFF {
            // invalid opcodes
            None
        } else if self.cycles_cnt != exp {
            Some(self.cycles_cnt)
        } else {
            None
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

    pub fn apu(&self) -> &Apu {
        &self.apu
    }

    pub fn apu_mut(&mut self) -> &mut Apu {
        &mut self.apu
    }

    pub fn mapper(&self) -> &dyn Mapper {
        self.mapper.as_ref()
    }

    pub fn burn_ppu_ticks(&mut self) {
        if self.trace {
            trace!("Burning PPU ticks")
        }
        self.internal_ppu_ticks();
    }

    fn ppu_ticks(&mut self) {
        if self.trace {
            trace!("PPU ticks from fetch/store")
        }
        self.internal_ppu_ticks();
    }

    fn internal_ppu_ticks(&mut self) {
        if self.trace {
            trace!("Advancing PPU")
        }
        self.cycles_cnt += 1;
        let mapper = self.mapper.as_mut();
        for _ in 0..3 {
            self.ppu.tick(mapper);
        }
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

    pub fn peek(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.nes_ram[(address & 0x07FF) as usize],
            0x8000..=0xFFFF => self.mapper.fetch_prg_rom(address),

            0x2002 => self.ppu.status_bits_shadow(),
            0x2007 => self.ppu.vram_buffer_shadow(self.mapper.as_ref()),
            0x4016 => self.controller1.peek_state(),
            0x4017 => self.controller2.peek_state(),

            // Per qualsiasi altro registro I/O restituiamo 0 o un valore sicuro
            _ => 0,
        }
    }

    pub fn fetch(&mut self, address: u16) -> u8 {
        let mut update_open_bus = true;
        let value = match address {
            0x0000..=0x1FFF => {
                update_open_bus = false;
                let ind = address & 0x07FF;
                self.nes_ram[ind as usize]
            }
            0x2000..=0x3FFF => {
                let ind = (address & 0x0007) as u8;
                self.ppu.cpu_read(ind, self.mapper.as_ref())
            }
            0x4000..=0x4017 => {
                if address == 0x4016 {
                    self.controller1.read()
                } else if address == 0x4017 {
                    self.controller2.read()
                } else if address <= 0x4015 {
                    let ind = address & 0xFF;
                    self.apu.cpu_read(ind, self.open_bus_value)
                } else {
                    update_open_bus = false;
                    0
                }
            }
            0x4018..=0x5FFF => {
                update_open_bus = false;
                self.open_bus_value
            }
            0x6000..=0x7FFF => {
                if self.mapper.has_prg_ram() {
                    self.mapper.fetch_prg_ram(address)
                } else {
                    self.open_bus_value
                }
            }
            _ => self.mapper.fetch_prg_rom(address),
        };
        if update_open_bus {
            self.open_bus_value = value;
        }
        if self.trace {
            trace!(
                "Fetch from bus, ADDRESS = {:#06X}, VALUE = {:#04X}",
                address, value
            );
        }
        self.ppu_ticks();
        value
    }

    pub fn store(&mut self, address: u16, val: u8) {
        if self.trace {
            trace!(
                "Store in bus, ADDRESS = {:#06X}, VALUE = {:#04X}",
                address, val
            );
        }

        self.ppu_ticks();

        match address {
            0x0000..=0x1FFF => {
                let ind = address & 0x07FF;
                replace(&mut self.nes_ram, ind as usize, val);
            }
            0x2000..=0x3FFF => {
                let ind = address & 0x0007;
                self.ppu.cpu_write(ind as u8, val, self.mapper.as_ref());
            }
            0x4000..=0x4017 => {
                if address == 0x4016 {
                    self.controller1.write(val);
                    self.controller2.write(val);
                } else if address == 0x4014 {
                    // DMA implementation
                    // TODO stall?
                    let mut buf = vec![0; 256];
                    let start = (val as u16) << 8;
                    self.fetch_many(start, &mut buf);
                    self.ppu_mut().dma_copy(&buf);
                } else {
                    let ind = address & 0xFF;
                    self.apu.cpu_write(ind as u8, val);
                }
            }
            0x4018..=0x401F => {
                // do nothing here
            }
            0x4020..=0x7FFF => {
                self.mapper.store_prg_ram(address, val);
            }
            _ => {
                self.mapper.store_prg_rom(address, val);
            }
        };
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
