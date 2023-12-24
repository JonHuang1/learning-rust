fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("user1 username: {}", user1.username);

    let user2 = build_user("user2@example.com".to_string(), "user2".to_string());
    println!("user2 email: {}", user2.email);

    let user3 = build_user_shorthand(String::from("user3@example.com"), String::from("user3"));
    println!("user3 active?: {}", user3.active);

    // Struct update syntax example
    // Without update syntax:
    let user4 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("user4@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("user4 sign_in_count: {}", user4.sign_in_count);

    // With update syntax:
    let user5 = User {
        email: String::from("user5@example.com"),
        ..user2 // specifies any remaining fields come from corresponding ones in user2
    };
    println!("user5 username: {}", user5.username);

    // black and origin are different types even though types of fields are the same
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

/*
Having parameter name as the same as the struct field name allows you to use field init shorthand syntax
*/
fn build_user_shorthand(email: String, username: String) -> User {
    User {
        active: true,
        username, 
        email,
        sign_in_count: 1,
    }
}

// Tuple Structs
// No names associated with fields, but having a named tuple is convenient
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs
// No fields. Behave similarly to the unit type, ().
// Useful when you need to implement a trait on a type but don't have any data you want to store in the type itself
struct AlwaysEqual;
