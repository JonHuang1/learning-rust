// When importing functions with "use", specify down to the parent module
// example:

// When importing structs, enums and other items, specify the full path
use std::collections::HashMap;
// Exception is when bringing two items with same name
use std::fmt;
use std::io;
// Solution to same name is to use "as"
use std::fmt::Result;
use std::io::Result as IoResult;

// Steps to using External packages
// 1. listing them in Cargo.toml (rand = "0.8.5")
// 2. use "use" to bring items from their crates into scope

// Use neted paths to clean up large "use" lists
// THE FOLLOWING...
// use std::cmp::Ordering;
// use std::io;
// CAN BE CONVERTED TO THIS:
// use std::{cmp::Ordering, io};
// THE FOLLOWING...
// use std::io;
// use std::io::Write;
// CAN BE CONVERTED TO THIS:
// use std::io::{self, Write};

// Glob operator
// To bring all public items defined in a path into scope, use *
// use std::collections::*;
// This can make it hard to tell what names are in scope and where a name was defined
// Usually used to bring everything under test into the "tests" module

fn main() {
    println!("Read both main.rs and lib.rs");
    // Example 2
    let mut map = HashMap::new();
    map.insert(1, 2);

}

// Two different Result types with different parent modules
// fn function1() -> fmt::Result {}
// fn function2() -> io::Result<()> {}

// Updated functions
// fn function1() -> Result {}
// fn function2() -> IoResult<()> {}