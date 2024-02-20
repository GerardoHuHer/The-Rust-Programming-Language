fn main() {
    let number: i32 = 7;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
    different_zero();
    divisibility();
    ifLetStatement();
}

fn different_zero() {
    let number = 3;
    if number != 0 {
        println!("Number was something other than zero");
    }
}

fn divisibility(){
    let number = 6;
    if number % 4 == 0{
        println!("Number is divisible by 4");
    } else if number % 3 == 0{
        println!("Number is divisible by 3");
    } else if number % 2 == 0{
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3 or 2");
    }
}

fn ifLetStatement(){
    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of number is {number}");
}
