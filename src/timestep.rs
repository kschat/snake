use std::time::{Duration, Instant};

pub struct Timestep {
    last_time: Instant,
    delta_time: Duration,
    frame_time: Duration,
    frame_count: u16,
}

impl Timestep {
    pub fn new() -> Self {
        Self {
            last_time: Instant::now(),
            delta_time: Duration::from_millis(0),
            frame_time: Duration::from_millis(0),
            frame_count: 0,
        }
    }

    pub fn delta(&mut self) -> Duration {
        let now = Instant::now();
        self.delta_time = now.duration_since(self.last_time);
        self.last_time = now;

        self.delta_time
    }

    pub fn elapsed_time(&self) -> Duration {
        Instant::now().duration_since(self.last_time)
    }

    #[allow(dead_code)]
    pub fn frame_rate(&mut self) -> Option<u16> {
        self.frame_time += self.elapsed_time();
        self.frame_count += 1;

        if self.frame_time >= Duration::from_secs(1) {
            let frame_count = self.frame_count;
            self.frame_time = Duration::from_millis(0);
            self.frame_count = 0;

            return Some(frame_count);
        }

        None
    }
}
