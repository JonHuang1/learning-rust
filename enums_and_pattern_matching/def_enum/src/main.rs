fn main() {
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));

    // let home2 = IpAddr2::V4(127, 0, 0, 1);
    // let loopback2 = IpAddr2::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}

enum IpAddrKind {
    V4,
    V6,
}

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}