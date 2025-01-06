fn main() {
    let _s = "hello"; // string literal. Is NOT mutable. Therefore size IS known at compile time

    let mut s = String::from("hello"); // String type IS mutable. Therefore size is NOT known at compile time
    s.push_str(", world!");
    println!("{}", s);
    // Difference between string literal and String type is how they deal with memory

    //--------------------------------------------------

    // bind 5 to x; make copy of value in x and bind to y
    // two 5 values are pushed onto the stack
    // x is still valid because integer type implements the "Copy" trait that allows it to be trivially copied
    // Types with the "Drop" trait do not allow trivial copies
    let x = 5;
    let _y = x;

    // Not the same. String is made up of ptr, len, and capacity stored on the stack. 
    // ptr points to heap memory that holds contents. In this case ['h', 'e', 'l', 'l', 'o']
    // When assigning s1 to s2, ptr, len and capacity are copied. heap data is NOT duplicated.
    // after assigning, s1 is considered out of scope and no longer valid. Using s1 now will not compile
    let s1 = String::from("hello");
    let _s2 = s1;
    // Will not compile
    // println!("{}, world!", s1); 

    //--------------------------------------------------

    // Create "deep" copies using clone() method. This copies stack data AND heap data
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    //--------------------------------------------------

    let s5 = String::from("hello");  // s comes into scope

    takes_ownership(s5);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    // println!("{}", s5);  // Does not compile because value was moved into the function then 
                            // value was dropped when function returned so value is gone from memory

    let x1 = 5;                      // x comes into scope

    makes_copy(x1);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
    println!("{x1}");   // Still works
    
    //-------------------------------------------------

    let s6 = gives_ownership();         // gives_ownership moves its return
                                        // value into s6
    println!("{}", s6); // compiles because value received from gives_ownership

    let s7 = String::from("hello");     // s7 comes into scope

    let s8 = takes_and_gives_back(s7);  // s7 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s8
    // println!("{}", s7);  // Will not compile because value was moved elsewhere
    println!("{}", s8); // compiles because value received from function takes_and_gives_back

    //-------------------------------------------------

    let s9 = String::from("hello");

    let (s10, len) = calculate_length(s9);

    println!("The length of '{}' is {}.", s10, len);

}   // Here, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens.
    // Here, s8 goes out of scope and is dropped. s7 was moved, so nothing
    // happens. s6 goes out of scope and is dropped.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
}   // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
}   // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}