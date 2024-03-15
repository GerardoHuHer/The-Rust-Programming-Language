// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// impl Message {
//     fn call(&self) {}
// }

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    // let m = Message::Write(String::from("hello"));
    // m.call();
    let some_number = Some(5);
    let some_char = Some('e');
    // Pregunta a Roger por qué tuviste que poner std::option::Option<i32> y no solo Option<i32>
    let absent_number: std::option::Option<i32> = None;
}
