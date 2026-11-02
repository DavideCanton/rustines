use crate::{
    AudioOutputBox,
    arch::apu::{
        dmc::Dmc,
        filter::{Filter, HighPassFilter, LowPassFilter, SequenceFilter},
        noise::Noise,
        pulse::Pulse,
        triangle::Triangle,
    },
    utils::bit_utils::{BitIndex, BitManip},
};

// 48 kHz
const CPU_CLOCK: u64 = 1_789_773;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum ApuMode {
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

    sample_accumulator: u64,

    output: AudioOutputBox,
    filter: SequenceFilter,
}

impl Apu {
    pub fn new(output: AudioOutputBox) -> Self {
        let sample_rate = output.sample_rate() as f32;
        Self {
            pulse1: Pulse::new(true),
            pulse2: Pulse::new(false),
            triangle: Triangle::default(),
            noise: Noise::default(),
            dmc: Dmc::new(),
            irq_disabled: false,
            mode: ApuMode::FourStep,
            frame_irq_active: false,
            frame_cycles: 0,

            delayed_reset_cycles: None,
            delayed_write_value: 0,
            cpu_cycle_count: 0,

            sample_accumulator: 0,

            output,
            filter: SequenceFilter::new(vec![
                Box::new(HighPassFilter::new(90.0, sample_rate)),
                Box::new(HighPassFilter::new(440.0, sample_rate)),
                Box::new(LowPassFilter::new(14_000.0, sample_rate)),
            ]),
        }
    }

    pub fn tick(&mut self) {
        self.cpu_cycle_count += 1;

        if self.cpu_cycle_count.is_multiple_of(2) {
            self.pulse1.tick_timer();
            self.pulse2.tick_timer();
            self.noise.tick_timer();
            self.dmc.tick_timer();
        }

        self.triangle.tick_timer();

        self.update_frame_counter();
        self.emit_output();
    }

    fn emit_output(&mut self) {
        self.sample_accumulator += self.output.sample_rate();

        if self.sample_accumulator >= CPU_CLOCK {
            self.sample_accumulator -= CPU_CLOCK;

            let sample = self.mix_output();
            let sample = self.filter.process(sample);

            self.output.push_sample(sample);
        }
    }

    fn update_frame_counter(&mut self) {
        self.delayed_reset_cycles = match self.delayed_reset_cycles {
            None => None,
            Some(0) => {
                let val = self.delayed_write_value;
                self.mode = val.get_bit(BitIndex::_7).into();
                self.irq_disabled = val.get_bit(BitIndex::_6);
                self.frame_cycles = 0;

                if self.mode == ApuMode::FiveStep {
                    self.quarter_frame();
                    self.half_frame();
                }

                None
            }
            Some(value) => Some(value - 1),
        };

        self.frame_cycles += 1;

        match self.frame_cycles {
            7457 => {
                self.quarter_frame();
            }
            14913 => {
                self.quarter_frame();
                self.half_frame();
            }
            22371 => {
                self.quarter_frame();
            }
            29829 if self.mode == ApuMode::FourStep => {
                self.quarter_frame();
                self.half_frame();

                if !self.irq_disabled {
                    self.frame_irq_active = true;
                }

                self.frame_cycles = 0;
            }
            37281 if self.mode == ApuMode::FiveStep => {
                self.quarter_frame();
                self.half_frame();
                self.frame_cycles = 0;
            }
            _ => {}
        }
    }

    pub fn irq_active(&self) -> bool {
        self.dmc.irq_active() || self.frame_irq_active
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
                self.dmc.set_irq_active(false);
                self.dmc.set_enabled(value.get_bit(BitIndex::_4));
                self.noise.set_enabled(value.get_bit(BitIndex::_3));
                self.triangle.set_enabled(value.get_bit(BitIndex::_2));
                self.pulse2.set_enabled(value.get_bit(BitIndex::_1));
                self.pulse1.set_enabled(value.get_bit(BitIndex::_0));
            }
            0x17 => {
                self.delayed_write_value = value;

                if self.cpu_cycle_count.is_multiple_of(2) {
                    self.delayed_reset_cycles = Some(2);
                } else {
                    self.delayed_reset_cycles = Some(3);
                }

                if value.get_bit(BitIndex::_6) {
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
                let open_bus_5 = open_bus_value.get_bit(BitIndex::_5);

                ret = ret.set_bit(BitIndex::_7, self.dmc.irq_active());
                ret = ret.set_bit(BitIndex::_6, self.frame_irq_active);
                ret = ret.set_bit(BitIndex::_5, open_bus_5);
                ret = ret.set_bit(BitIndex::_4, self.dmc.enabled());
                ret = ret.set_bit(BitIndex::_3, self.noise.length_counter().running());
                ret = ret.set_bit(BitIndex::_2, self.triangle.length_counter().running());
                ret = ret.set_bit(BitIndex::_1, self.pulse2.length_counter().running());
                ret = ret.set_bit(BitIndex::_0, self.pulse1.length_counter().running());

                self.frame_irq_active = false;
                self.dmc.set_irq_active(false);

                ret
            }
            _ => open_bus_value,
        }
    }

    fn quarter_frame(&mut self) {
        self.pulse1.clock_envelope();
        self.pulse2.clock_envelope();
        self.noise.clock_envelope();
        self.triangle.clock_linear_counter();
    }

    fn half_frame(&mut self) {
        self.pulse1.clock_length_counter();
        self.pulse2.clock_length_counter();

        self.triangle.clock_length_counter();
        self.noise.clock_length_counter();

        self.pulse1.clock_sweep();
        self.pulse2.clock_sweep();
    }

    fn mix_output(&self) -> f32 {
        let pulse1 = self.pulse1.output() as f32;
        let pulse2 = self.pulse2.output() as f32;

        let pulse_sum = pulse1 + pulse2;
        let pulse_out = if pulse_sum == 0.0 {
            0.0
        } else {
            95.88 / (8128.0 / pulse_sum + 100.0)
        };

        let triangle = self.triangle.output() as f32;
        let noise = self.noise.output() as f32;
        let dmc = self.dmc.output() as f32;

        let tnd_sum = triangle / 8227.0 + noise / 12241.0 + dmc / 22638.0;

        let tnd_out = if tnd_sum == 0.0 {
            0.0
        } else {
            159.79 / (1.0 / tnd_sum + 100.0)
        };

        pulse_out + tnd_out
    }

    pub fn dmc_mut(&mut self) -> &mut Dmc {
        &mut self.dmc
    }

    pub fn dmc(&self) -> &Dmc {
        &self.dmc
    }
}
