// #[derive(Debug)]
// struct Rectangle {
//     width: i32,
//     height: i32,
// }
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }

// pub fn greeting(name: &str) -> String {
//     format!("Hello {}!", name)
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn greeting_contains_name() {
//         let result = greeting("Gerardo");
//         assert!(
//             result.contains("Gerardo"),
//             "Greeting did not contain name, value was {}",
//             result
//         );
//     }

//     #[test]
//     fn it_adds_two() {
//         assert_eq!(4, add_two(2))
//     }

//     #[test]
//     fn larger_cannot_hold_larger() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };
//         assert!(!smaller.can_hold(&larger));
//     }

//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };
//         assert!(larger.can_hold(&smaller));
//     }

//     #[test]
//     fn exploration() {
//         assert_eq!(2 + 2, 4);
//     }

//     #[test]
//     fn another() {
//         panic!("Make this test fail");
//     }
// }
// pub struct Guess {
//     value: i32,
// }

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 {
//             panic!("Guess value must be greater or equal to 1, got {}", value)
//         } else if value > 100 {
//             panic!("Guess value must be less or equal to 100, got {}", value)
//         }
//         Guess { value: value }
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     #[should_panic(expected = "Less than or equal to 100")]
//     fn greater_than_100() {
//         Guess::new(200);
//     }
// }

#[cfg(test)]
mod test {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Two plus two does no equal four"))
        }
    }
}
