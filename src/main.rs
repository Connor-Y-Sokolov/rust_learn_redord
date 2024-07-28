use rand::{Rng, thread_rng}; 
use std::cmp::Ordering::{Equal, Less, Greater}; 
use std::io::stdin; 

fn play_guessing_game(secret_number: u32) {
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, 
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Less => println!("Too small!"), 
            Greater => println!("Too big!"), 
            Equal => {
                println!("You win!"); 
                break; 
            }
        }
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1..=100); 

    play_guessing_game(secret_number);
}
