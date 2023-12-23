fn main() {
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
