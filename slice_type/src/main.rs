fn main() {
    let s = String::from("Hello, world");
    let num = first_word_len(&s);
    let word = first_word(&s);
    println!(
        "The first string of '{}' is {}, and the length is {}",
        s, word, num
    )
}

fn first_word_len(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
