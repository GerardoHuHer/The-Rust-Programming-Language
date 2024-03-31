// Introduction
// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//     let largest = largest(&number_list);
//     println!("The largest number is {}", largest)
// }

// fn largest(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// ----------------------------------------------------------------------------
// Checa lo de implementar el trait std::cmp::PartialOrd
// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//     let result = largest_i32(&number_list);
//     println!("The largest number is {}", result);
//     let char_list = vec!['y', 'm', 'a', 'q'];
//     let result = largest_char(&char_list);
//     println!("The largest char is {}", result)
// }

// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
// ----------------------------------------------------------------------------
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    // let integer_and_float = Point { x: 5, y: 4.0 };
}
