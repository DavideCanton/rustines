use arch::memory::Memory;
use arch::registers::*;
use arch::instr_table::INSTR_TABLE;
use arch::bit_utils::*;

use std::rc::Rc;
use std::cell::RefCell;

pub struct CPU
{
    pub clock: u8,
    pub registers: Registers,
    pub memory: Rc<RefCell<Memory>>,
    pub irq: bool,
    pub nmi: bool,
    pub rst: bool
}

impl CPU
{
    pub fn new(mem: Rc<RefCell<Memory>>) -> CPU
    {
        init_flags();

        CPU
        {
            clock: 0,
            registers: Registers::new(),
            memory: mem,
            irq: false,
            nmi: false,
            rst: false
        }
    }

    fn save_state_before_interrupt(&mut self)
    {
        let pc = self.registers.PC;
        self.push16(pc);
        let p = self.registers.getP();
        self.push8(p);
    }

    fn perform_irq(&mut self)
    {
        self.save_state_before_interrupt();

        let low = self.memory.borrow().fetch(0xFFFE);
        let high = self.memory.borrow().fetch(0xFFFF);

        self.registers.PC = to_u16(low, high);
    }

    fn perform_nmi(&mut self)
    {
        self.save_state_before_interrupt();

        let low = self.memory.borrow().fetch(0xFFFA);
        let high = self.memory.borrow().fetch(0xFFFB);

        self.registers.PC = to_u16(low, high);
    }

    fn perform_rst(&mut self)
    {
        let low = self.memory.borrow().fetch(0xFFFC);
        let high = self.memory.borrow().fetch(0xFFFD);

        self.registers.PC = to_u16(low, high);
    }

    pub fn execute(&mut self)
    {
        loop
        {
            // fetch
            let opcode = self.memory.borrow().fetch(self.registers.PC);

            let instr = INSTR_TABLE[opcode as usize];

            //execute
            let (cycles, ilen) = instr(self);

            if cycles == 0xFF
            {
                break;
            }

            self.registers.PC += ilen as u16;

            // update time?

            println!("{}", cycles);
        }
    }

    pub fn push32(&mut self, v: u32)
    {
        let (low, high) = to_u16_lh(v);

        // TODO is the order right?
        self.push16(low);
        self.push16(high);
    }

    pub fn push16(&mut self, v: u16)
    {
        let (low, high) = to_u8_lh(v);

        // TODO is the order right?
        self.push8(low);
        self.push8(high);
    }

    pub fn push8(&mut self, v: u8)
    {
        self.memory.borrow_mut().store(self.registers.SP as u16 + 0x0100, v);
        self.registers.SP -= 1;
    }

    pub fn pop8(&mut self) -> u8
    {
        self.registers.SP += 1;
        self.memory.borrow().fetch(self.registers.SP as u16 + 0x0100)
    }

    pub fn pop16(&mut self) -> u16
    {
        let high = self.pop8();
        let low = self.pop8();

        to_u16(low, high)
    }

    pub fn pop32(&mut self) -> u32
    {
        let high = self.pop16();
        let low = self.pop16();

        to_u32(low, high)
    }
}
