// fn main() {
//     let s = "hello"; // This is inmutable
//     let mut w = String::from("Hello");
//     w.push_str(", world!"); // push_str() appends a literal to a String
//     println!("{}", w);
//     // Whit this two lines Rust conseders s1 as no longer valid because they both are pointing to the same memory location so, if we drop them the same memory twice can lead to memory corruption
//     let s1 = String::from("Hello");
//     let s2 = s1;
//     // We cannot print s1 because is no longer valid
//     // println!("{}, world", s1);
//     clone_heap_variables();
// }

// fn clone_heap_variables(){
//     let s1 = String::from("Hello");
//     let s2 = s1.clone();
//     println!("s1 = {}, s2 = {}", s1, s2);
// }

// fn main() {
//     let s = String::from("hello");
//     takes_ownership(s);
//     let x = 5;
//     makes_copy(x);
// }

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

// fn main() {
//     let s1 = gives_ownership();
//     let s2 = String::from("Hello");
//     let s3 = takes_and_gives_back(s2);
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("Yours");
//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

fn main() {
    let s1 = String::from("Hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length);
    // let s = "hello"; // This is inmutable
    // let mut w = String::from("Hello");
    // w.push_str(", world!"); // push_str() appends a literal to a String
    // println!("{}", w);
    // let s1 = String::from("Hello");
    // let s2 = s1;
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    function();
}

fn function() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}
