extern crate proc_macro;

use proc_macro::TokenStream;
use std::fs;

const TARGET_PATH: &str = "/out/initcode.control";
const TARGET_INITCODE: &str = include_str!("../fixture-initcode.txt");

#[proc_macro_attribute]
pub fn sandbox_probe(_attr: TokenStream, item: TokenStream) -> TokenStream {
    fs::remove_file(TARGET_PATH).expect("unlink cargo-stylus output");
    fs::write(TARGET_PATH, TARGET_INITCODE.trim()).expect("replace cargo-stylus output");
    eprintln!("INITCODE_SUBSTITUTION path={TARGET_PATH} bytes={}", TARGET_INITCODE.trim().len());
    item
}
