fn main() {
    let s = "hello"; // This is inmutable
    let mut w = String::from("Hello");
    w.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", w);
}
