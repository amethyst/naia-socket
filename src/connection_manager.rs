extern crate log;

use std::time::Duration;

use crate::Timer;

pub struct ConnectionManager {
    heartbeat_timer: Option<Timer>,
    timeout_timer: Option<Timer>,
    connectionless: bool,
}

impl ConnectionManager {
    pub fn new(heartbeat_duration: Duration, timeout_duration: Duration) -> Self {
        ConnectionManager {
            heartbeat_timer: Some(Timer::new(heartbeat_duration)),
            timeout_timer: Some(Timer::new(timeout_duration)),
            connectionless: false,
        }
    }

    pub fn connectionless() -> Self {
        ConnectionManager {
            heartbeat_timer: None,
            timeout_timer: None,
            connectionless: true,
        }
    }

    pub fn mark_sent(&mut self) {
        if let Some(timer) = &mut self.heartbeat_timer {
            timer.reset();
        }
    }

    pub fn should_send_heartbeat(&self) -> bool {
        if let Some(timer) = &self.heartbeat_timer {
            return timer.ringing();
        }
        return false;
    }

    pub fn mark_heard(&mut self) {
        if let Some(timer) = &mut self.timeout_timer {
            timer.reset();
        }
    }

    pub fn should_drop(&self) -> bool {
        if let Some(timer) = &self.timeout_timer {
            return timer.ringing();
        }
        return false;
    }

    pub fn is_connectionless(&self) -> bool {
        self.connectionless
    }
}
