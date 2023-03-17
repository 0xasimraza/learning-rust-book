// vector is resizeable array
use std::mem;
pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    numbers[4] = 23;

    numbers.push(24);

    numbers.pop();

    println!("Vector occupies {} bytes ", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[0..2];
    println!("{:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }
    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers); // [2, 4, 40, 8, 10]
}
