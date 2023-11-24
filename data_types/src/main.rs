fn main() {
    /*
    Use single quote for char literals
    */
    let _c = 'z';
    let _z: char = 'ℤ';
    let _heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
}
