use log::trace;

use crate::arch::apu::Apu;
use crate::arch::controller::NesController;
use crate::arch::mappers::mapper::{Mapper, MapperBox};

use crate::arch::common::replace;
use crate::arch::ppu::Ppu;
use crate::utils::bit_utils::to_u16;

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

    pub fn burn_cycle_from_cpu(&mut self) {
        self.do_internal_cycle();
    }

    fn burn_cycle_from_bus(&mut self) {
        self.do_internal_cycle();
    }

    fn do_internal_cycle(&mut self) {
        self.cycles_cnt += 1;
        let mapper = self.mapper.as_mut();
        if self.trace {
            trace!("Advancing PPU x 3 AND APU x 1");
        }
        for _ in 0..3 {
            self.ppu.tick(mapper);
        }
        self.apu.tick();
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

            _ => self.open_bus_value,
        }
    }

    pub fn fetch(&mut self, address: u16) -> u8 {
        let mut update_open_bus = true;
        let value = match address {
            0x0000..=0x1FFF => {
                let ind = address & 0x07FF;
                self.nes_ram[ind as usize]
            }
            0x2000..=0x3FFF => {
                let ind = (address & 0x0007) as u8;
                self.ppu.cpu_read(ind, self.mapper.as_ref())
            }
            0x4000..=0x4017 => {
                if address == 0x4016 {
                    let data = self.controller1.read();
                    (data & 0x1F) | (self.open_bus_value & 0xE0)
                } else if address == 0x4017 {
                    let data = self.controller2.read();
                    (data & 0x1F) | (self.open_bus_value & 0xE0)
                } else if address == 0x4015 {
                    update_open_bus = false;
                    let ind = address & 0xFF;
                    self.apu.cpu_read(ind, self.open_bus_value)
                } else {
                    update_open_bus = false;
                    self.open_bus_value
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
        self.burn_cycle_from_bus();
        value
    }

    pub fn store(&mut self, address: u16, val: u8) {
        if self.trace {
            trace!(
                "Store in bus, ADDRESS = {:#06X}, VALUE = {:#04X}",
                address, val
            );
        }

        self.open_bus_value = val;

        self.burn_cycle_from_bus();

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
                    if let Some(&last_dma_byte) = buf.last() {
                        self.open_bus_value = last_dma_byte;
                    }
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

    pub fn read_with_dummy(&mut self, low: u8, high: u8, offset: u8, is_write: bool) -> (u16, u8) {
        let base = to_u16(low, high);
        let raw_addr = base.wrapping_add(offset as u16);
        let boundary = if low.overflowing_add(offset).1 { 1 } else { 0 };
        let dummy_addr = (base & 0xFF00) | (raw_addr & 0x00FF);

        // dummy read
        if boundary == 1 {
            // if boundary is crossed, do a dummy read in any case at the wrong address
            let _ = self.fetch(dummy_addr);
            (raw_addr, 1)
        } else {
            // if boundary is not crossed, only write instructions do a dummy read to the same address
            if is_write {
                let _ = self.fetch(dummy_addr);
            }
            (raw_addr, 0)
        }
    }
}
