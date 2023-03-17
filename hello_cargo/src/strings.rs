use std::vec;

pub fn run() {
    let mut hello = String::from("Hello");
    println!("Length of string is {}", hello.len());

    hello.push('W');
    hello.push_str("orld");
    println!("Length of string is {}", hello);

    println!("Capacity of string is {}", hello.capacity());

    println!("Is hello empty: {}", hello.is_empty());

    println!("Is hello contain World? : {}", hello.contains("World"));

    println!(
        "Replace World with There : {}",
        hello.replace("World", "There")
    );

    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    // Loop through string by whitespace
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    let mut s = String::with_capacity(10);
    s.push('s');
    s.push('t');
    println!("{}", s);
    //Assertion test
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    let mut chars: Vec<char> = pangram.chars().collect();
    // println!("{:?}", chars);
    chars.sort();
    chars.dedup();
    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ")
    }
    // println!("string is {}", string);

    // The trimmed string is a slice to the original string, hence no new
    // allocation is performed
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    // println!("Used characters: {}", trimmed_str);
}
