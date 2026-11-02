#[derive(Debug, Clone, Copy)]
pub struct Flag(bool);

impl Flag {
    pub fn toggle(&mut self) {
        self.0 = !self.0;
    }

    pub fn set(&mut self) {
        self.0 = true;
    }

    pub fn value(&self) -> bool {
        self.0
    }

    pub fn ternary<T>(&self, ok: T, or: T) -> T {
        if self.0 { ok } else { or }
    }
}

impl From<bool> for Flag {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<Flag> for bool {
    fn from(value: Flag) -> Self {
        value.0
    }
}

pub struct Mixer {
    pulse1_flag: Flag,
    pulse2_flag: Flag,
    triangle_flag: Flag,
    noise_flag: Flag,
    dmc_flag: Flag,
}

impl Default for Mixer {
    fn default() -> Self {
        Mixer {
            pulse1_flag: true.into(),
            pulse2_flag: true.into(),
            triangle_flag: true.into(),
            noise_flag: true.into(),
            dmc_flag: true.into(),
        }
    }
}

impl Mixer {
    pub fn mix_output(&self, pulse1: u8, pulse2: u8, triangle: u8, noise: u8, dmc: u8) -> f32 {
        let pulse1 = self.pulse1_flag.ternary(pulse1 as f32, 0.0);
        let pulse2 = self.pulse2_flag.ternary(pulse2 as f32, 0.0);
        let triangle = self.triangle_flag.ternary(triangle as f32, 0.0);
        let noise = self.noise_flag.ternary(noise as f32, 0.0);
        let dmc = self.dmc_flag.ternary(dmc as f32, 0.0);

        let pulse_sum = pulse1 + pulse2;
        let pulse_out = if pulse_sum == 0.0 {
            0.0
        } else {
            95.88 / (8128.0 / pulse_sum + 100.0)
        };

        let tnd_sum = triangle / 8227.0 + noise / 12241.0 + dmc / 22638.0;

        let tnd_out = if tnd_sum == 0.0 {
            0.0
        } else {
            159.79 / (1.0 / tnd_sum + 100.0)
        };

        pulse_out + tnd_out
    }

    pub fn pulse1(&mut self) -> &mut Flag {
        &mut self.pulse1_flag
    }

    pub fn pulse2(&mut self) -> &mut Flag {
        &mut self.pulse2_flag
    }

    pub fn triangle(&mut self) -> &mut Flag {
        &mut self.triangle_flag
    }

    pub fn noise(&mut self) -> &mut Flag {
        &mut self.noise_flag
    }

    pub fn dmc(&mut self) -> &mut Flag {
        &mut self.dmc_flag
    }

    pub fn enable_all(&mut self) {
        self.pulse1_flag.set();
        self.pulse2_flag.set();
        self.triangle_flag.set();
        self.noise_flag.set();
        self.dmc_flag.set();
    }
}
