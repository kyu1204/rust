use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..100);
    // println!("The secret number: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line!");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type number!");
                continue;
            }
        };
    
        println!("Your guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
