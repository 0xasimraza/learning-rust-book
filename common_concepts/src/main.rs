use std::io;

fn main() {
    // setup_variables();
    // setup_constant_variables();
    // variable_shadowing();
    // type1();
    // type2();
    // find_value_by_index();
    let (x,y) = two_args_returns(2,3);
    println!("sum is {x} & muliply is {y}");
}

fn setup_variables(){
    // basic mutable variable
    let mut x = 5;
    println!("The x value is {x}");
    x = 6;
    println!("The x value is {x}");
}


fn setup_constant_variables(){
  // constant variable 
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
  println!("THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}")
}

fn variable_shadowing(){
    let x = 5;

    let x = x + 1;

    //definig scope
    {
        let x = x*2 ;
        println!("The value of x in the inner space is: {x}" );
    }

    println!("The value of x is {x}");
}

// fn variableShadowing2(){
//     let mut spaces = "    ";
//     spaces = spaces.len();
// }

// DATA TYPES
// Number-literals	Example
// Decimal	        98_222
// Hex	            0xff
// Octal	        0o77
// Binary	        0b1111_0000
// Byte (u8 only)	    b'A'

fn type1(){
    let guesses : u32 = "42".parse().expect("Not a number");

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

       // addition
       let sum = 5 + 10;

       // subtraction
       let difference = 95.5 - 4.3;
   
       // multiplication
       let product = 4 * 30;
   
       // division
       let quotient = 56.7 / 32.2;
       let truncated = -5 / 3; // Results in -1
   
       // remainder
       let remainder = 43 % 5;


       let t = true;

    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

fn type2(){
    //Compound Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}",tup.2);

    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    println!("{}",months[3]);

    // You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so:
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{}",a[2]);

    // You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:
    let b = [5; 3];
    println!("{}",b[2]);
}



fn find_value_by_index() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

// functions behaviors
fn two_args_returns(value1: isize,value2: isize ) -> (isize,isize){

( value1 + value2, value1 * value2)

}