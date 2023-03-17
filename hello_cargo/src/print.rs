pub fn run() {
    println!("First statement to print");

    println!("My first message is {}", "Hello World");

    println!(
        "{} is better than {} & {} is much better than {}",
        "green", "blue", "yellow", "green"
    );

    // Positional arg
    println!(
        "{3} is friend of {0} & {2} is friend of {1}",
        "Alan", "Brad", "John", "Brad"
    );
    // Placeholder traits
    println!("binary: {:b} Octal: {:o} Hex:{:x}", 10, 10, 10);
    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));
    // Maths
    println!("10 + 10 = {}", 10 + 10);

    // Named arguments
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "Baseball"
    );
}
