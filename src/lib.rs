
#[macro_use]
extern crate cfg_if;

pub mod find_available_port;
pub mod find_my_ip_address;

mod constants;
pub use constants::{MessageHeader, DEFAULT_MTU};

mod config;
pub use config::Config;

mod utils;
pub use utils::{StringUtils};

mod connection_manager;
pub use connection_manager::ConnectionManager;

mod timer;
pub use timer::Timer;

mod instant;
pub use instant::Instant;

mod duration;
pub use duration::Duration;