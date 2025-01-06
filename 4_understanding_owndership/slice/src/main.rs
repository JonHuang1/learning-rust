fn main() {
    let mut s = String::from("hello world");
    let _word = old_first_word(&s); // word will get the value 5
    s.clear(); // this empties the String, making it equal to ""
    // word still has the vlaue 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    //--------------------------------------------------

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("slice1: {}", hello);
    println!("slice2: {}", world);

    /* Rust's Range syntax notes
    &s[0..2] is the same as &s[..2]
    len = s.len();
    &s[3..len] is the same as &s[3..]
    &s[0..len] is the same as &s[..] */

    let first_word_s = first_word(&s);
    println!("first word: {}", first_word_s);
    let second_word_s = second_word(&s);
    println!("second word: {}", second_word_s);

    let s0 = String::from("hello");
    let s1 = String::from("hello world hola earth");
    let second_word_s0 = second_word(&s0);
    println!("second word in s0: {}", second_word_s0);
    let second_word_s1 = second_word(&s1);
    println!("second word in s1: {}", second_word_s1);

    //------------------------------------------------

    //String literals

    let s = "Hello, world!"; // s is type &str. string literals are immutable. &str is an immutable reference

    //-------------------------------------------------

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    //--------------------------------------------------

    // Other Slices

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

// Task: write a function that takes a string of words separated by spaces 
// and returns the first word it finds in that string. 
// If the function doesn’t find a space in the string, the whole string must be one word, 
// so the entire string should be returned.
fn old_first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // converts String to an arroy of bytes
    for (i, &item) in bytes.iter().enumerate() { // Creates iterator over the array of bytes
        if item == b' ' { // search for the byte that represents a space by useing byte literal syntax
            return i; // if space is found, return the position
        } // else return length of the string with s.len()
    }
    s.len()
}

// fn new_first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str { // changing parameter to &str allows function to work 
                                     // with both &String and &str values, 
                                     // making it more general and useful without losing any functionality
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut start = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if start == 0 { // executes where there are two or more words and the first space is reached
                start = i + 1;
            } else { // executes when there are three or more words.
                return &s[start..i]; // Returns from after the first space to second space. ie second word
            } 
        }
    }
    &s[start..] // if there is only one word, return entire word
}