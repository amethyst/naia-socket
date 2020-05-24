extern crate log;
use log::info;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        /// WebRTC ///
        ///
        /// Date.now().getTime(),

        use std::time::Duration;
        use js_sys::Date;

        pub struct ConnectionManager {
            heartbeat_interval: f64,
            last_sent: f64,
        }

        impl ConnectionManager {
            pub fn new(heartbeat_interval: Duration) -> Self {
                ConnectionManager {
                    last_sent: Date::now(),
                    heartbeat_interval: heartbeat_interval.as_millis() as f64,
                }
            }

            pub fn mark_sent(&mut self) {
                self.last_sent = Date::now();
            }

            pub fn should_send_heartbeat(&self) -> bool {
                //let now = Date::now();
                //info!("testing heartbeat. last: {}, now: {}, difference: {}", self.last_sent, now, self.last_sent - now);
                (Date::now() - self.last_sent) > self.heartbeat_interval
            }
        }
    }
    else {
        /// UDP ///
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
    }
}



