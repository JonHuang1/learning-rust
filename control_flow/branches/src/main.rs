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

    // REPETIITON WITH LOOPS
    // REPEATING CODE WITH "LOOP"
    // loop uses: retry operation you know will fail
    // ie. checking whether a thread has completed its job
    
    // loop {
    //     println!("again!");
    // }
    
    // pass result of operation out of loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // LOOP LAELS TO DISAMBIGUATE BETWEEN MULTIPLE LOOPS
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}
