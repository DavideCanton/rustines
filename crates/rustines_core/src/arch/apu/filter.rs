use std::f32::consts::PI;

pub trait Filter {
    fn process(&mut self, input: f32) -> f32;
}

pub struct HighPassFilter {
    previous_input: f32,
    previous_output: f32,
    alpha: f32,
}

impl HighPassFilter {
    pub fn new(cutoff: f32, sample_rate: f32) -> Self {
        let rc = 1.0 / (2.0 * PI * cutoff);
        let dt = 1.0 / sample_rate;

        Self {
            previous_input: 0.0,
            previous_output: 0.0,
            alpha: rc / (rc + dt),
        }
    }
}

impl Filter for HighPassFilter {
    fn process(&mut self, input: f32) -> f32 {
        let output = self.alpha * (self.previous_output + input - self.previous_input);

        self.previous_input = input;
        self.previous_output = output;

        output
    }
}

pub struct LowPassFilter {
    previous_output: f32,
    alpha: f32,
}

impl LowPassFilter {
    pub fn new(cutoff: f32, sample_rate: f32) -> Self {
        let rc = 1.0 / (2.0 * PI * cutoff);
        let dt = 1.0 / sample_rate;

        Self {
            previous_output: 0.0,
            alpha: dt / (rc + dt),
        }
    }
}

impl Filter for LowPassFilter {
    fn process(&mut self, input: f32) -> f32 {
        let output = self.alpha * input + (1.0 - self.alpha) * self.previous_output;
        self.previous_output = output;
        output
    }
}

pub struct SequenceFilter {
    filters: Vec<Box<dyn Filter>>,
}

impl SequenceFilter {
    pub fn new(filters: Vec<Box<dyn Filter>>) -> Self {
        Self { filters }
    }
}

impl Filter for SequenceFilter {
    fn process(&mut self, input: f32) -> f32 {
        let mut res = input;
        for f in self.filters.iter_mut() {
            res = f.process(res);
        }
        res
    }
}
