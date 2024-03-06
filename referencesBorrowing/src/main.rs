fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    println!("The message from s is '{}'", s);
    change(&mut s);
    println!("The new mesagge from s is '{}'", s)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn data_race() {
    let mut s = String::from("Hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;
}
