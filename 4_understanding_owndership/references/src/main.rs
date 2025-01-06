fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("the length of '{}' is {}.", s1, len);

    // let s = String::from("hello");
    // change(&s); // Does not work. References are immutable by default

    let mut s = String::from("hello");
    println!("s before change(): {}", s);
    change(&mut s);
    println!("s after change(): {}", s);

    // Can only have ONE mutable reference to a value. Any more will fail
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    {
        let r1 = &mut s;
        println!("Inner scope r1: {}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems
    let r2 = &mut s;
    println!("Outer scope r2: {}", r2);

    // Cannot have mutable and immutable borrows
    // Multiple immutable references are allowed because value will not change. essentially read-only
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point so they are no longer in scope

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // Rust does not allow dangling pointers to compile. Safety issue
    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// fn change(some_string: &String) {
//     some_string.push_str(",world"); // Does not work because do not have ownership
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference tothe String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger! Reference is pointing to an invalid String

fn no_dangle() -> String {
    let s = String::from("hello");
    s
} // Solution is to return String directly thus ownership of value is moved out
  // and nothing is deallocated

  
// RULES OF REFERENCES
// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.