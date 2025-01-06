fn main() {
    println!("Convert temperatures between Fahrenheit and Celsius.");
    let temp0 = convert(32, 'C');
    println!("32F => {temp0}C");
    let temp1 = convert(0, 'F');
    println!("0C => {temp1}C");
    let temp2 = convert(10, 'F');
    println!("10C => {temp2}F");
    let temp3 = convert(212, 'C');
    println!("212F => {temp3}C");

    println!("");

    println!("Generate the nth Fibonacci number.");
    let f0 = fib(0);
    println!("0th Fibonacci number = {f0}");
    let f1 = fib(1);
    println!("1th Fibonacci number = {f1}");
    let f2 = fib(5);
    println!("5th Fibonacci number = {f2}");   
    let f3 = fib(10);
    println!("10th Fibonacci number = {f3}");   

    println!("");
    
    println!("Print the lyrics to the Christmas carol “The Twelve Days of Christmas, taking advantage of the repetition in the song.");
    song();
}

fn convert(temp: i32, unit: char) -> i32 {
    if unit == 'C' {
        (temp - 32) * 5 / 9
    } else {
        (temp * 9 / 5) + 32
    }
}

fn fib(n: i32) -> i32 {
    if n == 0 {return 0}
    if n == 1 {return 1}
    let mut count = 2;
    let mut t0 = 0;
    let mut t1 = 1;
    while count < n {
        let t2 = t0 + t1;
        t0 = t1;
        t1 = t2;
        count += 1;
    }
    return t1;
}

fn song() {
    let lyrics = ["And a partridge in a pear tree",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,"
    ];
    
    let day_of = ["On the first day of Christmas,",
        "On the second day of Christmas,",
        "On the third day of Christmas,",
        "On the fourth day of Christmas,",
        "On the fifth day of Christmas,",
        "On the sixth day of Christmas,",
        "On the seventh day of Christmas,",
        "On the eighth day of Christmas,",
        "On the ninth day of Christmas,",
        "On the tenth day of Christmas,",
        "On the eleventh day of Christmas,",
        "On the twelfth day of Christmas,"
    ];
    
    for i in 0..lyrics.len() {
        println!("{}\nMy true love gave to me", day_of[i]);
        for j in (0..i+1).rev() {
            if i == 0 {
                println!("A partridge in a pear tree");
                break;
            }
            println!("{}", lyrics[j]);
        }
        println!("");
    }
}
    