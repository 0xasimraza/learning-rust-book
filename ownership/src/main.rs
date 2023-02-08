fn main() {
    //concept2
    let s1: String = gives_ownership();
    let s2: String = String::from("hello");
    let s3: String = takes_and_gives_back(s2);
    println!("s1 ={}, s3 = {}", s1, s3);

    //concept1
    play();

    //concept3
    let s4: String = String::from("hello");

    let len: usize = calculate_length(&s4);
    println!("The length of '{}' is {}.", &s4, len); //value borrowed here after move, remove to add & before variable

    //concept 4
    let mut s5: String = String::from("hello");
    change(&mut s5);

    //concept 5
    // let mut s: String = String::from("hello");
    // let r1: &mut String = &mut s;
    // let r2: &mut String = &mut s; // second mutable borrow occurs here

    let s: String = String::from("hello");
    let r1: &String = &s;
    let r2: &String = &s;
    // let r3: &mut String = &mut s; //  mutable borrow occurs here if we make s to mutable

    println!("{},{}", r1, r2);

    // but if use after print the bahavior look like this
    // let r3: &mut String = &mut s; // it works (Make sure update s to mutable first to use)
    // println!("{}", r3);

    // concept6 -> Reference not valid & drop string after execution it clears the memory
    //  Rules: 1   At any given time, you can have either one mutable reference or any number of immutable references.
    // 2 References must always be valid.
    // let reference_to_nothing : &String = dangle();
    let reference: String = no_dangle();

    //concept7
    // let mut s1: String = String::from("hello world");
    // let hello: &str = &s1[..5];
    // let world: &str = &s1[..];
    // let word: &str = first_word(&s1);
    // s1.clear(); ------->  mutable borrow occurs here
    // println!("the first word is {}", word)

    let s2: &str = "hello world";
    let word: &str = first_word(&s2);
    println!("the first word is {}", word);

    let a = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[0..2];
    // println!("the slice is {}", slice[0]);
    assert_eq!(slice, &[2, 3]);
}

fn gives_ownership() -> String {
    let some_string: String = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn play() {
    let x: i32 = 5;
    let y: i32 = x; //copy

    let s1: String = String::from("hello");
    // let s2:String = s1;  // value borrowed here after move
    let s2: String = s1.clone(); // use this way
    println!("{}, world", s1);
}

fn calculate_length(s: &String) -> usize {
    // s.push_str("oops"); // `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    let length: usize = s.len(); // len() returns the size of a string
    length
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String{
//     let s:String =String::from("hello")
//     &s
// }

fn no_dangle() -> String {
    let s: String = String::from("hello");
    s
}

//String
fn first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
