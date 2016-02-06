pub struct Memory
{
    memory: Vec<u8>
}

impl Memory
{
    pub fn new() -> Memory
    {
        //TODO initialize memory properly
        Memory { memory: vec![0; 1 << 16] }
    }

    pub fn from_array(mem: Vec<u8>) -> Option<Memory>
    {
        if mem.len() == 1 << 16
        {
            Some(Memory { memory: mem })
        }
        else
        {
            None
        }
    }

    pub fn fetch(&self, addr: u16) -> u8
    {
        //TODO how to handle mirroring?
        self.memory[addr as usize]
    }

    pub fn store(&mut self, addr: u16, val: u8) -> u8
    {
        let addr = addr as usize;
        let old = self.memory[addr];
        self.memory[addr] = val;
        old
    }

    pub fn push8(&mut self, sp: u8, val: u8)
    {
        let sp = sp as u16 + 0x0100;
        self.store(sp, val);
    }

    pub fn peek8(&self, sp: u8) -> u8
    {
        let sp = sp as u16 + 0x0100;
        self.fetch(sp)
    }
}
