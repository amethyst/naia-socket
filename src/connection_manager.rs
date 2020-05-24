extern crate log;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        /// WebRTC ///
        ///
        /// Date.now().getTime(),

        use std::time::Duration;
        use js_sys::Date;

        pub struct ConnectionManager {
            heartbeat_duration: f64,
            timeout_duration: f64,
            last_sent: f64,
            last_heard: f64,
        }

        impl ConnectionManager {
            pub fn new(heartbeat_duration: Duration, timeout_duration: Duration) -> Self {
                ConnectionManager {
                    last_sent: Date::now(),
                    last_heard: Date::now(),
                    heartbeat_duration: heartbeat_duration.as_millis() as f64,
                    timeout_duration: timeout_duration.as_millis() as f64,
                }
            }

            pub fn mark_sent(&mut self) {
                self.last_sent = Date::now();
            }

            pub fn should_send_heartbeat(&self) -> bool {
                (Date::now() - self.last_sent) > self.heartbeat_duration
            }

            pub fn mark_heard(&mut self) {
                self.last_heard = Date::now();
            }

            pub fn should_drop(&self) -> bool {
                (Date::now() - self.last_heard) > self.timeout_duration
            }
        }
    }
    else {
        /// UDP ///
        use std::time::{Duration, Instant};

        pub struct ConnectionManager {
            heartbeat_duration: Duration,
            last_sent: Instant,
            timeout_duration: Duration,
            last_heard: Instant,
        }

        impl ConnectionManager {
            pub fn new(heartbeat_duration: Duration, timeout_duration: Duration) -> Self {
                ConnectionManager {
                    last_sent: Instant::now(),
                    heartbeat_duration,
                    last_heard: Instant::now(),
                    timeout_duration,
                }
            }

            pub fn mark_sent(&mut self) {
                self.last_sent = Instant::now();
            }

            pub fn should_send_heartbeat(&self) -> bool {
                Instant::now().duration_since(self.last_sent) > self.heartbeat_duration
            }

            pub fn mark_heard(&mut self) {
                self.last_heard = Instant::now();
            }

            pub fn should_drop(&self) -> bool {
                Instant::now().duration_since(self.last_heard) > self.timeout_duration
            }
        }
    }
}



