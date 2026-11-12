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

#[cfg(test)]
mod tests {
    use num_complex::Complex;
    use rustfft::FftPlanner;

    use super::*;
    use std::f32::consts::PI;

    struct Freq {
        value: usize,
        passes: bool,
    }

    impl Freq {
        fn pass(value: usize) -> Self {
            Self {
                value,
                passes: true,
            }
        }

        fn filtered(value: usize) -> Self {
            Self {
                value,
                passes: false,
            }
        }
    }

    #[test]
    fn test_high_pass_filter_frequencies() {
        run_filter_test(
            HighPassFilter::new(150.0, 44100.0),
            44100.0,
            44100,
            &[Freq::filtered(10), Freq::pass(1000)],
            0.7,
            0.3,
        );
    }

    #[test]
    fn test_low_pass_filter_frequencies() {
        run_filter_test(
            LowPassFilter::new(150.0, 44100.0),
            44100.0,
            44100,
            &[Freq::pass(10), Freq::filtered(1000)],
            0.7,
            0.3,
        );
    }

    #[test]
    fn test_sequence_filter_frequencies() {
        run_filter_test(
            SequenceFilter::new(vec![
                Box::new(HighPassFilter::new(150.0, 44100.0)),
                Box::new(LowPassFilter::new(650.0, 44100.0)),
            ]),
            44100.0,
            44100,
            &[Freq::filtered(10), Freq::pass(500), Freq::filtered(2000)],
            0.7,
            0.3,
        );
    }

    fn run_filter_test<F: Filter>(
        mut filter: F,
        sample_rate: f32,
        num_samples: usize,
        frequencies: &[Freq],
        pass_threshold: f32,
        filter_threshold: f32,
    ) {
        let mut output = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            let t = i as f32 / sample_rate;
            let input = frequencies
                .iter()
                .map(|f| (2.0 * PI * (f.value as f32) * t).sin())
                .sum();
            output.push(filter.process(input));
        }

        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(num_samples);

        let mut buffer: Vec<Complex<f32>> =
            output.iter().map(|&val| Complex::new(val, 0.0)).collect();

        fft.process(&mut buffer);

        for f in frequencies {
            let mag = (buffer[f.value].norm() * 2.0) / (num_samples as f32);
            if f.passes {
                assert!(
                    mag > pass_threshold,
                    "{} > {} [{}Hz]",
                    mag,
                    pass_threshold,
                    f.value
                );
            } else {
                assert!(
                    mag < filter_threshold,
                    "{} < {} [{}Hz]",
                    mag,
                    filter_threshold,
                    f.value
                );
            }
        }
    }
}
