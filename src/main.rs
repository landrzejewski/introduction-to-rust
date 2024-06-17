/*
https://github.com/landrzejewski/introduction-to-rust

Installation/environment setup
- rustup tool from https://rustup.rs
- Visual Code + Rust extension, alternatively RustRover
- git

Important commands:
rustup --version                       # check rustup and rustc version
rustc main.rs                          # compile a file
rustfmt main.rs                        # format source a file
cargo new training_project             # create new project with cargo tool
cargo build                            # build an application in debug mode
cargo run                              # build and run an application in debug mode
cargo build --release                  # build an application in release mode
cargo check                            # check/build code without generating executables
cargo fmt                              # format source files in the project
*/

#![allow(dead_code)]

// crate scope attribute (entire application)
mod language_basics;
mod memory_management;
mod exercises;


fn main() {
    println!("Hello World in Rust");

    // language_basics::run();
    // fib::run();
    exercises::money_with_enums::run();
}
