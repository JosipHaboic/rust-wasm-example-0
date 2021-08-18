#![allow(unused_unsafe)]
mod utils;
use ueight;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn byte_repr(byte: u8, reverse: bool) -> String {
    ueight::convert::byte_repr(byte, reverse).iter().collect()
}