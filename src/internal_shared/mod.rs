use log;

#[cfg(all(feature = "Logging", not(target_arch = "wasm32")))]
use simple_logger;

#[cfg(all(feature = "Logging", target_arch = "wasm32"))]
use web_logger;

pub(crate) const SERVER_PORT: &str = "3179";
pub(crate) const PING_MSG: &str = "ping";
pub(crate) const PONG_MSG: &str = "pong";

pub fn init() {
    #[cfg(feature = "Logging")]
    set_logger(log::Level::Info);
}

#[cfg(feature = "Logging")]
fn set_logger(level: log::Level) {
    #[cfg(all(feature = "Logging", target_arch = "wasm32"))]
    web_logger::custom_init(web_logger::Config { level });

    #[cfg(all(feature = "Logging", not(target_arch = "wasm32")))]
    simple_logger::init_with_level(level).expect("A logger was already initialized");
}
