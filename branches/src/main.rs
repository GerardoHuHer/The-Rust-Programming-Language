fn main() {
    let number = 7;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
    different_zero();
}

fn different_zero() {
    let number = 3;
    if number != 0 {
        println!("Number was something other than zero");
    }
}
