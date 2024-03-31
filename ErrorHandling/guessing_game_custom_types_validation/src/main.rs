use core::panic;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess: ");
        let guess = lectura_i32();
        let guess = Guess::new(guess);
        println!("You guessed: {}", guess.value());
        match guess.value().cmp(&secret_number) {
            Ordering::Less => print!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn lectura_i32() -> i32 {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: Result<i32, io::Error> = match guess.trim().parse() {
        Ok(num) => Ok(num),
        Err(_) => panic!("The value is not an i32"),
    };
    guess.unwrap()
}
