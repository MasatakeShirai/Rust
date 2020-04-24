extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);
    //println!("The secret number is {}", secret_number);

    loop{
        println!("Please input your guess.");

        //"let" is variable definition,
        //"mut" is make variable mutable
        let mut guess = String::new(); 
    
        io::stdin().read_line(&mut guess)
            .expect("Faild to read line");
        
        //"parse" is rarse strings to number. 
        //"u32" is 32-bit non-negative integer
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            //match all Error
            Err(_) => continue,
        };
        
        println!("You guessed:{}",guess);
        
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
