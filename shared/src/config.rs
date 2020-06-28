use std::{default::Default, time::Duration};

#[derive(Clone, Debug)]

pub struct Config {
    pub tick_interval: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tick_interval: Duration::from_secs(1),
        }
    }
}
