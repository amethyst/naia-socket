cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        // Wasm //

        use std::time::Duration;
        use js_sys::Date;

        /// A Timer with a given duration after which it will enter into a "Ringing" state.
        /// The Timer can be reset at an given time, or manually set to start "Ringing" again.
        #[derive(Debug)]
        pub struct Timer {
            duration: f64,
            last: f64,
        }

        impl Timer {
            /// Creates a new Timer with a given Duration
            pub fn new(duration: Duration) -> Self {
                Timer {
                    last: Date::now(),
                    duration: duration.as_millis() as f64,
                }
            }

            /// Reset the Timer to stop ringing and wait till 'Duration' has elapsed again
            pub fn reset(&mut self) {
                self.last = Date::now();
            }

            /// Gets whether or not the Timer is "Ringing" (i.e. the given Duration has elapsed
            /// since the last "reset")
            pub fn ringing(&self) -> bool {
                (Date::now() - self.last) > self.duration
            }

            /// Manually causes the Timer to enter into a "Ringing" state
            pub fn ring_manual(&mut self) {
                self.last -= self.duration;
            }
        }
    }
    else {
        // Linux //
        use std::time::{Duration, Instant};

        /// A Timer with a given duration after which it will enter into a "Ringing" state.
        /// The Timer can be reset at an given time, or manually set to start "Ringing" again.
        #[derive(Debug)]
        pub struct Timer {
            duration: Duration,
            last: Instant,
        }

        impl Timer {
            /// Creates a new Timer with a given Duration
            pub fn new(duration: Duration) -> Self {
                Timer {
                    last: Instant::now(),
                    duration,
                }
            }

            /// Reset the Timer to stop ringing and wait till 'Duration' has elapsed again
            pub fn reset(&mut self) {
                self.last = Instant::now();
            }

            /// Gets whether or not the Timer is "Ringing" (i.e. the given Duration has elapsed
            /// since the last "reset")
            pub fn ringing(&self) -> bool {
                Instant::now().saturating_duration_since(self.last) > self.duration
            }

            /// Manually causes the Timer to enter into a "Ringing" state
            pub fn ring_manual(&mut self) {
                self.last -= self.duration;
            }
        }
    }
}
