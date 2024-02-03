// This line is to obtain user input output. We use io library from standard library std
use std::io;
fn main() {
    println!("Guess the number!");
    println!("Please input your guess: ");
    // Variable creation, we use the let statement to create a variable example:  let apples = 5;
    // Variables in rust are inmutables if we wanna make them mutables we add the word mut before the variable name
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}")
}
