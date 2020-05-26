use std::{default::Default, time::Duration};

#[derive(Clone, Debug)]

pub struct Config {
    pub tick_interval: Duration,
    pub connection_events_enabled: bool,
    pub send_handshake_interval: Duration,
    pub disconnection_events_enabled: bool,
    pub disconnection_timeout_duration: Duration,
    pub heartbeats_enabled: bool,
    pub heartbeat_interval: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tick_interval: Duration::from_secs(1),
            connection_events_enabled: true,
            disconnection_events_enabled: true,
            disconnection_timeout_duration: Duration::from_secs(10),
            heartbeats_enabled: true,
            heartbeat_interval: Duration::from_secs(4),
            send_handshake_interval: Duration::from_secs(1),
        }
    }
}