use super::rom_structs::NesRom;

pub trait FetchStore {
    fn fetch(&self, addr: u16) -> u8;
    fn store(&mut self, addr: u16, val: u8) -> u8;

    fn fetch_many(&self, addr: u16, destination: &mut [u8]) {
        for (i, v) in destination.iter_mut().enumerate() {
            *v = self.fetch(addr + i as u16);
        }
    }

    fn store_many(&mut self, addr: u16, values: &[u8]) {
        for (i, v) in values.iter().enumerate() {
            self.store(addr + (i as u16), *v);
        }
    }
}

pub struct Memory {
    mem: Vec<u8>,
    handlers: Vec<Box<dyn MemoryRegionHandler>>,
}

impl Memory {
    pub fn new(rom: NesRom) -> Self {
        let mut mem = vec![0; 0x10000];

        // TODO this should be handled by the mapper
        match rom.header.prg_rom_size {
            1 => {
                mem[0x8000..0xC000].copy_from_slice(&rom.prg_rom_banks[0].data);
                mem[0xC000..0x10000].copy_from_slice(&rom.prg_rom_banks[0].data);
            }
            2 => {
                mem[0x8000..0xC000].copy_from_slice(&rom.prg_rom_banks[0].data);
                mem[0xC000..0x10000].copy_from_slice(&rom.prg_rom_banks[1].data);
            }
            _ => panic!("Unsupported banks"),
        }

        Memory {
            mem,
            handlers: vec![
                Box::new(RamRegionHandler),
                Box::new(PpuRegionHandler),
                Box::new(IoRegionHandler),
                Box::new(ExpRomRegionHandler),
                Box::new(SramRegionHandler),
                Box::new(PrgRomRegionHandler),
            ],
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
    fn fetch(&self, addr: u16) -> u8 {
        let addr = addr as usize;

        let h = self.handlers.iter().find(|h| h.matches(addr)).unwrap();
        h.fetch(&self.mem, addr)
    }

    fn store(&mut self, addr: u16, val: u8) -> u8 {
        let addr = addr as usize;
        let h = self.handlers.iter().find(|h| h.matches(addr)).unwrap();
        h.store(&mut self.mem, addr, val)
    }
}

trait MemoryRegionHandler {
    fn matches(&self, addr: usize) -> bool;
    fn fetch(&self, memory: &[u8], addr: usize) -> u8;
    fn store(&self, memory: &mut [u8], addr: usize, val: u8) -> u8;
}

struct RamRegionHandler;

impl MemoryRegionHandler for RamRegionHandler {
    fn matches(&self, addr: usize) -> bool {
        addr < 0x2000
    }

    fn fetch(&self, memory: &[u8], addr: usize) -> u8 {
        let addr = addr & 0x7FF;
        memory[addr]
    }

    fn store(&self, memory: &mut [u8], addr: usize, val: u8) -> u8 {
        let addr = addr & 0x7FF;
        let old = memory[addr];
        memory[addr] = val;
        old
    }
}

struct PpuRegionHandler;

impl MemoryRegionHandler for PpuRegionHandler {
    fn matches(&self, addr: usize) -> bool {
        addr < 0x4000
    }

    fn fetch(&self, memory: &[u8], addr: usize) -> u8 {
        let addr = addr & 0x7;
        0
    }

    fn store(&self, memory: &mut [u8], addr: usize, val: u8) -> u8 {
        // let addr = addr & 0x7;
        unimplemented!();
    }
}

struct IoRegionHandler;

impl MemoryRegionHandler for IoRegionHandler {
    fn matches(&self, addr: usize) -> bool {
        addr < 0x4020
    }

    fn fetch(&self, memory: &[u8], addr: usize) -> u8 {
        unimplemented!();
    }

    fn store(&self, memory: &mut [u8], addr: usize, val: u8) -> u8 {
        unimplemented!();
    }
}

struct ExpRomRegionHandler;

impl MemoryRegionHandler for ExpRomRegionHandler {
    fn matches(&self, addr: usize) -> bool {
        addr < 0x6000
    }

    fn fetch(&self, memory: &[u8], addr: usize) -> u8 {
        unimplemented!();
    }

    fn store(&self, memory: &mut [u8], addr: usize, val: u8) -> u8 {
        unimplemented!();
    }
}

struct SramRegionHandler;

impl MemoryRegionHandler for SramRegionHandler {
    fn matches(&self, addr: usize) -> bool {
        addr < 0x8000
    }

    fn fetch(&self, memory: &[u8], addr: usize) -> u8 {
        unimplemented!();
    }

    fn store(&self, memory: &mut [u8], addr: usize, val: u8) -> u8 {
        unimplemented!();
    }
}

// TODO this should be handled by the mapper, understand how to relate cpu, memory and mapper
struct PrgRomRegionHandler;

impl MemoryRegionHandler for PrgRomRegionHandler {
    fn matches(&self, addr: usize) -> bool {
        addr <= 0xFFFF
    }

    fn fetch(&self, memory: &[u8], addr: usize) -> u8 {
        // TODO
        memory[addr]
    }

    fn store(&self, memory: &mut [u8], addr: usize, val: u8) -> u8 {
        unimplemented!();
    }
}
