
#[macro_use]
extern crate log;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
pub mod main_common;

#[cfg(target_arch = "wasm32")]
mod app;

#[cfg(target_arch = "wasm32")]
mod app_wasm;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main_js() {
    main_common::main_common();
}