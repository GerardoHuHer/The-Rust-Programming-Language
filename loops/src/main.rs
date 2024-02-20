fn main() {
    // counter();
    println!("-------------------------------");
    // exe2();
    // f2();
    // whileindex();
    // forfn();
    countdown();
}


fn counter(){
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 10{
            break counter*2;
        }
    };
    println!("The result is: {result}");    
}

fn exe2(){
    let mut count = 0;
    'counting_up: loop{
        println!("count = {count}");
        let mut remaining = 10;
        loop{
            println!("remaining = {remaining}");
            if remaining == 9{
                break;
            }
            if count == 2{
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}")
}

fn f2(){
    let mut number = 3;
    while number != 0{
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn whileindex(){
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5{
        println!("The value is: {}", a[index]);
        index += 1;
    }
}

fn forfn(){
    let a = [10, 20, 30, 40, 50];
    for element in a{
        println!("The value o element is: {}", element)
    }
}

fn countdown(){
    // rev() method allow us to reverse the range
    for number in (1..4).rev(){
        println!("{number}!");
    }
    println!("LIFTOFF!");
}

fn farenheitToCelcius(){
    
}
