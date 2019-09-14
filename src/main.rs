use std::io::{stdin};

fn main() {
    println!("Guess the nummer!");
    println!("Input your guess!");

    let mut guess = String::new();

    match stdin().read_line(&mut guess) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", guess);
        }
        Err(error) => println!("error: {}", error)
    }

    println!("You guessed: {}", guess);
}
