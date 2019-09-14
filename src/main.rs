use std::io::{stdin};
use rand::Rng;

fn main() {
    println!("Guess the nummer!");
    println!("Input your guess!");

    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);    
    let mut did_guess: bool = false;
    
    while !did_guess {
        let mut guess = String::new();
        match stdin().read_line(&mut guess) {
            Ok(_n) => {
                let s = String::from(secret_number.to_string());
                if are_two_strings_equal(guess, s) {
                    did_guess = true;
                }
                
            }
            Err(error) => println!("error: {}", error)
        };
    }

    fn are_two_strings_equal(str1: String, str2: String) -> bool {
        // Slice the trailing space...
        let trimmed1: i32 = str1.trim().parse::<i32>().unwrap();
        let trimmed2: i32 = str2.trim().parse::<i32>().unwrap();
        if trimmed1 > trimmed2 { println!("Try lower") } else { println!("Try higher") }
        if trimmed1 == trimmed2 { println!("success") }
        return trimmed1 == trimmed2;
    }
}
