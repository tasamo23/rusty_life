extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;

pub mod universe;

pub mod cell {
    #[repr(u8)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Cell {
        Dead = 0,
        Alive = 1,
    }
}

#[wasm_bindgen]
extern "C" {}
