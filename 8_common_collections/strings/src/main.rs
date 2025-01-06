fn main() {
    // Creating a new String --------------------------------------------
    let mut _s0 = String::new();

    let data = "inital contents";
    let _s1 = data.to_string();
    let _s2 = "initial contents".to_string();
    let _s3 = String::from("initial contents");
    // String::from and to_string do the same thing

    // Strings are UTF-8 encoded. All the folowwing are valid String values
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    // Updating a String ----------------------------------------------
    let mut s4 = String::from("foo");
    s4.push_str("bar"); // grow a String by using push_str method
    println!("s4 = {}", s4);
    let mut s5 = String::from("foo");
    let s6 = "bar";
    s5.push_str(s6); // push_str takes a string slice so it does not take ownership
    println!("s6 is {s6}"); // s6 still has ownership of "bar" so it can be used here

    let mut s7 = String::from("lo");
    s7.push('l'); // push method takes a SINGLE character as a parameter
    println!("s7 is {s7}");

    // Concatenation with the + Operator or the format! macro ------------
    let s8 = String::from("Hello, ");
    let s9 = String::from("world!");
    let s10 = s8 + &s9; // note s1 has been moved here and can no longer be used
    println!("s10 is {s10}. s9 is {s9}");
    // s8 is used directly and s9 is used with a reference because of how the + operator works
    // The + operator uses the add method (fn add(self, s:&str) -> String {...})
    // s9 is technically used as &String but can be coerced into &str[..] which matches the function signature
    let s11 = String::from("tic");
    let s12 = String::from("tac");
    let s13 = String::from("toe");
    let s14 = s11 + "-" + &s12 + "-" + &s13; // Difficult to see what's going on. Use format! instead
    println!("s14 is {s14}");
    let s11 = String::from("tic");
    let s12 = String::from("tac");
    let s13 = String::from("toe");
    let s15 = format!("{s11}-{s12}-{s13}"); // format! doesn't take ownership of any of its parameters
    println!("s15 is {s15}");

    // Indexing into Strings ------------------------------------------
    let _s16 = String::from("hello");
    // let h0 = s16[0]; 
    // indexing like this does not work in Rust because of the internal representation
    // String::from("Hola") => len will be 4. Each of these letters takes 1 byte when encoded in UTF-8
    // String::from("Здравствуйте") => len will be 24, not 12. each character takes 2 bytes when encoded in UTF-8.
    // Thus an index into the string's bytes will not always correlate to a valid Unicode scalar value

    // Slicing Strings -----------------------------------------------
    let hello = "Здравствуйте";
    let _s17 = &hello[0..4]; // s17 is a &str that contains the first 4 bytes of the string. 
                            // As each char is 2 bytes s17 is Зд. If range is not at a boundary, Rust will panic
    
    // Methods for iterating over Strings --------------------------------
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
