fn main() {
    let q0 = value_in_cents(Coin::Quarter(UsState::Alaska));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    // Uses the catch all pattern( _ ) instead of "other"
    // match dice_roll {
    //     3 => add_fancy_hat(),
    //     7 => remove_fancy_hat(),
    //     _ => reroll(), 
    // }
    // 
    // Uses the unit value () so it does nothing when its not 3 or 7
    // let dice_roll = 9;
    // match dice_roll {
    //     3 => add_fancy_hat(),
    //     7 => remove_fancy_hat(),
    //     _ => (),
    // }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

/*
Matches are Exhaustive
The following will not compile as the None case is not handles.

fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }

*/

