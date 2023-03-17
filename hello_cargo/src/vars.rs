pub fn run() {
    let name = "Alexandar";
    let mut age = 38;

    println!("My name is {name} and age is {}", age);
    age = 41;
    println!("My name is {name} and age is {}", age);

    const ID: i32 = 001;
    println!("ID: {ID}");

    let (my_name, my_age) = ("Alex", 34);

    println!("My name is {my_name} and age is {}", my_age);
}
