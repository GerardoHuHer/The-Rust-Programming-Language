// Statement are instructions that performs some actions and do not return a value
// Expressions evaluate to a resultant value.

fn main() {
    // let y = 6; // This is a statement
    let x = five();
    println!("The value of x is: {x}");
    let y: i32 = plus_one(x);
    println!("The value of x with the function plus_one is: {y}")
    // main1();
}

// fn main1() {
//     println!("Hello, world!");
//     another_function(5);
//     print_labeled_measurement(5, 'h');
// }

// fn another_function(x: i32) {
//     println!("The value of x is {x}");
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
