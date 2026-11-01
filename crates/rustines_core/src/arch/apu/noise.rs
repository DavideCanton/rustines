use crate::{
    arch::apu::constants::LENGTH_TABLE,
    utils::bit_utils::{
        BitCount, BitIndex, extract_bits_mask_lsb, extract_bits_shift, extract_flag,
    },
};

pub struct Noise {
    loop_flag: bool,
    const_vol: bool,
    vol: u8,
    length_counter: u8,
    noise_mode: bool,
    period: u8,
    enabled: bool,

    timer_counter: u16,
    shift_register: u16,

    envelope_start: bool,
    envelope_divider: u8,
    envelope_decay: u8,
}

impl Default for Noise {
    fn default() -> Self {
        Self {
            loop_flag: false,
            const_vol: false,
            vol: 0,
            length_counter: 0,
            noise_mode: false,
            period: 0,
            enabled: false,
            timer_counter: 0,
            shift_register: 1,
            envelope_start: false,
            envelope_divider: 0,
            envelope_decay: 0,
        }
    }
}

impl Noise {
    pub fn update_1(&mut self, value: u8) {
        self.loop_flag = extract_flag(value, BitIndex::_5);
        self.const_vol = extract_flag(value, BitIndex::_4);
        self.vol = extract_bits_mask_lsb(value, BitCount::_4);
    }

    pub fn update_2(&mut self, value: u8) {
        self.noise_mode = extract_flag(value, BitIndex::_7);
        self.period = extract_bits_mask_lsb(value, BitCount::_4);
    }

    pub fn update_3(&mut self, value: u8) {
        let index = extract_bits_shift(value, BitIndex::_3, BitCount::_5);

        if self.enabled {
            self.length_counter = LENGTH_TABLE[index as usize];
        }

        self.envelope_start = true;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;

        if !enabled {
            self.length_counter = 0;
        }
    }

    pub fn clock_envelope(&mut self) {
        if self.envelope_start {
            self.envelope_start = false;
            self.envelope_decay = 15;
            self.envelope_divider = self.vol;
        } else if self.envelope_divider == 0 {
            self.envelope_divider = self.vol;

            if self.envelope_decay > 0 {
                self.envelope_decay -= 1;
            } else if self.loop_flag {
                self.envelope_decay = 15;
            }
        } else {
            self.envelope_divider -= 1;
        }
    }

    pub fn clock_length_counter(&mut self) {
        if !self.loop_flag && self.length_counter > 0 {
            self.length_counter -= 1;
        }
    }

    pub fn length_counter(&self) -> u8 {
        self.length_counter
    }

    pub fn output(&self) -> u8 {
        if !self.enabled || self.length_counter == 0 || (self.shift_register & 1) != 0 {
            return 0;
        }

        if self.const_vol {
            self.vol
        } else {
            self.envelope_decay
        }
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

const NOISE_PERIOD_TABLE: [u16; 16] = [
    4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068,
];
