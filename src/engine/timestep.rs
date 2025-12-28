use std::time::{Duration, Instant};

pub struct Timestep {
    last_time: Instant,
    delta_time: Duration,
    frame_time: Duration,
    frame_count: u16,
    pub frame_rate: u16,
}

impl Timestep {
    const ONE_SECOND: Duration = Duration::from_secs(1);

    pub fn new() -> Self {
        Self {
            last_time: Instant::now(),
            delta_time: Duration::from_millis(0),
            frame_time: Duration::from_millis(0),
            frame_count: 0,
            frame_rate: 0,
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

    pub fn track_frame(&mut self) -> Option<u16> {
        self.frame_time += self.elapsed_time();
        self.frame_count += 1;

        if self.frame_time < Self::ONE_SECOND {
            return None;
        }

        self.frame_rate = self.frame_count;
        self.frame_time = Duration::from_millis(0);
        self.frame_count = 0;

        Some(self.frame_rate)
    }
}
