use std::fs::read_to_string;

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
    let data: Vec<char> = data_line.chars().collect();
    for c in &data {
        if c.is_digit(10) {
            a = c.to_digit(10).expect("couldnt transform");
            println!("found a = {}", a.to_string());
            break;
        }
    }
    for c in (0..data.len()).rev() {
        let c1 = data[c];
        if c1.is_digit(10) {
            b = c1.to_digit(10).expect("couldnt transform b");
            println!("found b = {}", b.to_string());
            break;
        }
    }
    let c3: String = format!("{}{}", a.to_string(), b.to_string());
    let d = c3.parse::<u32>();
    match d {
        Ok(d) => d,
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
