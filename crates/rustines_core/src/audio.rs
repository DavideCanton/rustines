pub trait AudioOutput {
    fn push_sample(&mut self, sample: f32);
    fn sample_rate(&self) -> u64;
}

pub struct NoAudio;

impl AudioOutput for NoAudio {
    fn push_sample(&mut self, _sample: f32) {}
    fn sample_rate(&self) -> u64 {
        0
    }
}

pub type AudioOutputBox = Box<dyn AudioOutput>;
