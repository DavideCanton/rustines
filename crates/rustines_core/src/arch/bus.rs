use log::trace;

use crate::{
    arch::{
        apu::Apu,
        controller::NesController,
        mappers::mapper::{Mapper, MapperBox},
        memento::{
            MementoReadAndFill, MementoReadable, MementoReader, MementoWriteable, MementoWriter,
        },
        ppu::Ppu,
        zapper::Zapper,
    },
    utils::bit_utils::to_u16,
};

pub enum Controller2 {
    NesController(NesController),
    Zapper(Zapper),
}

impl Controller2 {
    pub fn nes_controller() -> Self {
        Controller2::NesController(NesController::new(2))
    }

    pub fn zapper() -> Self {
        Controller2::Zapper(Zapper::new())
    }

    pub fn reset(&mut self) {
        match self {
            Controller2::NesController(c) => c.reset(),
            Controller2::Zapper(c) => c.reset(),
        }
    }
}

pub struct Bus {
    nes_ram: [u8; 2048],
    ppu: Ppu,
    apu: Apu,
    mapper: MapperBox,
    controller1: NesController,
    controller2: Controller2,
    open_bus_value: u8,
    cycles_cnt: usize,
    total_cycles: usize,
    tracing_enabled: bool,
    oam_dma_state: OamDmaState,
    dmc_dma_state: DmcDmaState,
}

impl Bus {
    pub fn new(mapper: MapperBox, ppu: Ppu, apu: Apu, controller2: Controller2) -> Self {
        Self {
            nes_ram: [0; 2048],
            ppu,
            apu,
            mapper,
            controller1: NesController::new(1),
            controller2,
            open_bus_value: 0,
            cycles_cnt: 0,
            total_cycles: 0,
            tracing_enabled: false,
            oam_dma_state: OamDmaState::NotRunning,
            dmc_dma_state: DmcDmaState::NotRunning,
        }
    }

    pub fn reset(&mut self) {
        self.nes_ram.fill(0);
        self.ppu.reset();
        self.apu.reset();
        self.mapper.reset();
        self.controller1.reset();
        self.controller2.reset();
        self.open_bus_value = 0;
        self.cycles_cnt = 0;
        self.total_cycles = 0;
        self.oam_dma_state = OamDmaState::NotRunning;
        self.dmc_dma_state = DmcDmaState::NotRunning;
    }

    pub fn enable_tracing(&mut self, tracing_enabled: bool) {
        self.tracing_enabled = tracing_enabled;
    }

    pub fn tick_started(&mut self) {
        self.cycles_cnt = 0;
    }

    pub fn tick_ended(&mut self, exp: u8) -> Option<usize> {
        let exp = exp as usize;

        if self.cycles_cnt != exp {
            Some(self.cycles_cnt)
        } else {
            None
        }
    }

    pub fn total_cycles(&self) -> usize {
        self.total_cycles
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

    pub fn mapper_mut(&mut self) -> &mut dyn Mapper {
        self.mapper.as_mut()
    }

    pub fn controller1_mut(&mut self) -> &mut NesController {
        &mut self.controller1
    }

    pub fn controller2_mut(&mut self) -> Option<&mut NesController> {
        match &mut self.controller2 {
            Controller2::NesController(ctrl) => Some(ctrl),
            Controller2::Zapper(_) => None,
        }
    }
    pub fn zapper_mut(&mut self) -> Option<&mut Zapper> {
        match &mut self.controller2 {
            Controller2::Zapper(zapper) => Some(zapper),
            Controller2::NesController(_) => None,
        }
    }

    pub fn set_zapper_input(&mut self, position: Option<(usize, usize)>, trigger_pressed: bool) {
        if let Some(zapper) = self.zapper_mut() {
            zapper.set_input(position, trigger_pressed);
            self.ppu.set_zapper_position(position);
        }
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
        self.cycles_cnt += 1;
        self.total_cycles += 1;

        let mapper = self.mapper.as_mut();
        if self.tracing_enabled {
            trace!("Advancing PPU x 3 AND APU x 1");
        }
        for _ in 0..3 {
            self.ppu.tick(mapper);
        }

        self.apu_tick();
    }

    fn apu_tick(&mut self) {
        self.apu.tick();

        let dmc = self.apu.dmc();

        if self.dmc_dma_state == DmcDmaState::NotRunning && dmc.needs_dma() {
            self.dmc_dma_state = DmcDmaState::Halt {
                address: dmc.current_address(),
            };
        }
    }

    /// Peek the value at the given address without affecting the open bus value or triggering
    /// side effects.
    pub fn peek(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.nes_ram[(address & 0x7FF) as usize],
            0x6000..=0xFFFF => self.mapper.fetch_cpu(address),
            0x2002 => self.ppu.status_bits_shadow(),
            0x2007 => self.ppu.vram_buffer_shadow(self.mapper.as_ref()),
            0x4016 => self.controller1.peek_state(),
            0x4017 => match &self.controller2 {
                Controller2::NesController(ctrl) => ctrl.peek_state(),
                Controller2::Zapper(zapper) => zapper.read(self.ppu.zapper_light_detected()),
            },
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
                    let data = match &mut self.controller2 {
                        Controller2::NesController(ctrl) => ctrl.read(),
                        Controller2::Zapper(zapper) => {
                            zapper.read(self.ppu.zapper_light_detected())
                        }
                    };
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
            0x6000..=0xFFFF => {
                self.mapper.fetch_cpu(address)
                // TODO
                // self.open_bus_value
            }
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
                self.nes_ram[ind as usize] = val;
            }
            0x2000..=0x3FFF => {
                let ind = address & 0b0000_0000_0000_0111;
                self.ppu.cpu_write(ind as u8, val, self.mapper.as_mut());
            }
            0x4000..=0x4017 => {
                if address == 0x4016 {
                    self.controller1.write(val);
                    if let Some(ctrl) = self.controller2_mut() {
                        ctrl.write(val)
                    };
                } else if address == 0x4014 {
                    let address = (val as u16) << 8;
                    // DMA implementation

                    let state = OamDmaState::Starting {
                        cnt: 255,
                        address,
                        needs_alignment: !self.total_cycles.is_multiple_of(2),
                    };

                    self.oam_dma_state = state;
                } else {
                    let ind = address & 0b1111_1111;
                    self.apu.cpu_write(ind as u8, val);
                }
            }
            0x4018..=0x401F => {
                // do nothing here
            }
            0x4020..=0xFFFF => {
                self.mapper.store_cpu(address, val);
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

    pub fn dmc_needs_dma(&self) -> bool {
        self.apu.dmc().needs_dma()
    }

    pub fn oam_dma_state(&self) -> OamDmaState {
        self.oam_dma_state
    }

    pub fn is_oam_dma_state_running(&self) -> bool {
        !matches!(self.oam_dma_state, OamDmaState::NotRunning)
    }

    pub(crate) fn set_oam_dma_state(&mut self, next_state: OamDmaState) {
        self.oam_dma_state = next_state;
    }

    pub(crate) fn is_dmc_dma_state_running(&self) -> bool {
        !matches!(self.dmc_dma_state, DmcDmaState::NotRunning)
    }

    pub fn dmc_dma_state(&self) -> DmcDmaState {
        self.dmc_dma_state
    }

    pub(crate) fn set_dmc_dma_state(&mut self, next_state: DmcDmaState) {
        self.dmc_dma_state = next_state;
    }
}

impl MementoWriteable for Bus {
    fn write_memento<W: MementoWriter>(&self, writer: &mut W) {
        writer.write_memento_ref(&self.nes_ram.as_slice());
        writer.write_memento_ref(self.ppu.state());
        writer.write_memento_ref(self.apu.state());
        // mapper
        writer.write_memento(self.open_bus_value);
        writer.write_memento(self.cycles_cnt);
        writer.write_memento(self.total_cycles);
        writer.write_memento(self.tracing_enabled);
        writer.write_memento(self.oam_dma_state);
        writer.write_memento(self.dmc_dma_state);
    }
}

impl MementoReadAndFill for Bus {
    fn read_and_fill(&mut self, data: &mut MementoReader) {
        self.nes_ram = data.read_memento();
        self.ppu.state_mut().read_and_fill(data);
        self.apu.state_mut().read_and_fill(data);
        // mapper
        self.controller1.reset();
        self.controller2.reset();
        self.open_bus_value = data.read_memento();
        self.cycles_cnt = data.read_memento();
        self.total_cycles = data.read_memento();
        self.tracing_enabled = data.read_memento();
        self.oam_dma_state = data.read_memento();
        self.dmc_dma_state = data.read_memento();
    }
}

#[derive(Clone, Copy)]
pub enum OamDmaState {
    NotRunning,
    Starting {
        cnt: usize,
        address: u16,
        needs_alignment: bool,
    },
    Aligning {
        cnt: usize,
        address: u16,
    },
    Reading {
        cnt: usize,
        address: u16,
    },
    Writing {
        cnt: usize,
        address: u16,
        data: u8,
    },
}

impl MementoWriteable for OamDmaState {
    fn write_memento<W: MementoWriter>(&self, writer: &mut W) {
        match self {
            OamDmaState::NotRunning => writer.write_memento(0u8),
            OamDmaState::Starting {
                cnt,
                address,
                needs_alignment,
            } => {
                writer.write_memento(1u8);
                writer.write_memento_ref(cnt);
                writer.write_memento_ref(address);
                writer.write_memento_ref(needs_alignment);
            }
            OamDmaState::Aligning { cnt, address } => {
                writer.write_memento(2u8);
                writer.write_memento_ref(cnt);
                writer.write_memento_ref(address);
            }
            OamDmaState::Reading { cnt, address } => {
                writer.write_memento(3u8);
                writer.write_memento_ref(cnt);
                writer.write_memento_ref(address);
            }
            OamDmaState::Writing { cnt, address, data } => {
                writer.write_memento(4u8);
                writer.write_memento_ref(cnt);
                writer.write_memento_ref(address);
                writer.write_memento_ref(data);
            }
        }
    }
}
impl MementoReadable for OamDmaState {
    fn read_memento(reader: &mut MementoReader) -> Self {
        match reader.read_memento::<u8>() {
            0 => OamDmaState::NotRunning,
            1 => OamDmaState::Starting {
                cnt: reader.read_memento(),
                address: reader.read_memento(),
                needs_alignment: reader.read_memento(),
            },
            2 => OamDmaState::Aligning {
                cnt: reader.read_memento(),
                address: reader.read_memento(),
            },
            3 => OamDmaState::Reading {
                cnt: reader.read_memento(),
                address: reader.read_memento(),
            },
            4 => OamDmaState::Writing {
                cnt: reader.read_memento(),
                address: reader.read_memento(),
                data: reader.read_memento(),
            },
            _ => panic!("Invalid value"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DmcDmaState {
    NotRunning,
    Halt { address: u16 },
    Dummy { address: u16 },
    Alignment { address: u16 },
    Read { address: u16 },
}

impl MementoWriteable for DmcDmaState {
    fn write_memento<W: MementoWriter>(&self, writer: &mut W) {
        match self {
            DmcDmaState::NotRunning => writer.write_memento(0u8),
            DmcDmaState::Halt { address } => {
                writer.write_memento(1u8);
                writer.write_memento_ref(address);
            }
            DmcDmaState::Dummy { address } => {
                writer.write_memento(2u8);
                writer.write_memento_ref(address);
            }
            DmcDmaState::Alignment { address } => {
                writer.write_memento(3u8);
                writer.write_memento_ref(address);
            }
            DmcDmaState::Read { address } => {
                writer.write_memento(4u8);
                writer.write_memento_ref(address);
            }
        }
    }
}
impl MementoReadable for DmcDmaState {
    fn read_memento(reader: &mut MementoReader) -> Self {
        match reader.read_memento::<u8>() {
            0 => DmcDmaState::NotRunning,
            1 => DmcDmaState::Halt {
                address: reader.read_memento(),
            },
            2 => DmcDmaState::Dummy {
                address: reader.read_memento(),
            },
            3 => DmcDmaState::Alignment {
                address: reader.read_memento(),
            },
            4 => DmcDmaState::Read {
                address: reader.read_memento(),
            },
            _ => panic!("Invalid value"),
        }
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
        self.page_boundary_crossed as u8
    }
}
