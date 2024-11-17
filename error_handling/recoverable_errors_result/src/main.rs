use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // Matching on Different Errors --------------------------------------

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    // Shortcuts for Panic on Error: unwrap and expect ------------------------------------

    // unwrap() returns value in case of Ok variant or panics in case of Err variant
    // let greeting_file = File::open("hello.txt").unwrap(); 

    // expect() method allows to provide verbose error message and intent that make tracking error easier
    // Use expect as it is better
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    // Propagating Errors -----------------------------------------

    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // A Shortcut for Propagating Errors: the ? Operator -----------------------------------

    fn read_username_from_file() -> Result<String, io::Error> {
        // the "?" works the same way as the match in previous function except converts err type received to err type of current function
        // Useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons
        let mut username_file = File::open("hello.txt")?; 
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("hello.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }

    // Side note ----------------------------------------------------

    /*
    Same code without match statements. Uses unwrap_or_else method taught in Chapter 13

    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    */
}
