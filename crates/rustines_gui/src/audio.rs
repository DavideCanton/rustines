use cpal::{
    ErrorKind, Sample, Stream,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use ringbuf::{
    HeapProd, HeapRb,
    traits::{Consumer, Producer, Split},
};
use rustines_core::AudioOutput;

pub struct CpalOutput {
    sample_rate: u64,
    producer: HeapProd<f32>,
    stream: Stream,
}

impl CpalOutput {
    pub fn new() -> Result<Self, cpal::Error> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");

        let config = device.default_output_config()?;
        let sample_rate = config.sample_rate() as u64;
        let channels = config.channels() as usize;

        let ring_buffer = HeapRb::<f32>::new(8192);

        let (producer, mut consumer) = ring_buffer.split();

        let stream = device.build_output_stream(
            config.into(),
            move |data: &mut [f32], _| {
                for frame in data.chunks_mut(channels) {
                    let sample = consumer.try_pop().unwrap_or(Sample::EQUILIBRIUM);

                    for channel in frame.iter_mut() {
                        *channel = sample;
                    }
                }
            },
            |err| {
                if err.kind() != ErrorKind::StreamInvalidated {
                    eprintln!("Audio error: {err:?}");
                }
            },
            None,
        )?;

        Ok(Self {
            sample_rate,
            producer,
            stream,
        })
    }

    pub fn play(&mut self) -> Result<(), cpal::Error> {
        self.stream.play()
    }
}

impl AudioOutput for CpalOutput {
    fn push_sample(&mut self, sample: f32) {
        let _ = self.producer.try_push(sample);
    }

    fn sample_rate(&self) -> u64 {
        self.sample_rate
    }
}
