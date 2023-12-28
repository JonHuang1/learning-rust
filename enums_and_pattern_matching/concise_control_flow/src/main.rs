fn main() {
    /*
    If you have a situation in which your program has logic that is too 
    verbose to express using a "match", remember that "if let" is in your Rust toolbox as well.
    "if let" is a match that only matches 1 potential pattern
    */

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to by {}", max),
        _ => (),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let coin = Coin::Quarter(State::Alabama);
    let mut count = 0;
    match coin {
        Coin::Quarter(ref state) => println!("state quarter from {:?}!", state),
        _ => count += 1,
    }

    // let coin = Coin::Quarter(State::Alaska);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quearter form {:?}!", state);
    } else {
        count += 1;
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
}