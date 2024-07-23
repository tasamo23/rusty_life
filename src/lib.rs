extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;

pub mod universe;

#[wasm_bindgen]
extern "C" {}

extern crate web_sys;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}
