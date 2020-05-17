
#[macro_use]
extern crate log;

#[cfg(not(target_arch = "wasm32"))]
pub mod main_common;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    main_common::main_common();
}