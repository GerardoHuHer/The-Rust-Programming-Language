use std::collections::HashMap;
// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn main() {
//     let mut map = HashMap::new();
//     map.insert(1, 2);
// }

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }

use std::io;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

mod customer {
    pub use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    customer::eat_at_restaurant();
    println!("Ingrese un número: ");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    let num: u32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    println!("Your number is {num}")
}
