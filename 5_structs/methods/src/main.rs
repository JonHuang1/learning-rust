fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(40);
    println!("Rect4 dimensions: {:?}", sq);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/*
Implementation block for Rectangle.
Anything inside will be associated with Rectangle

Multiple impl blocks are allowed
 */
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Can make methods with same name as one of the struct's fields
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Not all associated functions must have "self" as the first parameter
    // Often times, functions like these are used for constructors that return a new instance of the struct
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}