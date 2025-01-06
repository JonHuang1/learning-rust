fn main() {
    // Declaring empty vector. Must add type annotation if starting empty so Rust knows what is intended
    let _v0: Vec<i32> = Vec::new();

    // Declaring vector using vec! macro
    let v1 = vec![1, 2, 3];
    println!("vector v1: {:?}", v1);

    // Updating vector
    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);
    println!("vector v2: {:?}", v2);

    // Reading elements
    let v3 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v3[2];
    println!("The third element is {third}");
    let third: Option<&i32> = v3.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    if let Some(third) = third {println!("The third element is {third} (if let practice)")}

    /*
    let v = vec![1, 2, 3, 4, 5];
    // This causes panic. Best used when you want program to crash if there is attempt to access element past the end
    let does_not_exist = &v[100]; 
    // This returns None type of Option<T> enum. Best used if accessing element beyond range of 
    // vector may happen occasionally under normal circumstances. Code should have logic handing Some(&element) or None
    // Example: user inputs index. If index too large, program gets None value, retry getting new input
    let does_not_exist = v.get(100); 
    */

    // Iterating over values
    let v4 = vec![100, 32, 57];
    for i in &v4 {
        println!("{i}");
    }

    let mut v5 = vec![100, 32, 57];
    for i in &mut v5 {
        *i += 50;
        println!("{i}");
    }

    // Using an enum to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Dropping a vector drops its elements
    {
        let v6 = vec![1, 2, 3, 4];
        println!("v6 = {:?}", v6);
    } // <- v6 goes out of scope and is freed here
}