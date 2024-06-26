use std::os::unix::thread;

// #[derive(Debug, PartialEq, Copy, Clone)]
// enum ShirtColor {
//     Red,
//     Blue,
// }

// struct Inventory {
//     shirts: Vec<ShirtColor>,
// }
// impl Inventory {
//     fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
//         user_preference.unwrap_or_else(|| self.most_stocked())
//     }
//     fn most_stocked(&self) -> ShirtColor {
//         let mut num_red = 0;
//         let mut num_blue = 0;
//         for color in &self.shirts {
//             match color {
//                 ShirtColor::Red => num_red += 1,
//                 ShirtColor::Blue => num_blue += 1,
//             }
//         }
//         if num_red > num_blue {
//             ShirtColor::Red
//         } else {
//             ShirtColor::Blue
//         }
//     }
// }

fn main() {
    // let store = Inventory {
    //     shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    // };
    // let user_pref1 = Some(ShirtColor::Red);
    // let giveaway1 = store.giveaway(user_pref1);
    // println!(
    //     "The user with preference {:?} gets {:?}",
    //     user_pref1, giveaway1
    // );
    // let user_pref2 = None;
    // let giveaway2 = store.giveaway(user_pref2);
    // println!(
    //     "The user with preference {:?} gets {:?}",
    //     user_pref2, giveaway2
    // );
    // let expensive_closure = |num: u32| -> u32 {
    //     println!("Calculating slowly...");
    //     std::thread::sleep(std::time::Duration::from_secs(2));
    //     num
    // };
    // expensive_closure(5);

    // let list = vec![1, 2, 3];
    // println!("Before defining closure: {:?}", list);

    // let only_borrows = || println!("From closure: {:?}", list);

    // println!("Before calling closure: {:?}", list);
    // only_borrows();
    // println!("After calling closure: {:?}", list);

    // let mut list1 = vec![1, 2, 3];
    // println!("Before defining closure: {:?}", list1);

    // let mut borrows_mutably = || list1.push(7);
    // borrows_mutably();
    // println!("After calling closure: {:?}", list1);
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    std::thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    let funct = |x: i32| println!("{x}");
    funct(5);
}
