use super::rom_structs::NesRom;

pub struct Memory {
    mem: Vec<u8>,
    handlers: Vec<Box<dyn MemoryRegionHandler>>,
}

impl Memory {
    pub fn new(rom: NesRom) -> Self {
        let mut mem = vec![0; 0x10000];
        unsafe {
            let ptr = mem.as_mut_ptr();
            let first;
            let second;

            // TODO this should be handled by the mapper
            match rom.header.prg_rom_size {
                1 => {
                    let prg = &rom.prg_rom_banks[0];
                    first = prg.data.as_ptr();
                    second = prg.data.as_ptr();
                }
                2 => {
                    first = rom.prg_rom_banks[0].data.as_ptr();
                    second = rom.prg_rom_banks[1].data.as_ptr();
                }
                _ => panic!("Unsupported banks"),
            }
            std::ptr::copy_nonoverlapping(first, ptr.offset(0x8000), 0x4000);
            std::ptr::copy_nonoverlapping(second, ptr.offset(0xC000), 0x4000);
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

    pub fn fetch(&self, addr: u16) -> u8 {
        let addr = addr as usize;

        let h = self.handlers.iter().find(|h| h.matches(addr)).unwrap();
        h.fetch(&self.mem, addr)
    }

    pub fn fetch_many(&self, addr: u16, count: u16) -> Box<[u8]> {
        let mut v = Vec::with_capacity(count as usize);
        for i in addr..(addr + count) {
            v.push(self.fetch(i));
        }
        v.into_boxed_slice()
    }

    pub fn store(&mut self, addr: u16, val: u8) -> u8 {
        let addr = addr as usize;
        let h = self.handlers.iter().find(|h| h.matches(addr)).unwrap();
        h.store(&mut self.mem, addr, val)
    }

    pub fn store_many(&mut self, addr: u16, values: &[u8]) {
        for (i, v) in values.iter().enumerate() {
            self.store(addr + (i as u16), *v);
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
