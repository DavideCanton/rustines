use std::time::{Duration, Instant};

/// Helper to count the fps at which a gui application is running.
///
/// # Usage
///
/// - create a new instance of `FpsCounter` using `FpsCounter::new`
/// - call the `update` method every time a frame is completed
/// - read the current fps value from `current_fps()`.
pub struct FpsCounter {
    last_fps_check: Instant,
    frame_count: u64,
    current_fps: f64,
}

impl FpsCounter {
    /// Creates a new instance of `FpsCounter`.
    pub fn new() -> Self {
        FpsCounter {
            last_fps_check: Instant::now(),
            frame_count: 0,
            current_fps: 0.0,
        }
    }

    /// Function to be invoked when a frame is ended.
    pub fn update(&mut self) {
        self.frame_count += 1;
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_fps_check);

        if elapsed >= Duration::from_secs(1) {
            let current_fps = self.frame_count as f64 / elapsed.as_secs_f64();
            self.frame_count = 0;
            self.last_fps_check = now;
            self.current_fps = current_fps;
        }
    }

    pub fn current_fps(&self) -> f64 {
        self.current_fps
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
    fn test_update() {
        // T
        let mut counter = FpsCounter::new();

        counter.update(); // 1

        for _ in 0..3 {
            counter.update();
            sleep(Duration::from_millis(100));
        }

        // T = 300ms
        // count = 4

        sleep(Duration::from_secs(1));

        // T = 1300ms
        counter.update();

        // count = 5

        // fps ~= 5 / 1.3
        let exp = 5.0 / 1.3;
        let diff = (counter.current_fps() - exp).abs();
        assert!(diff < 0.1, "diff {} above the threshold", diff);
    }
}
