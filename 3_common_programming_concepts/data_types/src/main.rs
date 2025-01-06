use std::io;
fn main() {
    /*
    Use single quote for char literals
    */
    let _c = 'z';
    let _z: char = 'ℤ';
    let _heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("Five_hundred: {five_hundred}");
    println!("Six_point_four: {six_point_four}");
    println!("One: {one}");

    let _a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    /*
    array _a will contain 3 elements all set to 3 initially. 
    ie. [3, 3, 3, 3, 3]
     */
    let _a = [3; 5];

    println!("Please enter an array index.");
    
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index: {index} is: {element}");
}
