use crate::utils::bit_utils::{extract_bits_mask_lsb, extract_bits_shift, extract_flag};

#[derive(Default)]
pub struct Apu {
    pulse1: Pulse,
    pulse2: Pulse,
    triangle: Triangle,
    noise: Noise,
    dmc: Dmc,
    irq_disabled: bool,
    mode: bool,
}

impl Apu {
    pub fn tick(&mut self) {
        self.dmc.tick();
    }

    pub fn irq_active(&self) -> bool {
        self.dmc.irq_active
    }

    pub fn cpu_write(&mut self, reg_index: u8, value: u8) {
        match reg_index {
            0 => self.pulse1.update_1(value),
            1 => self.pulse1.update_2(value),
            2 => self.pulse1.update_3(value),
            3 => self.pulse1.update_4(value),
            4 => self.pulse2.update_1(value),
            5 => self.pulse2.update_2(value),
            6 => self.pulse2.update_3(value),
            7 => self.pulse2.update_4(value),
            8 => self.triangle.update_1(value),
            10 => self.triangle.update_2(value),
            11 => self.triangle.update_3(value),
            12 => self.noise.update_1(value),
            14 => self.noise.update_2(value),
            15 => self.noise.update_3(value),
            16 => self.dmc.update_1(value),
            17 => self.dmc.update_2(value),
            18 => self.dmc.update_3(value),
            19 => self.dmc.update_4(value),
            21 => {
                log::info!(
                    "Scrittura $4015: abilitato={}, lunghezza_campionata={}",
                    extract_flag(value, 4),
                    self.dmc.sample_length
                );

                self.dmc.irq_active = false;
                self.dmc.set_enabled(extract_flag(value, 4));
                self.noise.set_enabled(extract_flag(value, 3));
                self.triangle.set_enabled(extract_flag(value, 2));
                self.pulse2.set_enabled(extract_flag(value, 1));
                self.pulse1.set_enabled(extract_flag(value, 0));
            }
            23 => {
                self.mode = extract_flag(value, 7);
                self.irq_disabled = extract_flag(value, 6);
            }
            _ => {}
        }
    }

    pub fn cpu_read(&mut self, reg_index: u16, open_bus_value: u8) -> u8 {
        match reg_index {
            21 => {
                let mut ret = 0;

                if self.dmc.irq_active {
                    ret |= 1 << 7;
                }

                if false {
                    // TODO F
                    ret |= 1 << 6;
                }

                if self.dmc.enabled {
                    ret |= 1 << 4;
                }
                if self.noise.length_counter > 0 {
                    ret |= 1 << 3;
                }
                if self.triangle.length_counter_load > 0 {
                    ret |= 1 << 2;
                }
                if self.pulse2.length_counter_load > 0 {
                    ret |= 1 << 1;
                }
                if self.pulse1.length_counter_load > 0 {
                    ret |= 1;
                }

                self.dmc.irq_active = false;

                // bit 5 is always open bus
                (ret & 0xDF) | (open_bus_value & 0x20)
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
        self.duty = extract_bits_shift(value, 6, 2);
        self.loop_flag = extract_flag(value, 5);
        self.const_vol = extract_flag(value, 4);
        self.vol = extract_bits_mask_lsb(value, 4);
    }

    fn update_2(&mut self, value: u8) {
        self.sweep_enabled = extract_flag(value, 7);
        self.sweep_period = extract_bits_shift(value, 4, 3);
        self.sweep_negate = extract_flag(value, 3);
        self.sweep_shift = extract_bits_mask_lsb(value, 3);
    }

    fn update_3(&mut self, value: u8) {
        self.timer_low = value;
    }

    fn update_4(&mut self, value: u8) {
        self.timer_high = extract_bits_mask_lsb(value, 3);
        self.length_counter_load = extract_bits_shift(value, 3, 5);
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
        self.counter_ctl = extract_flag(value, 7);
        self.counter_load = extract_bits_mask_lsb(value, 7);
    }

    fn update_2(&mut self, value: u8) {
        self.timer_low = value;
    }

    fn update_3(&mut self, value: u8) {
        self.timer_high = extract_bits_mask_lsb(value, 3);
        self.length_counter_load = extract_bits_shift(value, 3, 5);
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
        self.loop_flag = extract_flag(value, 5);
        self.const_vol = extract_flag(value, 4);
        self.vol = extract_bits_mask_lsb(value, 4);
    }

    fn update_2(&mut self, value: u8) {
        self.noise_mode = extract_flag(value, 7);
        self.period = extract_bits_mask_lsb(value, 4);
    }

    fn update_3(&mut self, value: u8) {
        self.length_counter = extract_bits_shift(value, 3, 5);
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
    pub fn tick(&mut self) {
        if !self.enabled {
            return;
        }

        if self.timer > 0 {
            self.timer -= 1;
        } else {
            self.timer = DMC_PERIOD_TABLE[(self.freq & 0x0F) as usize];

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
                            log::info!(
                                "DMC IRQ SCATTATO ADESSO! Attivato dopo il corretto intervallo di bit."
                            );
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
                self.timer = DMC_PERIOD_TABLE[(self.freq & 0x0F) as usize];
            }

            if self.bytes_remaining == 0 {
                self.bytes_remaining = self.sample_length;
            }
        }
    }

    fn update_1(&mut self, value: u8) {
        self.irq_enable = extract_flag(value, 7);
        self.loop_flag = extract_flag(value, 6);
        self.freq = extract_bits_mask_lsb(value, 4);

        if !self.irq_enable {
            self.irq_active = false;
        }
    }

    fn update_2(&mut self, value: u8) {
        self.load_counter = extract_bits_mask_lsb(value, 7);
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
            timer: DMC_PERIOD_TABLE[0],
            enabled: false,
        }
    }
}

const DMC_PERIOD_TABLE: [u16; 16] = [
    428, 380, 340, 320, 286, 254, 226, 214, 190, 160, 142, 128, 106, 84, 72, 54,
];
