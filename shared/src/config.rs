use std::{default::Default, time::Duration};

/// A Config object required to initialize a given Server/Client Socket
#[derive(Clone, Debug)]
pub struct Config {
    /// The time to wait before the socket will emit a Tick event
    pub tick_interval: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tick_interval: Duration::from_secs(1),
        }
    }
}
