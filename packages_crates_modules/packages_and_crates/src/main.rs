// Crate is the smallest amount of code that the Rust compiler considers at a time
// 2 forms: binary crate or library crate

// Binary crates are programs that compile to a runnable executable
// Must have main function

// Library crates do not have a main function and do not compile to an executable
// They define functionaliuty intended to be shared with multiple projects.
// Example: "rand" crate used to generate random numbers
// To Rust enjoyers, "crate" usually means library crate. "crate" is interchangable with "library"

// Package is a bundle of one or more crates that provides functionality.
// Contains a Cargo.toml file.
// A package must contain at least one crate. It can contain any number of binary crates but only one library crate.

// Running cargo new creates a Cargo.toml file, which is a package.
// Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package
// Cargo knows that if package directory contains src/lib.rs, the package contains a library crate with
// the same name as package and src/lib.rs as its crate root

// A package can have multiple binary crates by placing files in the src/bin directory: each file will 
// be a separate binary crate

fn main() {
    println!("Read the comments");
}
