use std::time::{Duration, Instant};

/// Helper to count the fps at which a gui application is running.
///
/// # Usage
///
/// - create a new instance of `FpsCounter` using `FpsCounter::new`
/// - call the `drawn` method every time the window is drawn. The return value, if not `None`, is the number of frames rendered since the last call.
pub struct FpsCounter {
    pub(super) last_fps_check: Instant,
    pub(super) frame_count: u64,
}

impl FpsCounter {
    /// Creates a new instance of `FpsCounter`.
    pub fn new() -> Self {
        FpsCounter {
            last_fps_check: Instant::now(),
            frame_count: 0,
        }
    }

    /// Callback that should be invoked when the window is drawn.
    ///
    /// Returns a `f64` containing the current FPS, if at least a second has passed since the
    /// last non-None timestamp (or the counter creation), else `None`.
    pub fn drawn(&mut self) -> Option<f64> {
        self.frame_count += 1;
        let now: Instant = Instant::now();
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

impl Default for FpsCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use crate::FpsCounter;

    #[test]
    fn test_new() {
        let counter = FpsCounter::new();
        assert_eq!(counter.frame_count, 0);
        assert_eq!(counter.last_fps_check.elapsed().as_secs(), 0);
    }

    #[test]
    fn test_default() {
        let counter = FpsCounter::default();
        let counter2 = FpsCounter::new();

        assert_eq!(counter.frame_count, counter2.frame_count);
        assert_eq!(
            counter
                .last_fps_check
                .duration_since(counter2.last_fps_check)
                .as_secs(),
            0
        );
    }

    #[test]
    fn test_drawn() {
        // T
        let mut counter = FpsCounter::new();

        let res = counter.drawn(); // 1
        assert!(res.is_none());

        for _ in 0..3 {
            counter.drawn();
            sleep(Duration::from_millis(100));
        }

        // T = 300ms
        // count = 4

        sleep(Duration::from_secs(1));

        // T = 1300ms
        let res = counter.drawn().unwrap();

        // count = 5

        // fps ~= 5 / 1.3
        let exp = 5.0 / 1.3;
        let diff = (res - exp).abs();
        assert!(diff < 0.1, "diff {} above the threshold", diff);
    }
}
