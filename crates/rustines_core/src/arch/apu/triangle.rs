use crate::{
    arch::{
        apu::common::{LengthCounter, Sequencer},
        memento::{MementoReadAndFill, MementoReader, MementoWriteable, MementoWriter},
    },
    utils::bit_utils::{BitCount, BitIndex, BitManip},
};

pub struct Triangle {
    counter_ctl: bool,
    counter_load: u8,
    timer_low: u8,
    timer_high: u8,
    length_counter_load: u8,
    enabled: bool,

    // Internal state
    timer_counter: u16,
    length_counter: LengthCounter,

    linear_counter: u8,
    linear_counter_reload: bool,

    sequencer: Sequencer,
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            counter_ctl: false,
            counter_load: 0,
            timer_low: 0,
            timer_high: 0,
            length_counter_load: 0,
            enabled: false,
            timer_counter: 0,
            length_counter: LengthCounter::default(),
            linear_counter: 0,
            linear_counter_reload: false,
            sequencer: Sequencer::new(32),
        }
    }
}

impl Triangle {
    pub fn update_1(&mut self, value: u8) {
        self.counter_ctl = value.get_bit(BitIndex::_7);
        self.counter_load = value.extract_bits_mask_lsb(BitCount::_7);
    }

    pub fn update_2(&mut self, value: u8) {
        self.timer_low = value;
    }

    pub fn update_3(&mut self, value: u8) {
        self.timer_high = value.extract_bits_mask_lsb(BitCount::_3);
        self.length_counter_load = value.extract_bits_shift(BitIndex::_3, BitCount::_5);

        self.linear_counter_reload = true;

        if self.enabled {
            self.length_counter
                .update(self.length_counter_load as usize);
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.length_counter.enabled_change(enabled);
    }

    pub fn clock_linear_counter(&mut self) {
        if self.linear_counter_reload {
            self.linear_counter = self.counter_load;
        } else if self.linear_counter > 0 {
            self.linear_counter -= 1;
        }

        if !self.counter_ctl {
            self.linear_counter_reload = false;
        }
    }

    pub fn clock_length_counter(&mut self) {
        if !self.counter_ctl {
            self.length_counter.decrement();
        }
    }

    pub fn length_counter(&self) -> &LengthCounter {
        &self.length_counter
    }

    pub fn tick_timer(&mut self) {
        if self.timer_counter == 0 {
            let period = self.timer_period();
            self.timer_counter = period;

            // don't increment the sequencer if period < 2
            // to avoid high frequencies
            if self.linear_counter > 0 && self.length_counter.running() && period >= 2 {
                self.sequencer.increment();
            }
        } else {
            self.timer_counter -= 1;
        }
    }

    pub fn output(&self) -> u8 {
        TRIANGLE_SEQUENCE[self.sequencer.value() as usize]
    }

    fn timer_period(&self) -> u16 {
        ((self.timer_high as u16 & 0x07) << 8) | self.timer_low as u16
    }
}

impl MementoWriteable for Triangle {
    fn write_memento<W: MementoWriter>(&self, writer: &mut W) {
        writer.write_memento(self.counter_ctl);
        writer.write_memento(self.counter_load);
        writer.write_memento(self.timer_low);
        writer.write_memento(self.timer_high);
        writer.write_memento(self.length_counter_load);
        writer.write_memento(self.enabled);
        writer.write_memento(self.timer_counter);
        writer.write_memento_ref(&self.length_counter);
        writer.write_memento(self.linear_counter);
        writer.write_memento(self.linear_counter_reload);
        writer.write_memento_ref(&self.sequencer);
    }
}

impl MementoReadAndFill for Triangle {
    fn read_and_fill(&mut self, data: &mut MementoReader) {
        self.counter_ctl = data.read_memento();
        self.counter_load = data.read_memento();
        self.timer_low = data.read_memento();
        self.timer_high = data.read_memento();
        self.length_counter_load = data.read_memento();
        self.enabled = data.read_memento();
        self.timer_counter = data.read_memento();
        self.length_counter.read_and_fill(data);
        self.linear_counter = data.read_memento();
        self.linear_counter_reload = data.read_memento();
        self.sequencer.read_and_fill(data);
    }
}

const TRIANGLE_SEQUENCE: [u8; 32] = [
    15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
    13, 14, 15,
];
