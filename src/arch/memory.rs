pub struct Memory
{
    memory: [u8; 1 << 16]
}

impl Memory
{
    pub fn new() -> Memory
    {
        //TODO initialize memory properly
        Memory { memory: [0; 1 << 16] }
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
}
