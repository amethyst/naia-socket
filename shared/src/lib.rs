//! # Naia Socket Shared
//! Common data types shared between Naia Server Socket & Naia Client Socket

#![deny(missing_docs,
    missing_debug_implementations,
    trivial_casts, trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces, unused_qualifications)]

#[macro_use]
extern crate cfg_if;

mod find_available_port;
pub use find_available_port::find_available_port;

mod find_my_ip_address;
pub use find_my_ip_address::find_my_ip_address;

mod config;
pub use config::Config;

mod timer;
pub use timer::Timer;
