fn main() {
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
