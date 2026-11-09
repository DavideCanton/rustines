use crate::{
    arch::{
        apu::common::{Envelope, LengthCounter},
        memento::{MementoReadAndFill, MementoReader, MementoWriteable, MementoWriter},
    },
    utils::bit_utils::{BitCount, BitIndex, BitManip},
};

pub struct Noise {
    length_counter: LengthCounter,
    noise_mode: bool,
    period: u8,
    enabled: bool,

    timer_counter: u16,
    shift_register: u16,

    envelope: Envelope,
}

impl Default for Noise {
    fn default() -> Self {
        Self {
            length_counter: LengthCounter::default(),
            noise_mode: false,
            period: 0,
            enabled: false,
            timer_counter: 0,
            shift_register: 1,
            envelope: Envelope::default(),
        }
    }
}

impl Noise {
    pub fn update_1(&mut self, value: u8) {
        self.envelope.update(
            value.get_bit(BitIndex::_5),
            value.get_bit(BitIndex::_4),
            value.extract_bits_mask_lsb(BitCount::_4),
        );
    }

    pub fn update_2(&mut self, value: u8) {
        self.noise_mode = value.get_bit(BitIndex::_7);
        self.period = value.extract_bits_mask_lsb(BitCount::_4);
    }

    pub fn update_3(&mut self, value: u8) {
        let index = value.extract_bits_shift(BitIndex::_3, BitCount::_5);

        if self.enabled {
            self.length_counter.update(index as usize);
        }

        self.envelope.start();
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.length_counter.enabled_change(enabled);
    }

    pub fn clock_envelope(&mut self) {
        self.envelope.clock();
    }

    pub fn clock_length_counter(&mut self) {
        if !self.envelope.loop_flag() {
            self.length_counter.decrement();
        }
    }

    pub fn length_counter(&self) -> &LengthCounter {
        &self.length_counter
    }

    pub fn output(&self) -> u8 {
        if !self.enabled || !self.length_counter.running() || (self.shift_register & 1) != 0 {
            return 0;
        }

        self.envelope.output()
    }

    pub fn tick_timer(&mut self) {
        if self.timer_counter == 0 {
            self.timer_counter = self.timer_period();
            let tap = if self.noise_mode { 6 } else { 1 };

            let feedback = (self.shift_register & 1) ^ ((self.shift_register >> tap) & 1);

            self.shift_register >>= 1;
            self.shift_register |= feedback << 14;
        } else {
            self.timer_counter -= 1;
        }
    }

    fn timer_period(&self) -> u16 {
        NOISE_PERIOD_TABLE[self.period as usize]
    }
}

impl MementoWriteable for Noise {
    fn write_memento(&self, writer: &mut dyn MementoWriter) {
        self.length_counter.write_memento(writer);
        self.noise_mode.write_memento(writer);
        self.period.write_memento(writer);
        self.enabled.write_memento(writer);
        self.timer_counter.write_memento(writer);
        self.shift_register.write_memento(writer);
        self.envelope.write_memento(writer);
    }
}

impl MementoReadAndFill for Noise {
    fn read_and_fill(&mut self, data: &mut MementoReader) {
        self.length_counter.read_and_fill(data);
        self.noise_mode = data.read_memento();
        self.period = data.read_memento();
        self.enabled = data.read_memento();
        self.timer_counter = data.read_memento();
        self.shift_register = data.read_memento();
        self.envelope.read_and_fill(data);
    }
}

const NOISE_PERIOD_TABLE: [u16; 16] = [
    4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068,
];
