use crate::arch::memento::{
    MementoReadAndFill, MementoReader, MementoResult, MementoWriteable, MementoWriter,
};

#[derive(Default, Debug)]
pub struct LengthCounter {
    counter: u8,
}

impl LengthCounter {
    pub fn enabled_change(&mut self, enabled: bool) {
        if !enabled {
            self.counter = 0;
        }
    }

    pub fn update(&mut self, index: usize) {
        self.counter = LENGTH_TABLE[index];
    }

    pub fn decrement(&mut self) {
        if self.running() {
            self.counter -= 1;
        }
    }

    pub fn running(&self) -> bool {
        self.counter > 0
    }
}

impl MementoWriteable for LengthCounter {
    fn write_memento(&self, writer: &mut dyn MementoWriter) -> MementoResult {
        self.counter.write_memento(writer)
    }
}

impl MementoReadAndFill for LengthCounter {
    fn read_and_fill(&mut self, data: &mut MementoReader) -> MementoResult {
        self.counter = data.read_memento()?;
        Ok(())
    }
}

const LENGTH_TABLE: [u8; 32] = [
    10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26, 14, 12, 16, 24, 18, 48, 20, 96, 22,
    192, 24, 72, 26, 16, 28, 32, 30,
];

#[derive(Default)]
pub struct Envelope {
    start: bool,
    divider: u8,
    decay: u8,
    loop_flag: bool,
    const_vol: bool,
    vol: u8,
}

impl Envelope {
    const MAX_DECAY: u8 = 15;

    pub(crate) fn start(&mut self) {
        self.start = true;
    }

    pub(crate) fn update(&mut self, loop_flag: bool, const_vol: bool, vol: u8) {
        self.loop_flag = loop_flag;
        self.const_vol = const_vol;
        self.vol = vol;
    }

    pub(crate) fn clock(&mut self) {
        if self.start {
            self.start = false;
            self.decay = Envelope::MAX_DECAY;
            self.divider = self.vol;
        } else if self.divider > 0 {
            self.divider -= 1;
        } else {
            self.divider = self.vol;

            if self.decay > 0 {
                self.decay -= 1;
            } else if self.loop_flag {
                self.decay = Envelope::MAX_DECAY;
            }
        }
    }

    pub(crate) fn output(&self) -> u8 {
        if self.const_vol { self.vol } else { self.decay }
    }

    pub(crate) fn loop_flag(&self) -> bool {
        self.loop_flag
    }
}

impl MementoWriteable for Envelope {
    fn write_memento(&self, writer: &mut dyn MementoWriter) -> MementoResult {
        self.start.write_memento(writer)?;
        self.divider.write_memento(writer)?;
        self.decay.write_memento(writer)?;
        self.loop_flag.write_memento(writer)?;
        self.const_vol.write_memento(writer)?;
        self.vol.write_memento(writer)?;
        Ok(())
    }
}

impl MementoReadAndFill for Envelope {
    fn read_and_fill(&mut self, data: &mut MementoReader) -> MementoResult {
        self.start = data.read_memento()?;
        self.divider = data.read_memento()?;
        self.decay = data.read_memento()?;
        self.loop_flag = data.read_memento()?;
        self.const_vol = data.read_memento()?;
        self.vol = data.read_memento()?;
        Ok(())
    }
}

pub struct Sequencer {
    value: u8,
    max: u8,
}

impl Sequencer {
    pub fn new(max: u8) -> Self {
        Self { value: 0, max }
    }

    pub(crate) fn value(&self) -> u8 {
        self.value
    }

    pub(crate) fn increment(&mut self) {
        self.value += 1;
        if self.value == self.max {
            self.value = 0;
        }
    }
}

impl MementoWriteable for Sequencer {
    fn write_memento(&self, writer: &mut dyn MementoWriter) -> MementoResult {
        self.value.write_memento(writer)?;
        self.max.write_memento(writer)?;
        Ok(())
    }
}

impl MementoReadAndFill for Sequencer {
    fn read_and_fill(&mut self, data: &mut MementoReader) -> MementoResult {
        self.value = data.read_memento()?;
        self.max = data.read_memento()?;
        Ok(())
    }
}
