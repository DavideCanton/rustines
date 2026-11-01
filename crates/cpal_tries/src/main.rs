#![allow(unused)]
use std::{f32::consts::PI, thread::sleep, time::Duration};

use cpal::{
    BufferSize, Sample, StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};

fn phase_increment(target_freq: f32, sample_rate: u32) -> f32 {
    (2.0 * PI * target_freq) / sample_rate as f32
}

trait Generator {
    fn callback(&mut self, data: &mut [f32], info: &cpal::OutputCallbackInfo);
}

struct SineWaveGenerator {
    sample_clock: f32,
    phase_increment: f32,
    channels: Channels,
}

impl SineWaveGenerator {
    fn new(target_freq: f32, sample_rate: u32, channels: Channels) -> Self {
        Self {
            sample_clock: 0.0,
            phase_increment: phase_increment(target_freq, sample_rate),
            channels,
        }
    }
}

enum Channels {
    Left,
    Right,
    Both,
}

impl Generator for SineWaveGenerator {
    fn callback(&mut self, data: &mut [f32], _: &cpal::OutputCallbackInfo) {
        for frame in data.chunks_mut(2) {
            let value = self.sample_clock.sin();

            if matches!(self.channels, Channels::Left | Channels::Both) {
                frame[0] = value;
            } else {
                frame[0] = Sample::EQUILIBRIUM;
            }
            if matches!(self.channels, Channels::Right | Channels::Both) {
                frame[1] = value;
            } else {
                frame[1] = Sample::EQUILIBRIUM;
            }

            // frame[0..self.channels].fill(value);

            self.sample_clock += self.phase_increment;

            if self.sample_clock > 2.0 * PI {
                self.sample_clock -= 2.0 * PI;
            }
        }
    }
}

struct PulseWaveGenerator {
    sample_clock: f32,
    phase_increment: f32,
    channels: usize,
    threshold: f32,
    amplitude: f32,
}

impl PulseWaveGenerator {
    fn new(
        target_freq: f32,
        sample_rate: u32,
        channels: usize,
        duty_cycle: f32,
        amplitude: f32,
    ) -> Self {
        Self {
            sample_clock: 0.0,
            phase_increment: phase_increment(target_freq, sample_rate),
            channels,
            threshold: 2.0 * PI * duty_cycle,
            amplitude,
        }
    }
}

impl Generator for PulseWaveGenerator {
    fn callback(&mut self, data: &mut [f32], _: &cpal::OutputCallbackInfo) {
        for frame in data.chunks_mut(self.channels) {
            let value = (if self.sample_clock < self.threshold {
                1.0
            } else {
                -1.0
            }) * self.amplitude;

            frame[0..self.channels].fill(value);

            self.sample_clock += self.phase_increment;

            if self.sample_clock > 2.0 * PI {
                self.sample_clock -= 2.0 * PI;
            }
        }
    }
}

fn main() {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");

    let sample_rate = 8000;
    let channels = 2;
    let config = StreamConfig {
        channels: channels as u16,
        sample_rate,
        buffer_size: BufferSize::Default,
    };

    let mut gs = SineWaveGenerator::new(440.0, sample_rate, Channels::Left);
    // let mut gp = PulseWaveGenerator::new(380.0, sample_rate, channels, 0.5, 0.6);
    let mut gp = SineWaveGenerator::new(380.0, sample_rate, Channels::Right);

    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    let stream_s = device
        .build_output_stream(config, move |d, i| gs.callback(d, i), err_fn, None)
        .unwrap();

    let stream_p = device
        .build_output_stream(config, move |d, i| gp.callback(d, i), err_fn, None)
        .unwrap();

    stream_s.play().unwrap();
    stream_p.play().unwrap();

    sleep(Duration::from_secs(5));
}
