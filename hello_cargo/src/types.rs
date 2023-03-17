// Primitive types
// Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (unsigned: no negative values)
// Floats: f32, f64
// Boolean (bool)
// Characters (char)
// Tuples
// Arrays

pub fn run() {
    let x = 1;
    let y = 2.1;
    let z: i64 = 25252525252;

    println!("i32 MAX is {}", std::i32::MAX);
    println!("i64 MAX is {}", std::i64::MAX);
    // Boolean
    // let is_active = true;
    let is_active: bool = true;
    // Get boolean from expression
    let is_greater: bool = 10 > 5; // true
                                   // Char with single quotes
    let a1 = 'a';
    let face = '\u{1F600}'; // emoji unicode: https://unicode.org/emoji/charts/full-emoji-list.html
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
