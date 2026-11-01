use crate::utils::bit_utils::{BitCount, BitIndex, extract_bits_mask_lsb, extract_flag};

pub struct Dmc {
    irq_enable: bool,
    irq_active: bool,
    loop_flag: bool,
    freq: u8,
    load_counter: u8,
    address: u16,
    bytes_remaining: u16,
    bits_remaining: u8,
    sample_length: u16,
    timer: u16,
    enabled: bool,
}

impl Dmc {
    const PERIODS: [u16; 16] = [
        428, 380, 340, 320, 286, 254, 226, 214, 190, 160, 142, 128, 106, 84, 72, 54,
    ];

    pub fn tick_timer(&mut self) {
        if !self.enabled {
            return;
        }

        if self.timer > 0 {
            self.timer -= 1;
        } else {
            self.timer = self.get_period();

            if self.bits_remaining > 0 {
                self.bits_remaining -= 1;
            }

            if self.bits_remaining == 0 {
                self.bits_remaining = 8;

                if self.bytes_remaining > 0 {
                    self.bytes_remaining -= 1;

                    if self.bytes_remaining == 0 {
                        if self.loop_flag {
                            self.bytes_remaining = self.sample_length;
                        } else if self.irq_enable {
                            self.irq_active = true;
                        }
                    }
                }
            }
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        let previous_enabled = self.enabled;
        self.enabled = enabled;

        if !enabled {
            self.bytes_remaining = 0;
        } else {
            if !previous_enabled {
                self.bits_remaining = 8;
                self.timer = self.get_period();
            }

            if self.bytes_remaining == 0 {
                self.bytes_remaining = self.sample_length;
            }
        }
    }

    fn get_period(&self) -> u16 {
        Dmc::PERIODS[self.freq as usize]
    }

    pub fn update_1(&mut self, value: u8) {
        self.irq_enable = extract_flag(value, BitIndex::_7);
        self.loop_flag = extract_flag(value, BitIndex::_6);
        self.freq = extract_bits_mask_lsb(value, BitCount::_4);

        if !self.irq_enable {
            self.irq_active = false;
        }
    }

    pub fn update_2(&mut self, value: u8) {
        self.load_counter = extract_bits_mask_lsb(value, BitCount::_7);
    }

    pub fn update_3(&mut self, value: u8) {
        self.address = value as u16;
    }

    pub fn update_4(&mut self, value: u8) {
        self.sample_length = (value as u16 * 16) + 1;
        if !self.enabled {
            self.bytes_remaining = self.sample_length;
        }
    }

    pub fn irq_active(&self) -> bool {
        self.irq_active
    }

    pub fn set_irq_active(&mut self, value: bool) {
        self.irq_active = value;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn output(&self) -> u8 {
        0
    }
}

impl Default for Dmc {
    fn default() -> Self {
        Self {
            irq_enable: false,
            irq_active: false,
            loop_flag: false,
            freq: 0,
            load_counter: 0,
            address: 0,
            bytes_remaining: 0,
            bits_remaining: 0,
            sample_length: 0,
            timer: Dmc::PERIODS[0],
            enabled: false,
        }
    }
}
