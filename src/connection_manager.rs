use std::time::{Duration, Instant};

pub struct ConnectionManager {
    heartbeat_interval: Duration,
    last_sent: Instant,
}

impl ConnectionManager {
    pub fn new(heartbeat_interval: Duration) -> Self {
        ConnectionManager {
            last_sent: Instant::now(),
            heartbeat_interval,
        }
    }

    pub fn mark_sent(&mut self) {
        self.last_sent = Instant::now();
    }

    pub fn should_send_heartbeat(&self) -> bool {
        Instant::now().duration_since(self.last_sent) > self.heartbeat_interval
    }
}