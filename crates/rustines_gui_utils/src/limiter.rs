use std::{
    thread::sleep,
    time::{Duration, Instant},
};

/// Helper that limits the number of frames rendered to match a target FPS.
///
/// # Usage
///
/// - create a new `FpsLimiter` using `FpsLimiter::new(target_fps)`
/// - call its `update` method everytime the application is updated. This sleeps if the application is faster than the target fps.
pub struct FpsLimiter {
    pub(super) frame_target_duration: Duration,
    pub(super) last_frame_time: Instant,
}

impl FpsLimiter {
    /// Creates a new `FpsLimiter` that matches the provided `target_fps`.
    pub fn new(target_fps: f64) -> Self {
        Self {
            frame_target_duration: Duration::from_secs_f64(1.0 / target_fps),
            last_frame_time: Instant::now(),
        }
    }

    /// Callback that should be invoked everytime the application logic is updated.
    ///
    /// Sleeps if the elapsed time since the last invocation is less than `1 / target_fps`, to ensure the
    /// application doesn't run too much fast.
    pub fn update(&mut self) {
        let elapsed = self.last_frame_time.elapsed();
        if elapsed < self.frame_target_duration {
            sleep(self.frame_target_duration - elapsed);
        }
        self.last_frame_time = Instant::now();
    }
}

#[cfg(test)]
mod tests {
    use std::{
        thread::sleep,
        time::{Duration, Instant},
    };

    use crate::FpsLimiter;

    #[test]
    fn test_new() {
        let limiter = FpsLimiter::new(42.0);
        assert_eq!(
            limiter.frame_target_duration,
            Duration::from_secs_f64(1.0 / 42.0)
        );
        assert_eq!(limiter.last_frame_time.elapsed().as_secs(), 0);
    }

    #[test]
    fn test_update() {
        let mut limiter = FpsLimiter::new(5.0);
        let frac = limiter.frame_target_duration.as_secs_f64();
        assert!(frac < 0.3);

        let mut fun = || limiter.update();

        // little time has passed, so update waits
        assert!(time(&mut fun) >= frac);
        assert!(time(&mut fun) >= frac);

        sleep(Duration::from_millis(300));
        // more time has passed so update does not wait
        assert!(time(&mut fun) < frac);

        // ensure last frame time has been updated
        assert!(time(&mut fun) >= frac);
    }

    fn time(mut fun: impl FnMut()) -> f64 {
        let instant = Instant::now();
        fun();
        instant.elapsed().as_secs_f64()
    }
}
