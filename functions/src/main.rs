fn main() {
    println!("Hello, world!");

    another_function();

    another_function2(5);

    print_labeled_measurement(5, 'h');

    let _y0 = 6;

    /*
     This will cause error because "let y = 6" does not return a value
     so x has nothing to bind to
    */
    // let x = (let y = 6);

    let y1 = {
        let x = 3;
        x + 1
    };
    println!("The value of y1 is: {y1}");

}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}