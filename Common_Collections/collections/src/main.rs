enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    // let mut v: Vec<i32> = Vec::new();
    // let mut v1 = vec![1, 2, 3];
    // for elem in v {
    //     println!("The number is {elem}")
    // }
    // v1.push(4);
    // v1.push(5);
    // v1.push(6);
    // v1.push(7);
    // for (i, elem) in v1.iter().enumerate() {
    //     println!("The element number {i} is {elem}")
    // }
    // let third: &i32 = &v1[2];
    // println!("The third element is {third}");
    // let third: Option<&i32> = v1.get(2);
    // match third {
    //     Some(third) => println!("The third element is {third}"),
    //     None => println!("There is no third element"),
    // }
    // let v = vec![1, 2, 3, 4, 5];
    // let does_not_exits = &v[100];
    // let does_not_exits = v.get(100);
    // match does_not_exits {
    //     Some(third) => println!("The number is {third}"),
    //     None => println!("Out of range"),
    // }
    // El error aquí es que se está haciendo referencia al primer elemento y es inmutable pero al hacer un push se va a agregar un elemento al final pero en caso de no haber espacio va a tener que mover todo de lugar a otro espacio en memoria y se haría mutable.
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {first}");
    // let mut v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{i}");
    // }
    // for i in &mut v {
    //     *i += 50;
    // }
    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
