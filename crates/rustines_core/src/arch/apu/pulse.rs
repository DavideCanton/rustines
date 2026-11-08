use crate::{
    arch::{
        apu::common::{Envelope, LengthCounter, Sequencer},
        memento::{MementoReader, MementoReadAndFill, MementoWriteable, MementoWriter},
    },
    utils::bit_utils::{BitCount, BitIndex, BitManip},
};

pub struct Pulse {
    duty: u8,

    sweep_enabled: bool,
    sweep_period: u8,
    sweep_negate: bool,
    sweep_shift: u8,

    timer_low: u8,
    timer_high: u8,
    length_counter_load: u8,

    envelope: Envelope,

    // Internal state
    timer_counter: u16,
    sequencer: Sequencer,
    length_counter: LengthCounter,
    enabled: bool,
    sweep_divider: u8,
    sweep_reload: bool,
    is_pulse1: bool,
}

impl Pulse {
    pub fn new(is_pulse1: bool) -> Self {
        Self {
            duty: 0,
            sweep_enabled: false,
            sweep_period: 0,
            sweep_negate: false,
            sweep_shift: 0,
            timer_low: 0,
            timer_high: 0,
            length_counter_load: 0,
            envelope: Envelope::default(),
            timer_counter: 0,
            sequencer: Sequencer::new(8),
            length_counter: LengthCounter::default(),
            enabled: false,
            sweep_divider: 0,
            sweep_reload: false,
            is_pulse1,
        }
    }

    pub fn update_1(&mut self, value: u8) {
        self.duty = value.extract_bits_shift(BitIndex::_6, BitCount::_2);
        self.envelope.update(
            value.get_bit(BitIndex::_5),
            value.get_bit(BitIndex::_4),
            value.extract_bits_mask_lsb(BitCount::_4),
        );
    }

    pub fn update_2(&mut self, value: u8) {
        self.sweep_enabled = value.get_bit(BitIndex::_7);
        self.sweep_period = value.extract_bits_shift(BitIndex::_4, BitCount::_3);
        self.sweep_negate = value.get_bit(BitIndex::_3);
        self.sweep_shift = value.extract_bits_mask_lsb(BitCount::_3);
        self.sweep_reload = true;
    }

    pub fn update_3(&mut self, value: u8) {
        self.timer_low = value;
    }

    pub fn update_4(&mut self, value: u8) {
        self.timer_high = value.extract_bits_mask_lsb(BitCount::_3);
        self.length_counter_load = value.extract_bits_shift(BitIndex::_3, BitCount::_5);
        self.envelope.start();

        if self.enabled {
            self.length_counter
                .update(self.length_counter_load as usize);
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.length_counter.enabled_change(enabled);
    }

    pub fn length_counter(&self) -> &LengthCounter {
        &self.length_counter
    }

    fn sweep_target_period(&self) -> u16 {
        let period = self.timer_period();

        let change = period >> self.sweep_shift;

        if self.sweep_negate {
            // the result is clamped to zero if negative
            period
                .saturating_sub(change)
                // pulse 1 subtracts 1 more than pulse 2
                .saturating_sub(self.is_pulse1 as u16)
        } else {
            period.wrapping_add(change)
        }
    }

    fn timer_period(&self) -> u16 {
        ((self.timer_high as u16 & 0x07) << 8) | self.timer_low as u16
    }

    fn duty_output(&self) -> u8 {
        DUTY_TABLE[self.duty as usize][self.sequencer.value() as usize]
    }

    pub fn tick_timer(&mut self) {
        if self.timer_counter == 0 {
            self.timer_counter = self.timer_period();
            self.sequencer.increment();
        } else {
            self.timer_counter -= 1;
        }
    }

    pub fn output(&self) -> u8 {
        if !self.enabled
            || !self.length_counter.running()
            || self.sweep_mute()
            || self.duty_output() == 0
        {
            return 0;
        }

        self.envelope.output()
    }

    pub fn clock_envelope(&mut self) {
        self.envelope.clock();
    }

    pub fn clock_length_counter(&mut self) {
        if !self.envelope.loop_flag() {
            self.length_counter.decrement();
        }
    }

    pub fn clock_sweep(&mut self) {
        let target = self.sweep_target_period();

        if self.sweep_divider == 0
            && self.sweep_enabled
            && self.sweep_shift > 0
            && self.timer_period() >= 8
            && target <= 0x07FF
        {
            self.timer_low = target as u8;
            self.timer_high = (target >> 8) as u8 & 0x07;
        }

        if self.sweep_divider == 0 || self.sweep_reload {
            self.sweep_divider = self.sweep_period;
            self.sweep_reload = false;
        } else {
            self.sweep_divider -= 1;
        }
    }

    fn sweep_mute(&self) -> bool {
        let target = self.sweep_target_period();
        self.timer_period() < 8 || target > 0x07FF
    }
}

impl MementoWriteable for Pulse {
    fn write_memento<W: MementoWriter>(&self, writer: &mut W) {
        writer.write_memento(self.duty);
        writer.write_memento(self.sweep_enabled);
        writer.write_memento(self.sweep_period);
        writer.write_memento(self.sweep_negate);
        writer.write_memento(self.sweep_shift);
        writer.write_memento(self.timer_low);
        writer.write_memento(self.timer_high);
        writer.write_memento(self.length_counter_load);
        writer.write_memento_ref(&self.envelope);
        writer.write_memento(self.timer_counter);
        writer.write_memento_ref(&self.sequencer);
        writer.write_memento_ref(&self.length_counter);
        writer.write_memento(self.enabled);
        writer.write_memento(self.sweep_divider);
        writer.write_memento(self.sweep_reload);
        writer.write_memento(self.is_pulse1);
    }
}

impl MementoReadAndFill for Pulse {
    fn read_and_fill(&mut self, data: &mut MementoReader) {
        self.duty = data.read_memento();
        self.sweep_enabled = data.read_memento();
        self.sweep_period = data.read_memento();
        self.sweep_negate = data.read_memento();
        self.sweep_shift = data.read_memento();
        self.timer_low = data.read_memento();
        self.timer_high = data.read_memento();
        self.length_counter_load = data.read_memento();
        self.envelope.read_and_fill(data);
        self.timer_counter = data.read_memento();
        self.sequencer.read_and_fill(data);
        self.length_counter.read_and_fill(data);
        self.enabled = data.read_memento();
        self.sweep_divider = data.read_memento();
        self.sweep_reload = data.read_memento();
        self.is_pulse1 = data.read_memento();
    }
}

const DUTY_TABLE: [[u8; 8]; 4] = [
    [0, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 0, 0, 1, 1, 1, 1, 1],
];
