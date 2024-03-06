fn main() {
    let s = String::from("Hello, world");
    let num = first_word(&s);
    println!("The length of the first string of '{}' is {}", s, num)
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
