extern crate log;

use std::time::Duration;

use crate::Timer;

pub struct ConnectionManager {
    heartbeat_timer: Option<Timer>,
    timeout_timer: Option<Timer>,
}

impl ConnectionManager {
    pub fn new(heartbeat_duration: Duration, timeout_duration: Duration) -> Self {
        ConnectionManager {
            heartbeat_timer: Some(Timer::new(heartbeat_duration)),
            timeout_timer: Some(Timer::new(timeout_duration)),
        }
    }

    pub fn connectionless() -> Self {
        ConnectionManager {
            heartbeat_timer: None,
            timeout_timer: None,
        }
    }

    pub fn mark_sent(&mut self) {
        self.heartbeat_timer.as_mut().unwrap().reset();
    }

    pub fn should_send_heartbeat(&self) -> bool {
        self.heartbeat_timer.as_ref().unwrap().ringing()
    }

    pub fn mark_heard(&mut self) {
        self.timeout_timer.as_mut().unwrap().reset();
    }

    pub fn should_drop(&self) -> bool {
        self.timeout_timer.as_ref().unwrap().ringing()
    }
}
