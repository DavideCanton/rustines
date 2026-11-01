use crate::{
    arch::apu::constants::LENGTH_TABLE,
    utils::bit_utils::{
        BitCount, BitIndex, extract_bits_mask_lsb, extract_bits_shift, extract_flag,
    },
};

pub struct Pulse {
    // $4000
    duty: u8,
    loop_flag: bool,
    const_vol: bool,
    vol: u8,

    // $4001
    sweep_enabled: bool,
    sweep_period: u8,
    sweep_negate: bool,
    sweep_shift: u8,

    // $4002 / $4003
    timer_low: u8,
    timer_high: u8,
    length_counter_load: u8,

    // Envelope internal state
    envelope_start: bool,
    envelope_divider: u8,
    envelope_decay: u8,

    // Internal state
    timer_counter: u16,
    sequence_position: u8,
    length_counter: u8,
    enabled: bool,
    sweep_divider: u8,
    sweep_reload: bool,
    is_pulse1: bool,
}

impl Pulse {
    pub fn new(is_pulse1: bool) -> Self {
        Self {
            duty: 0,
            loop_flag: false,
            const_vol: false,
            vol: 0,
            sweep_enabled: false,
            sweep_period: 0,
            sweep_negate: false,
            sweep_shift: 0,
            timer_low: 0,
            timer_high: 0,
            length_counter_load: 0,
            envelope_start: false,
            envelope_divider: 0,
            envelope_decay: 0,
            timer_counter: 0,
            sequence_position: 0,
            length_counter: 0,
            enabled: false,
            sweep_divider: 0,
            sweep_reload: false,
            is_pulse1,
        }
    }

    pub fn update_1(&mut self, value: u8) {
        self.duty = extract_bits_shift(value, BitIndex::_6, BitCount::_2);
        self.loop_flag = extract_flag(value, BitIndex::_5);
        self.const_vol = extract_flag(value, BitIndex::_4);
        self.vol = extract_bits_mask_lsb(value, BitCount::_4);
    }

    pub fn update_2(&mut self, value: u8) {
        self.sweep_enabled = extract_flag(value, BitIndex::_7);
        self.sweep_period = extract_bits_shift(value, BitIndex::_4, BitCount::_3);
        self.sweep_negate = extract_flag(value, BitIndex::_3);
        self.sweep_shift = extract_bits_mask_lsb(value, BitCount::_3);
        self.sweep_reload = true;
    }

    pub fn update_3(&mut self, value: u8) {
        self.timer_low = value;
    }

    pub fn update_4(&mut self, value: u8) {
        self.timer_high = extract_bits_mask_lsb(value, BitCount::_3);
        self.length_counter_load = extract_bits_shift(value, BitIndex::_3, BitCount::_5);
        self.envelope_start = true;

        if self.enabled {
            self.length_counter = LENGTH_TABLE[self.length_counter_load as usize];
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.length_counter = 0;
        }
    }

    pub fn length_counter_load(&self) -> u8 {
        self.length_counter_load
    }

    fn sweep_target_period(&self) -> u16 {
        let period = self.timer_period();

        let change = period >> self.sweep_shift;

        if self.sweep_negate {
            period
                .wrapping_sub(change)
                .wrapping_sub(self.is_pulse1 as u16)
        } else {
            period.wrapping_add(change)
        }
    }

    fn timer_period(&self) -> u16 {
        ((self.timer_high as u16 & 0x07) << 8) | self.timer_low as u16
    }

    fn duty_output(&self) -> u8 {
        DUTY_TABLE[self.duty as usize][self.sequence_position as usize]
    }

    pub fn tick_timer(&mut self) {
        if self.timer_counter == 0 {
            self.timer_counter = self.timer_period();
            self.sequence_position = (self.sequence_position + 1) & 0x07;
        } else {
            self.timer_counter -= 1;
        }
    }

    pub fn output(&self) -> u8 {
        if !self.enabled
            || self.length_counter == 0
            || self.sweep_mute()
            || self.timer_period() < 8
            || self.duty_output() == 0
        {
            return 0;
        }

        if self.const_vol {
            self.vol
        } else {
            self.envelope_decay
        }
    }

    pub fn clock_envelope(&mut self) {
        if self.envelope_start {
            self.envelope_start = false;
            self.envelope_decay = 15;
            self.envelope_divider = self.vol;
        } else {
            if self.envelope_divider > 0 {
                self.envelope_divider -= 1;
            } else {
                self.envelope_divider = self.vol;

                if self.envelope_decay > 0 {
                    self.envelope_decay -= 1;
                } else if self.loop_flag {
                    self.envelope_decay = 15;
                }
            }
        }
    }

    pub fn clock_length_counter(&mut self) {
        if !self.loop_flag && self.length_counter > 0 {
            self.length_counter -= 1;
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

const DUTY_TABLE: [[u8; 8]; 4] = [
    [0, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 0, 0, 1, 1, 1, 1, 1],
];
