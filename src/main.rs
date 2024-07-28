
use std::io::stdin;
use rand::Rng;
use rand::thread_rng;

fn main()
{
    
    println!("guess the number");

    let secret_number = thread_rng()
    .gen_range(1..=100);

    println!("The secret number is: {}" , secret_number);
    
    println!("input you guess number ");
    
    let mut guess_number = String::new();

    stdin() 
    .read_line(&mut guess_number)
    .expect(" not read line ");
    
    println!("you guess number: {}" , guess_number)

}

