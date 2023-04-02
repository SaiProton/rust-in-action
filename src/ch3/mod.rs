#![allow(dead_code)]
mod fileio;
mod globalerror;
mod parse_log;
mod traits;

pub fn main() {
    fileio::main();
    // globalerror::main();
    // parse_log::main();
    // traits::main();
}
