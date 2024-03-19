#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximun is configured to be {}", max),
    //     _ => (),
    // }
    // Instead of that, we can write this
    // let config_max = Some(3u8);
    // if let Some(max) = config_max {
    //     println!("The maximum is configured to be {}", max);
    // }
    let coin: Coin = Coin::Quarter(UsState::Alabama);
    let value = count_noun_quarters(coin);
    println!("The value is {}", value);
    let coin1: Coin = Coin::Penny;
    let value2 = count_noun_quarters(coin1);
    println!("The value2 is {}", value2);
}

fn count_noun_quarters(coin: Coin) -> i32 {
    let mut count: i32 = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
    count
}
