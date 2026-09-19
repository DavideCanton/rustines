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
    cycles_cnt: usize,
    tracing_enabled: bool,
    dma_in_progress: bool,
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
            tracing_enabled: false,
            dma_in_progress: false,
        }
    }

    pub fn enable_tracing(&mut self, tracing_enabled: bool) {
        self.tracing_enabled = tracing_enabled;
    }

    pub fn tick_started(&mut self) {
        self.cycles_cnt = 0;
    }

    pub fn check_tick_end(&self, exp: u8) -> Option<usize> {
        let exp = exp as usize;

        if self.cycles_cnt != exp {
            Some(self.cycles_cnt)
        } else {
            None
        }
    }

    pub fn push(&mut self, sp: u8, val: u8) {
        let sp = sp as u16 + 0x0100;
        self.write(sp, val);
    }

    pub fn pop(&mut self, sp: u8) -> u8 {
        let sp = sp as u16 + 0x0100;
        self.read(sp)
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

    pub fn mapper_ref(&self) -> &dyn Mapper {
        self.mapper.as_ref()
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

    pub fn burn_cycle_from_cpu(&mut self) {
        self.do_internal_cycle();
    }

    fn burn_cycle_from_bus(&mut self) {
        self.do_internal_cycle();
    }

    fn do_internal_cycle(&mut self) {
        if !self.dma_in_progress {
            self.cycles_cnt += 1;
        }

        let mapper = self.mapper.as_mut();
        if self.tracing_enabled {
            trace!("Advancing PPU x 3 AND APU x 1");
        }
        for _ in 0..3 {
            self.ppu.tick(mapper);
        }
        self.apu.tick();
    }

    /// Peek the value at the given address without affecting the open bus value or triggering
    /// side effects.
    pub fn peek(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.nes_ram[(address & 0b0000_0111_1111_1111) as usize],
            0x8000..=0xFFFF => self.mapper.fetch_prg_rom(address),

            0x2002 => self.ppu.status_bits_shadow(),
            0x2007 => self.ppu.vram_buffer_shadow(self.mapper.as_ref()),
            0x4016 => self.controller1.peek_state(),
            0x4017 => self.controller2.peek_state(),

            _ => self.open_bus_value,
        }
    }

    /// Fetch the value at the given address, updating the open bus value and triggering side
    /// effects.
    pub fn read(&mut self, address: u16) -> u8 {
        let mut update_open_bus = true;
        let value = match address {
            0x0000..=0x1FFF => {
                let ind = address & 0b0000_0111_1111_1111;
                self.nes_ram[ind as usize]
            }
            0x2000..=0x3FFF => {
                let ind = (address & 0b0000_0000_0000_0111) as u8;
                self.ppu.cpu_read(ind, self.mapper.as_ref())
            }
            0x4000..=0x4017 => {
                if address == 0x4016 {
                    let data = self.controller1.read();
                    (data & 0b0001_1111) | (self.open_bus_value & 0b1110_0000)
                } else if address == 0x4017 {
                    let data = self.controller2.read();
                    (data & 0b0001_1111) | (self.open_bus_value & 0b1110_0000)
                } else if address == 0x4015 {
                    update_open_bus = false;
                    let ind = address & 0b1111_1111;
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
        if self.tracing_enabled {
            trace!(
                "Fetch from bus, ADDRESS = {:#06X}, VALUE = {:#04X}",
                address, value
            );
        }
        self.burn_cycle_from_bus();
        value
    }

    /// Store the value at the given address, updating the open bus value and triggering side
    /// effects.
    pub fn write(&mut self, address: u16, val: u8) {
        if self.tracing_enabled {
            trace!(
                "Store in bus, ADDRESS = {:#06X}, VALUE = {:#04X}",
                address, val
            );
        }

        self.open_bus_value = val;

        self.burn_cycle_from_bus();

        match address {
            0x0000..=0x1FFF => {
                let ind = address & 0b0000_0111_1111_1111;
                replace(&mut self.nes_ram, ind as usize, val);
            }
            0x2000..=0x3FFF => {
                let ind = address & 0b0000_0000_0000_0111;
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
                    self.dma_in_progress = true;
                    self.read_many(start, &mut buf);
                    self.dma_in_progress = false;
                    self.open_bus_value = buf[255];
                    self.ppu_mut().dma_copy(&buf);
                } else {
                    let ind = address & 0b1111_1111;
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

    /// Read multiple bytes from the bus starting at the given address into the provided destination
    /// slice. The destination slice will be filled with the read values.
    ///
    /// NOTE: This method will trigger side effects and update the open bus value for each read.
    pub fn read_many(&mut self, addr: u16, destination: &mut [u8]) {
        for (addr, v) in (addr..).zip(destination.iter_mut()) {
            *v = self.read(addr);
        }
    }

    /// Store multiple bytes to the bus starting at the given address from the provided source
    /// slice. The source slice will be read for the values to store.
    ///
    /// NOTE: This method will trigger side effects and update the open bus value for each write.
    pub fn write_many(&mut self, addr: u16, values: &[u8]) {
        for (addr, v) in (addr..).zip(values.iter()) {
            self.write(addr, *v);
        }
    }

    /// Simulates a dummy read from the bus, which is used in certain addressing modes to account
    /// for the extra cycle when crossing a page boundary.
    ///
    /// It returns the final address after adding the offset and a flag indicating whether a page
    /// boundary was crossed (1 if crossed, 0 otherwise).
    pub fn perform_dummy_read(
        &mut self,
        low: u8,
        high: u8,
        offset: u8,
        is_write: bool,
    ) -> DummyReadResult {
        let base = to_u16(low, high);
        let raw_addr = base.wrapping_add(offset as u16);
        let boundary_crossed = low.overflowing_add(offset).1;
        let dummy_addr = (base & 0b1111_1111_0000_0000) | (raw_addr & 0b0000_0000_1111_1111);

        // dummy read
        // if boundary is crossed, do a dummy read in any case at the wrong address, else
        // only write instructions do a dummy read to the same address
        if boundary_crossed || is_write {
            let _ = self.read(dummy_addr);
        }
        DummyReadResult::new(raw_addr, boundary_crossed)
    }
}

pub struct DummyReadResult {
    address: u16,
    page_boundary_crossed: bool,
}

impl DummyReadResult {
    fn new(address: u16, page_boundary_crossed: bool) -> Self {
        Self {
            address,
            page_boundary_crossed,
        }
    }

    pub fn address(&self) -> u16 {
        self.address
    }

    pub fn page_boundary_crossed(&self) -> bool {
        self.page_boundary_crossed
    }

    pub fn page_boundary_crossed_as_u8(&self) -> u8 {
        if self.page_boundary_crossed { 1 } else { 0 }
    }
}
