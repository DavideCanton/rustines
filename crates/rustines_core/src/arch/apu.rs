use crate::utils::bit_utils::{
    BitCount, BitIndex, extract_bits_mask_lsb, extract_bits_shift, extract_flag, set_flag,
};

#[derive(Default, Clone, Copy)]
enum ApuMode {
    #[default]
    FourStep,
    FiveStep,
}

impl From<bool> for ApuMode {
    fn from(value: bool) -> Self {
        if value {
            ApuMode::FiveStep
        } else {
            ApuMode::FourStep
        }
    }
}

#[derive(Default)]
pub struct Apu {
    pulse1: Pulse,
    pulse2: Pulse,
    triangle: Triangle,
    noise: Noise,
    dmc: Dmc,
    irq_disabled: bool,
    mode: ApuMode,
    frame_irq_active: bool,
    frame_cycles: u32,

    delayed_reset_cycles: Option<u8>,
    delayed_write_value: u8,
    cpu_cycle_count: u64,
}

impl Apu {
    pub fn tick(&mut self) {
        self.cpu_cycle_count += 1;

        self.dmc.tick();

        self.delayed_reset_cycles = match self.delayed_reset_cycles {
            None => None,
            Some(0) => {
                let val = self.delayed_write_value;
                self.mode = extract_flag(val, BitIndex::BIT_7).into();
                self.irq_disabled = extract_flag(val, BitIndex::BIT_6);
                self.frame_cycles = 0;
                None
            }
            Some(value) => Some(value - 1),
        };

        self.frame_cycles += 1;

        match self.mode {
            ApuMode::FiveStep => {
                if self.frame_cycles >= 37282 {
                    self.frame_cycles = 0;
                }
            }
            ApuMode::FourStep => {
                if (29828..=29831).contains(&self.frame_cycles) {
                    if !self.irq_disabled {
                        self.frame_irq_active = true;
                    }
                    if self.frame_cycles == 29831 {
                        self.frame_cycles = 0;
                    }
                }
            }
        }
    }

    pub fn irq_active(&self) -> bool {
        self.dmc.irq_active || self.frame_irq_active
    }

    pub fn cpu_write(&mut self, reg_index: u8, value: u8) {
        match reg_index {
            0x00 => self.pulse1.update_1(value),
            0x01 => self.pulse1.update_2(value),
            0x02 => self.pulse1.update_3(value),
            0x03 => self.pulse1.update_4(value),
            0x04 => self.pulse2.update_1(value),
            0x05 => self.pulse2.update_2(value),
            0x06 => self.pulse2.update_3(value),
            0x07 => self.pulse2.update_4(value),
            0x08 => self.triangle.update_1(value),
            0x0a => self.triangle.update_2(value),
            0x0b => self.triangle.update_3(value),
            0x0c => self.noise.update_1(value),
            0x0e => self.noise.update_2(value),
            0x0f => self.noise.update_3(value),
            0x10 => self.dmc.update_1(value),
            0x11 => self.dmc.update_2(value),
            0x12 => self.dmc.update_3(value),
            0x13 => self.dmc.update_4(value),
            0x15 => {
                self.dmc.irq_active = false;
                self.dmc.set_enabled(extract_flag(value, BitIndex::BIT_4));
                self.noise.set_enabled(extract_flag(value, BitIndex::BIT_3));
                self.triangle
                    .set_enabled(extract_flag(value, BitIndex::BIT_2));
                self.pulse2
                    .set_enabled(extract_flag(value, BitIndex::BIT_1));
                self.pulse1
                    .set_enabled(extract_flag(value, BitIndex::BIT_0));
            }
            0x17 => {
                self.delayed_write_value = value;

                if self.cpu_cycle_count.is_multiple_of(2) {
                    self.delayed_reset_cycles = Some(3);
                } else {
                    self.delayed_reset_cycles = Some(4);
                }

                if extract_flag(value, BitIndex::BIT_6) {
                    self.frame_irq_active = false;
                }
            }
            _ => {}
        }
    }

    pub fn cpu_read(&mut self, reg_index: u16, open_bus_value: u8) -> u8 {
        match reg_index {
            0x15 => {
                let mut ret = 0;

                // bit 5 is always open bus
                let open_bus_5 = extract_flag(open_bus_value, BitIndex::BIT_5);

                ret = set_flag(ret, BitIndex::BIT_7, self.dmc.irq_active);
                ret = set_flag(ret, BitIndex::BIT_6, self.frame_irq_active);
                ret = set_flag(ret, BitIndex::BIT_5, open_bus_5);
                ret = set_flag(ret, BitIndex::BIT_4, self.dmc.enabled);
                ret = set_flag(ret, BitIndex::BIT_3, self.noise.length_counter > 0);
                ret = set_flag(ret, BitIndex::BIT_2, self.triangle.length_counter_load > 0);
                ret = set_flag(ret, BitIndex::BIT_1, self.pulse2.length_counter_load > 0);
                ret = set_flag(ret, BitIndex::BIT_0, self.pulse1.length_counter_load > 0);

                self.frame_irq_active = false;
                self.dmc.irq_active = false;

                ret
            }
            _ => open_bus_value,
        }
    }
}

#[derive(Default)]
struct Pulse {
    duty: u8,
    loop_flag: bool,
    const_vol: bool,
    vol: u8,
    sweep_enabled: bool,
    sweep_period: u8,
    sweep_negate: bool,
    sweep_shift: u8,
    timer_low: u8,
    timer_high: u8,
    length_counter_load: u8,
    enabled: bool,
}

impl Pulse {
    fn update_1(&mut self, value: u8) {
        self.duty = extract_bits_shift(value, BitIndex::BIT_6, BitCount::BIT_2);
        self.loop_flag = extract_flag(value, BitIndex::BIT_5);
        self.const_vol = extract_flag(value, BitIndex::BIT_4);
        self.vol = extract_bits_mask_lsb(value, BitCount::BIT_4);
    }

    fn update_2(&mut self, value: u8) {
        self.sweep_enabled = extract_flag(value, BitIndex::BIT_7);
        self.sweep_period = extract_bits_shift(value, BitIndex::BIT_4, BitCount::BIT_3);
        self.sweep_negate = extract_flag(value, BitIndex::BIT_3);
        self.sweep_shift = extract_bits_mask_lsb(value, BitCount::BIT_3);
    }

    fn update_3(&mut self, value: u8) {
        self.timer_low = value;
    }

    fn update_4(&mut self, value: u8) {
        self.timer_high = extract_bits_mask_lsb(value, BitCount::BIT_3);
        self.length_counter_load = extract_bits_shift(value, BitIndex::BIT_3, BitCount::BIT_5);
    }
    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

#[derive(Default)]
struct Triangle {
    counter_ctl: bool,
    counter_load: u8,
    timer_low: u8,
    timer_high: u8,
    length_counter_load: u8,
    enabled: bool,
}

impl Triangle {
    fn update_1(&mut self, value: u8) {
        self.counter_ctl = extract_flag(value, BitIndex::BIT_7);
        self.counter_load = extract_bits_mask_lsb(value, BitCount::BIT_7);
    }

    fn update_2(&mut self, value: u8) {
        self.timer_low = value;
    }

    fn update_3(&mut self, value: u8) {
        self.timer_high = extract_bits_mask_lsb(value, BitCount::BIT_3);
        self.length_counter_load = extract_bits_shift(value, BitIndex::BIT_3, BitCount::BIT_5);
    }
    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

#[derive(Default)]
struct Noise {
    loop_flag: bool,
    const_vol: bool,
    vol: u8,
    length_counter: u8,
    noise_mode: bool,
    period: u8,
    enabled: bool,
}

impl Noise {
    fn update_1(&mut self, value: u8) {
        self.loop_flag = extract_flag(value, BitIndex::BIT_5);
        self.const_vol = extract_flag(value, BitIndex::BIT_4);
        self.vol = extract_bits_mask_lsb(value, BitCount::BIT_4);
    }

    fn update_2(&mut self, value: u8) {
        self.noise_mode = extract_flag(value, BitIndex::BIT_7);
        self.period = extract_bits_mask_lsb(value, BitCount::BIT_4);
    }

    fn update_3(&mut self, value: u8) {
        self.length_counter = extract_bits_shift(value, BitIndex::BIT_3, BitCount::BIT_5);
    }
    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

struct Dmc {
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

    pub fn tick(&mut self) {
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

    fn set_enabled(&mut self, enabled: bool) {
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

    fn update_1(&mut self, value: u8) {
        self.irq_enable = extract_flag(value, BitIndex::BIT_7);
        self.loop_flag = extract_flag(value, BitIndex::BIT_6);
        self.freq = extract_bits_mask_lsb(value, BitCount::BIT_4);

        if !self.irq_enable {
            self.irq_active = false;
        }
    }

    fn update_2(&mut self, value: u8) {
        self.load_counter = extract_bits_mask_lsb(value, BitCount::BIT_7);
    }

    fn update_3(&mut self, value: u8) {
        self.address = value as u16;
    }

    fn update_4(&mut self, value: u8) {
        self.sample_length = (value as u16 * 16) + 1;
        if !self.enabled {
            self.bytes_remaining = self.sample_length;
        }
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
