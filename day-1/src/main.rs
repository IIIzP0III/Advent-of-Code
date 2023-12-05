use std::fs::read_to_string;

use std::collections::HashMap;
fn main() {
    let data: Vec<String> = read_lines();

    let mut data2: Vec<u32> = Vec::new();
    for line in data {
        data2.push(get_val(line));
    }
    let product = get_sum(data2);
    println!("product: {}", product.to_string());
}

fn read_lines() -> Vec<String> {
    let mut data = Vec::new();
    for line in read_to_string("/home/zp0/aoc/1/src/src/cal")
        .unwrap()
        .lines()
    {
        data.push(line.to_string());
    }
    data
}
fn get_val(data_line: String) -> u32 {
    let mut a: u32 = 0;
    let mut b: u32 = 0;
    let mut pos_a: u32 = 0;
    let mut pos_pattern_a: u32 = 0;
    let mut pos_pattern_b: u32 = 0;
    let data: Vec<char> = data_line.chars().collect();
    println!("checking {}", data_line);
    for c in &data {
        if c.is_digit(10) {
            a = c.to_digit(10).expect("couldnt transform");
            println!("found a = {}", a.to_string());
            break;
        }
        pos_a += 1;
    }
    let mut pos_b = data.len() as u32;
    for c in (0..data.len()).rev() {
        let c1 = data[c];
        if c1.is_digit(10) {
            b = c1.to_digit(10).expect("couldnt transform b");
            println!("found b = {}", b.to_string());
            break;
        }
        pos_b -= 1;
    }
    let min_word_approach = extract_minnumber_pattern(&data_line);
    let max_word_approach = extract_maxnumber_pattern(&data_line);

    match min_word_approach {
        Some((number, index)) => {
            if pos_a > index.try_into().unwrap() {
                println!("found {} for a ", number);
                a = number;
            }
        }
        None => {}
    }
    match max_word_approach {
        Some((number, index)) => {
            if pos_b <= index.try_into().unwrap() {
                println!("found {} for b", number);
                b = number;
            }
        }
        None => {}
    }
    let c3: String = format!("{}{}", a.to_string(), b.to_string());
    let d = c3.parse::<u32>();
    match d {
        Ok(d) => {
            println!("solution {}", d);
            d
        }
        Err(d) => {
            println!("error {}", d);
            0
        }
    }
}
fn get_sum(data: Vec<u32>) -> u32 {
    let mut c: u32 = 0;
    for d in data {
        c += d;
    }
    c
}

fn extract_minnumber_pattern(line: &str) -> Option<(u32, usize)> {
    let patterns = string_to_number();
    patterns
        .iter()
        .filter_map(|(&word, &number)| line.find(&word).map(|index| (number, index)))
        .min_by_key(|&(_, index)| index)
}

fn extract_maxnumber_pattern(line: &str) -> Option<(u32, usize)> {
    let patterns = string_to_number();
    patterns
        .iter()
        .filter_map(|(&word, &number)| line.rfind(&word).map(|index| (number, index)))
        .max_by_key(|&(_, index)| index)
}
fn string_to_number() -> HashMap<&'static str, u32> {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);
    map.insert("zero", 0);
    map
}
