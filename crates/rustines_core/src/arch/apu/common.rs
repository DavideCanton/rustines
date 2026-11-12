use rustines_macro::Memento;

use crate::arch::memento::{
    MementoReadAndFill, MementoReader, MementoResult, MementoWriteable, MementoWriter,
};

#[derive(Default, Debug, Memento)]
pub struct LengthCounter {
    #[memento]
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

const LENGTH_TABLE: [u8; 32] = [
    10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26, 14, 12, 16, 24, 18, 48, 20, 96, 22,
    192, 24, 72, 26, 16, 28, 32, 30,
];

#[derive(Default, Memento)]
pub struct Envelope {
    #[memento]
    start: bool,
    #[memento]
    divider: u8,
    #[memento]
    decay: u8,
    #[memento]
    loop_flag: bool,
    #[memento]
    const_vol: bool,
    #[memento]
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

#[derive(Memento)]
pub struct Sequencer {
    #[memento]
    value: u8,
    #[memento]
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
