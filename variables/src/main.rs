fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
    // // Constants are always inmmutable and they are declarated with the keyword "const"
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
    say_hello();
}

fn say_hello() {
    println!("Hello, world!");
    another_function();
}
fn another_function() {
    println!("Another function.");
}
