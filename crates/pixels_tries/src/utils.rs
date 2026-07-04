use std::{
    thread::sleep,
    time::{Duration, Instant},
};

pub struct FpsLimiter {
    frame_target_duration: Duration,
    last_frame_time: Instant,
}

impl FpsLimiter {
    pub fn new(target_fps: f64) -> Self {
        Self {
            frame_target_duration: Duration::from_secs_f64(1.0 / target_fps),
            last_frame_time: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        let elapsed = self.last_frame_time.elapsed();
        if elapsed < self.frame_target_duration {
            sleep(self.frame_target_duration - elapsed);
        }
        self.last_frame_time = Instant::now();
    }
}

pub struct FpsCounter {
    last_fps_check: Instant,
    frame_count: u64,
}

impl FpsCounter {
    pub fn new() -> Self {
        FpsCounter {
            last_fps_check: Instant::now(),
            frame_count: 0,
        }
    }

    pub fn drawn(&mut self) -> Option<f64> {
        self.frame_count += 1;
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_fps_check);

        if elapsed >= Duration::from_secs(1) {
            let current_fps = self.frame_count as f64 / elapsed.as_secs_f64();
            self.frame_count = 0;
            self.last_fps_check = now;

            Some(current_fps)
        } else {
            None
        }
    }
}
