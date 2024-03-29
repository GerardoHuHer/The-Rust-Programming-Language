use std::collections::HashMap;

fn main() {
    let word = exercise2(&mut "Ale".to_string());
    println!("{word}");
}

fn exercise1() {
    println!("Exercise 1: Median and Mode");
    let mut arr: Vec<i32> = vec![1, 2, 3, 4, 5, 4, 5, 5, 6, 1, 1, 3, 2, 5, 5, 5, 5];
    let med = median(&mut arr);
    println!("The array is {:?}", arr);
    println!("Median is {}", med);
    let mod1 = mode(&arr);
    println!("The mode is {}", mod1)
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

fn mode(arr: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut ret_key: i32 = 0;
    let mut ret_value: i32 = 0;
    for &elem in arr {
        let count = map.entry(elem).or_insert(0);
        *count += 1;
    }
    for (key, value) in map {
        if ret_value < value {
            ret_key = key;
            ret_value = value;
        }
    }
    return ret_key;
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn starts_with_vowel(word: &String) -> bool {
    if let Some(first_char) = word.chars().next() {
        is_vowel(first_char.to_ascii_lowercase())
    } else {
        false
    }
}
fn exercise2(word: &mut String) -> String {
    if (starts_with_vowel(&word)) {
        word[..].to_string() + "-hay"
    } else {
        word[1..].to_string() + "-" + &word[..1] + "ay"
    }
}

fn exercise3() {
    /*
    Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
    */
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
}
