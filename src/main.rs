
#[macro_use]
extern crate log;

#[cfg(not(target_arch = "wasm32"))]
mod main_common;

#[cfg(not(target_arch = "wasm32"))]
mod app_native;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    main_common::main_common();
}