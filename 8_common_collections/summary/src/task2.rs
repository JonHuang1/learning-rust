fn pig_latinfy(original: String) -> String {
    match &original[0..1] {
        "a" => return vowel_convert(original),
        "e" => return vowel_convert(original),
        "i" => return vowel_convert(original),
        "o" => return vowel_convert(original),
        "u" => return vowel_convert(original),
        _ => return consonant_convert(original[0..1].to_string(), original[1..].to_string()),
    }
}

fn vowel_convert(string: String) -> String {
    return string + "-hay";
}

fn consonant_convert(first: String, string: String) -> String {
    return format!("{string}-{first}ay");
}

pub fn task2_print(original: String) {
    print!("{} -> ", original);
    println!("{}", pig_latinfy(original));
}
