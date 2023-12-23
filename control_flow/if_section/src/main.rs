fn main() {
    // IF EXPRESSIONS
    let number0 = 7;
    /* This will not work bc if condition must be a bool
    if number {
        ...
    }
     */
    if number0 < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // HANDLING MULTIPLE CONDITIONS WITH ELSE IF
    let number1 = 6;
    if number1 % 4 == 0 {
        println!("number1 is divisible by 4");
    } else if number1 % 3 == 0 { 
        println!("number1 is divisible by 3");
    } else if number1 % 2 == 0 {
        println!("number1 is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    } 
    
    // USING IF IN A LET STATEMENT
    let condition0 = true;
    let _number2 = if condition0 { 5 } else { 6 };
    /*
    if arm and else arm must match
    this will not compile
    let number = if condition { 5 } else { "size" };
    */
}
