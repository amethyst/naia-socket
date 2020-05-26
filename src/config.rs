use std::{default::Default, time::Duration};

use crate::constants::{DEFAULT_MTU};

#[derive(Clone, Debug)]

pub struct Config {
    pub idle_connection_timeout: Duration,
    pub heartbeat_interval: Duration,
    pub receive_buffer_max_size: usize,
    pub socket_event_buffer_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            idle_connection_timeout: Duration::from_secs(10),
            heartbeat_interval: Duration::from_secs(4),
            receive_buffer_max_size: DEFAULT_MTU as usize,
            socket_event_buffer_size: 1024,
        }
    }
}