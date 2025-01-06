fn main() {
    // Creating a new Hash Map -----------------------------------------------
    use std::collections::HashMap;
    
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accessing values in a hash map ----------------------------------------
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); 
    println!("Blue score = {score}");
    // .get() returns Option<&V> 
    // .copied() gets an Option<i32> rather than an Option<&i32> 
    // .unwrap_or() sets score to zero if scores doesn't have an entry for the key

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Hash Maps and ownership ---------------------------------------------
    // types that implement the Copy trait, like i32, values are copied into the hash map
    // for owned values like String, values will be moved and the hash map will be the owner of those values
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point. Will cause compiler error if used

    // Updating a Hash Map ----------------------------------------------
    // Overwriting a value --------
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
    // Adding a key and value only if a key isn't present ------
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    // .entry() returns Entry enum
    // .or_insert() method returns mutable reference to value of corresponding Entry key if exists, 
    // if not, inserts parameter as new value
    println!("{:?}", scores);

    // Updating a value based on old value ------
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // or_insert returns reference to value of word so must use deref
    }
    
    println!("{:?}", map);
}
