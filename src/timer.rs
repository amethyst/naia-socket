
cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        /// Wasm ///

        use std::time::Duration;
        use js_sys::Date;

        pub struct Timer {
            duration: f64,
            last: f64,
        }

        impl Timer {
            pub fn new(duration: Duration) -> Self {
                Timer {
                    last: Date::now(),
                    duration: duration.as_millis() as f64,
                }
            }

            pub fn reset(&mut self) {
                self.last = Date::now();
            }

            pub fn ringing(&self) -> bool {
                (Date::now() - self.last) > self.duration
            }
        }
    }
    else {
        /// Linux ///
        use std::time::{Duration, Instant};

        pub struct Timer {
            duration: Duration,
            last: Instant,
        }

        impl Timer {
            pub fn new(duration: Duration) -> Self {
                Timer {
                    last: Instant::now(),
                    duration,
                }
            }

            pub fn reset(&mut self) {
                self.last = Instant::now();
            }

            pub fn ringing(&self) -> bool {
                Instant::now().duration_since(self.last) > self.duration
            }
        }
    }
}