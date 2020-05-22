use std::{default::Default, time::Duration};

use crate::constants::{DEFAULT_MTU};

#[derive(Clone, Debug)]

pub struct Config {
    pub idle_connection_timeout: Duration,
    pub heartbeat_interval: Option<Duration>,
    pub receive_buffer_max_size: usize,
    pub socket_event_buffer_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            idle_connection_timeout: Duration::from_secs(5),
            heartbeat_interval: None,
            receive_buffer_max_size: DEFAULT_MTU as usize,
            socket_event_buffer_size: 1024,
        }
    }
}