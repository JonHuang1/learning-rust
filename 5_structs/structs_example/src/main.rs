fn main() {
    // Using single variables
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // Using Tuples
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    // Using Structs
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );

    //----------------------------------------------------------

    // println!("rect1 is {}", rect1); // Does not work because Rectangle doesn't implement "Display"
    // println!("rect1 is {:?}", rect1); // Does not work because Rectangle doesn't implement "Debug"
    println!("rect1 is {:?}", rect1); // After adding outer attribute #[derive(Debug)]
    println!("rect1 is {:#?}", rect1);

    //-------------------------------------------------------
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

// Using single variables
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// Using Tuples
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Using Structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}