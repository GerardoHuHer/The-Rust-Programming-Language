fn main() {
    let data = "initial contents";
    // let mut s = String::new();
    let s = data.to_string();
    let w = "works on literal directly".to_string();
    let string = String::from("from a string literal");
    nothing();
    multiple_concatenation();
    iteraBytesAndChar();
}

fn pushing_string() {
    let mut s = String::from("foo");
    s.push_str(" bar");
}

fn nothing() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
}

fn push_example() {
    let mut s = String::from("lo");
    s.push('l');
}

fn concatenation() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
}

fn multiple_concatenation() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s: {s}")
}
fn another_multiple_concatenation() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // The format macro uses references so that this call does not take ownership of any of its parameters.
    let s = format!("{s1}-{s2}-{s3}");
}

//signature from '+' operator in strings fn add(self, s: &str) -> String{}

fn iteraBytesAndChar() {
    for b in "Aa".bytes() {
        println!("{b}");
    }
    for b in "Aa".chars() {
        println!("{b}");
    }
}
