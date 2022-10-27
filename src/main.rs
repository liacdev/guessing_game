use std::io;

fn main() {
    // This is a guessing game built in Rust language
    println!("Guess the number!");

    println!("Please input your guess.");
    
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {guess}");
}
