use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{

        println!("Please input your guess");

        let mut guess = String::new();
    
        io::stdin()
            .read_line( &mut guess)                                                                         
            .expect( "Failed to read line ");
    
            let guess: u32 = match guess.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
            };
    
        println!("guess: {guess}" );
    
        match guess.cmp(&secret_number){
            Ordering::Less =>{
                println!("{}","Too Small!".red());
            }
            Ordering::Greater =>{
                println!("{}","Too Big!".red());
            }
            Ordering::Equal =>{
                println!("{}","You Win!".green());
                break;
            }
            
        }
    }
    
}
