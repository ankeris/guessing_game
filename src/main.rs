use std::io::{stdin};
use rand::Rng;

fn main() {
    println!("Guess the nummer!");
    println!("Input your guess!");

    let secret_number: i64 = rand::thread_rng().gen_range(1, 101);

    println!("Random number: {}", secret_number);

    let mut guess = String::new();

    match stdin().read_line(&mut guess) {
        Ok(_n) => {
            println!("{}", guess.len());
        }
        Err(error) => println!("error: {}", error)
    }

    println!("You guessed: {}", guess);
}
