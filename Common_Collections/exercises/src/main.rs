use std::collections::HashMap;

fn main() {
    let mut arr: Vec<i32> = vec![1, 2, 3, 4, 5, 4, 5, 5, 6, 1, 1, 3, 2, 5, 5, 5, 5];
    let med = median(&mut arr);
    println!("Median from {:?} is {}", arr, med);
    mode(&arr)
    // let mod1 = mode(&arr);
    // println!("The mode of {:?} is {}", arr, mod1)
}

fn median(arr: &mut Vec<i32>) -> i32 {
    arr.sort();
    let len = arr.len();
    if len % 2 == 0 {
        let pos1 = len / 2 - 1;
        let pos2 = len / 2;
        (arr[pos1] + arr[pos2]) / 2
    } else {
        let pos = len / 2;
        arr[pos]
    }
}

fn mode(arr: &Vec<i32>) /* -> i32*/
{
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut ret_key: i32 = 0;
    let mut ret_value: i32 = 0;
    for &elem in arr {
        let count = map.entry(elem).or_insert(0);
        *count += 1;
    }
    for (key, value) in map {
        println!("Number: {}, Occurs: {}", key, value)
        // if ret_value < value {
        //     ret_key = key;
        //     continue;
        // }
    }
    // return ret_key;
}
