use std::{collections::HashMap, io};

fn main() {
    exercise1();
    let word = exercise2(&mut "adios".to_string());
    println!("{word}");
    exercise3();
}

// ====================================== EXERCISE 1
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
        *count += 1
    }
    for (key, value) in map {
        if ret_value < value {
            ret_key = key;
            ret_value = value;
        }
    }
    return ret_key;
}

// ====================================== EXERCISE 2
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

// ====================================== EXERCISE 3
fn exercise3() {
    /*
    Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
    */
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("1) Añadir empleado \n2) Mostrar todos \n3) Mostrar por area \n0) Salir");
        println!("Ingrese la opción deseada");
        let opc = lectura_i32();
        match opc {
            0 => {
                println!("Hasta luego");
                break;
            }
            1 => llenado_mapa(&mut map),
            2 => mostrar(&mut map),
            3 => {
                println!("Ingrese el área buscada");
                let area = lectura_string();
                mostrar_por_zona(&mut map, area)
            }
            _ => println!("No se ingresó una opción válida"),
        };
    }
}

fn llenado_mapa(hm: &mut HashMap<String, Vec<String>>) {
    println!("Ingrese el nombre del area de trabajo: ");
    let area = lectura_string();
    println!("Ingrese el nombre del empleado: ");
    let name = lectura_string();
    hm.entry(area).or_insert(Vec::new()).push(name);
}

fn mostrar(mapa: &mut HashMap<String, Vec<String>>) {
    for (key, value) in mapa {
        value.sort();
        println!("Area: {key}");
        for elem in value {
            println!("\t-{elem}");
        }
    }
}
fn mostrar_por_zona(mapa: &mut HashMap<String, Vec<String>>, area: String) {
    let mut band: bool = false;
    for (key, value) in mapa {
        if key == &area {
            println!("{key}");
            for elem in value {
                println!("\t-{elem}")
            }
            band = true;
        }
    }
    if !band {
        println!("No se encontró el área buscada");
    }
}

fn lectura_string() -> String {
    let mut aux = String::new();
    io::stdin()
        .read_line(&mut aux)
        .expect("No se leyó correctamente");
    aux
}

fn lectura_i32() -> i32 {
    let mut aux = String::new();
    io::stdin()
        .read_line(&mut aux)
        .expect("No se leyó correctamente");
    let aux: i32 = match aux.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    aux
}
