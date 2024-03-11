#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels, with the area function.",
        area(width1, height1)
    );
    println!(
        "The area of the rectangle is {} square pixels, with the area1 function.",
        area1(rect1)
    );
    let rect2 = Rectangle {
        width: 50,
        height: 30,
    };
    println!(
        "The area of the rectangle is {} square pixels, with the struct area function.",
        area_struct(&rect2)
    );
    println!("rect2 is {:?}", rect2);
    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect3);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
