use crate::{
    AudioOutputBox,
    arch::{
        apu::{
            dmc::Dmc,
            filter::{Filter, HighPassFilter, LowPassFilter, SequenceFilter},
            mixer::Mixer,
            noise::Noise,
            pulse::Pulse,
            triangle::Triangle,
        },
        memento::{
            MementoReadAndFill, MementoReader, MementoResult, MementoWriteable, MementoWriter,
        },
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

impl From<ApuMode> for bool {
    fn from(value: ApuMode) -> Self {
        match value {
            ApuMode::FiveStep => true,
            ApuMode::FourStep => false,
        }
    }
}

pub struct ApuState {
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
}

impl ApuState {
    pub fn new() -> Self {
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
        }
    }
}

impl MementoWriteable for ApuState {
    fn write_memento(&self, writer: &mut dyn MementoWriter) -> MementoResult {
        self.pulse1.write_memento(writer)?;
        self.pulse2.write_memento(writer)?;
        self.triangle.write_memento(writer)?;
        self.noise.write_memento(writer)?;
        self.dmc.write_memento(writer)?;
        self.irq_disabled.write_memento(writer)?;
        Into::<bool>::into(self.mode).write_memento(writer)?;
        self.frame_irq_active.write_memento(writer)?;
        self.frame_cycles.write_memento(writer)?;
        self.delayed_reset_cycles.write_memento(writer)?;
        self.delayed_write_value.write_memento(writer)?;
        self.cpu_cycle_count.write_memento(writer)?;
        self.sample_accumulator.write_memento(writer)?;
        Ok(())
    }
}

impl MementoReadAndFill for ApuState {
    fn read_and_fill(&mut self, data: &mut MementoReader) -> MementoResult {
        self.pulse1.read_and_fill(data)?;
        self.pulse2.read_and_fill(data)?;
        self.triangle.read_and_fill(data)?;
        self.noise.read_and_fill(data)?;
        self.dmc.read_and_fill(data)?;
        self.irq_disabled = data.read_memento()?;
        self.mode = data.read_memento::<bool>()?.into();
        self.frame_irq_active = data.read_memento()?;
        self.frame_cycles = data.read_memento()?;
        self.delayed_reset_cycles = data.read_memento()?;
        self.delayed_write_value = data.read_memento()?;
        self.cpu_cycle_count = data.read_memento()?;
        self.sample_accumulator = data.read_memento()?;
        Ok(())
    }
}

pub struct Apu {
    state: ApuState,
    output: AudioOutputBox,
    filter: SequenceFilter,
    mixer: Mixer,
}

impl Apu {
    pub fn new(output: AudioOutputBox) -> Self {
        let sample_rate = output.sample_rate() as f32;
        Self {
            state: ApuState::new(),
            output,
            filter: SequenceFilter::new(vec![
                Box::new(HighPassFilter::new(90.0, sample_rate)),
                Box::new(HighPassFilter::new(440.0, sample_rate)),
                Box::new(LowPassFilter::new(14_000.0, sample_rate)),
            ]),
            mixer: Mixer::default(),
        }
    }

    pub fn reset(&mut self) {
        self.state = ApuState::new();
    }

    pub fn state(&self) -> &ApuState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut ApuState {
        &mut self.state
    }

    pub fn tick(&mut self) {
        self.state.cpu_cycle_count += 1;

        if self.state.cpu_cycle_count.is_multiple_of(2) {
            self.state.pulse1.tick_timer();
            self.state.pulse2.tick_timer();
            self.state.noise.tick_timer();
            self.state.dmc.tick_timer();
        }

        self.state.triangle.tick_timer();

        self.update_frame_counter();
        self.emit_output();
    }

    fn emit_output(&mut self) {
        self.state.sample_accumulator += self.output.sample_rate();

        if self.state.sample_accumulator >= CPU_CLOCK {
            self.state.sample_accumulator -= CPU_CLOCK;

            let sample = self.mix_output();
            let sample = self.filter.process(sample);

            self.output.push_sample(sample);
        }
    }

    fn update_frame_counter(&mut self) {
        self.state.delayed_reset_cycles = match self.state.delayed_reset_cycles {
            None => None,
            Some(0) => {
                let val = self.state.delayed_write_value;
                self.state.mode = val.get_bit(BitIndex::_7).into();
                self.state.irq_disabled = val.get_bit(BitIndex::_6);
                self.state.frame_cycles = 0;

                if self.state.mode == ApuMode::FiveStep {
                    self.quarter_frame();
                    self.half_frame();
                }

                None
            }
            Some(value) => Some(value - 1),
        };

        self.state.frame_cycles += 1;

        match self.state.frame_cycles {
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
            29829 if self.state.mode == ApuMode::FourStep => {
                self.quarter_frame();
                self.half_frame();

                if !self.state.irq_disabled {
                    self.state.frame_irq_active = true;
                }

                self.state.frame_cycles = 0;
            }
            37281 if self.state.mode == ApuMode::FiveStep => {
                self.quarter_frame();
                self.half_frame();
                self.state.frame_cycles = 0;
            }
            _ => {}
        }
    }

    pub fn irq_active(&self) -> bool {
        self.state.dmc.irq_active() || self.state.frame_irq_active
    }

    pub fn cpu_write(&mut self, reg_index: u8, value: u8) {
        match reg_index {
            0x00 => self.state.pulse1.update_1(value),
            0x01 => self.state.pulse1.update_2(value),
            0x02 => self.state.pulse1.update_3(value),
            0x03 => self.state.pulse1.update_4(value),
            0x04 => self.state.pulse2.update_1(value),
            0x05 => self.state.pulse2.update_2(value),
            0x06 => self.state.pulse2.update_3(value),
            0x07 => self.state.pulse2.update_4(value),
            0x08 => self.state.triangle.update_1(value),
            0x0a => self.state.triangle.update_2(value),
            0x0b => self.state.triangle.update_3(value),
            0x0c => self.state.noise.update_1(value),
            0x0e => self.state.noise.update_2(value),
            0x0f => self.state.noise.update_3(value),
            0x10 => self.state.dmc.update_1(value),
            0x11 => self.state.dmc.update_2(value),
            0x12 => self.state.dmc.update_3(value),
            0x13 => self.state.dmc.update_4(value),
            0x15 => {
                self.state.dmc.set_irq_active(false);
                self.state.dmc.set_enabled(value.get_bit(BitIndex::_4));
                self.state.noise.set_enabled(value.get_bit(BitIndex::_3));
                self.state.triangle.set_enabled(value.get_bit(BitIndex::_2));
                self.state.pulse2.set_enabled(value.get_bit(BitIndex::_1));
                self.state.pulse1.set_enabled(value.get_bit(BitIndex::_0));
            }
            0x17 => {
                self.state.delayed_write_value = value;

                if self.state.cpu_cycle_count.is_multiple_of(2) {
                    self.state.delayed_reset_cycles = Some(2);
                } else {
                    self.state.delayed_reset_cycles = Some(3);
                }

                if value.get_bit(BitIndex::_6) {
                    self.state.frame_irq_active = false;
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

                ret = ret.set_bit(BitIndex::_7, self.state.dmc.irq_active());
                ret = ret.set_bit(BitIndex::_6, self.state.frame_irq_active);
                ret = ret.set_bit(BitIndex::_5, open_bus_5);
                ret = ret.set_bit(BitIndex::_4, self.state.dmc.enabled());
                ret = ret.set_bit(BitIndex::_3, self.state.noise.length_counter().running());
                ret = ret.set_bit(BitIndex::_2, self.state.triangle.length_counter().running());
                ret = ret.set_bit(BitIndex::_1, self.state.pulse2.length_counter().running());
                ret = ret.set_bit(BitIndex::_0, self.state.pulse1.length_counter().running());

                self.state.frame_irq_active = false;
                self.state.dmc.set_irq_active(false);

                ret
            }
            _ => open_bus_value,
        }
    }

    fn quarter_frame(&mut self) {
        self.state.pulse1.clock_envelope();
        self.state.pulse2.clock_envelope();
        self.state.noise.clock_envelope();
        self.state.triangle.clock_linear_counter();
    }

    fn half_frame(&mut self) {
        self.state.pulse1.clock_length_counter();
        self.state.pulse2.clock_length_counter();

        self.state.triangle.clock_length_counter();
        self.state.noise.clock_length_counter();

        self.state.pulse1.clock_sweep();
        self.state.pulse2.clock_sweep();
    }

    fn mix_output(&self) -> f32 {
        let pulse1 = self.state.pulse1.output();
        let pulse2 = self.state.pulse2.output();
        let triangle = self.state.triangle.output();
        let noise = self.state.noise.output();
        let dmc = self.state.dmc.output();

        self.mixer.mix_output(pulse1, pulse2, triangle, noise, dmc)
    }

    pub fn dmc_mut(&mut self) -> &mut Dmc {
        &mut self.state.dmc
    }

    pub fn dmc(&self) -> &Dmc {
        &self.state.dmc
    }

    pub fn mixer_mut(&mut self) -> &mut Mixer {
        &mut self.mixer
    }
}
