use std::time::{Duration, Instant};

pub struct Timer {
    duration: Duration,
    last: Instant,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Timer {
            last: Instant::now(),
            duration,
        }
    }

    pub fn reset(&mut self) {
        self.last = Instant::now();
    }

    pub fn ringing(&self) -> bool {
        Instant::now().duration_since(self.last) > self.duration
    }
}