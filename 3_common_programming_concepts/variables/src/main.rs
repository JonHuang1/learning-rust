fn main() {
    // mutable variable can change value
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("the value of x is: {x}");

    // const values can be declared with expression. Cannot be changed
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    /*
    Shadowing example. Creates a new variable with same name. 
    Still immutable
     */
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    // shadowing can change the type of variable
    let spaces = "  ";
    println!("Space {spaces} and {spaces} Space");
    let spaces = spaces.len();
    println!("Spaces: {spaces}");

    // not allowed to mutate a variable's type. Will cause error
    // let mut spaces_2 = "  ";
    // spaces_2 = spaces_2.len();
}
