use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;
fn main() {
    let _cube = cubez {
        r: 12,
        g: 13,
        b: 14,
    };
    let data: Vec<String> = read_lines();
    //let not_positionz: Vec<i32> = possible_ID(data);
    //let possible: Vec<i32> = get_vec_possible(not_positionz);
    //let sum: i32 = pos_add(possible);
    let power_c: Vec<i32> = power(data);
    let sum: i32 = pos_add(power_c);
    println!("sum: {}", sum.to_string());
}

struct cubez {
    r: i32,
    g: i32,
    b: i32,
}
fn read_lines() -> Vec<String> {
    let mut data = Vec::new();
    for line in read_to_string("/home/zp0/aoc/day-2/src/input")
        .unwrap()
        .lines()
    {
        data.push(line.to_string());
    }
    data
}
fn pos_add(positionz: Vec<i32>) -> i32 {
    //    let pos: Vec<i32> = duplicatez_remove(positionz);
    let mut a: i32 = 0;
    for p in positionz {
        a += p;
    }
    a
}
fn get_vec_possible(numbers: Vec<i32>) -> Vec<i32> {
    let mut possible = Vec::new();
    for c in 0..=100 {
        if !numbers.contains(&c) {
            possible.push(c);
        }
    }
    possible
}
fn duplicatez_remove(data: Vec<i32>) -> Vec<i32> {
    let unique_numb: HashSet<i32> = data.into_iter().collect();
    unique_numb.into_iter().collect()
}

fn power(data: Vec<String>) -> Vec<i32> {
    let game_ID_re = Regex::new(r"Game (\d+):").unwrap();
    let colorz_re = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    let mut power_a: Vec<i32> = Vec::new();
    for line in data {
        let mut red: i32 = 1;
        let mut green: i32 = 1;
        let mut blue: i32 = 1;
        println!("{}", line);
        if let Some(game_id_cap) = game_ID_re.captures(&line) {
            let _game_id: i32 = game_id_cap[1]
                .parse::<i32>()
                .expect("couldnt parse game_ID");
            let color_set_str = &line[game_id_cap[0].len()..];
            let color_set = color_set_str.split(";").filter(|s| !s.is_empty());
            for set in color_set {
                //println!("set = {}", set.to_string());
                for color_c in colorz_re.captures_iter(set) {
                    let value = color_c[1].parse::<i32>().unwrap();
                    let color = color_c[2].to_string();
                    //println!(
                    //    "color = {} value = {}",
                    //    color.to_string(),
                    //    value.to_string()
                    //);
                    if color == "red".to_string() && value > red {
                        red = value;
                    }
                    if color == "green".to_string() && value > green {
                        green = value;
                    }
                    if color == "blue".to_string() && value > blue {
                        blue = value;
                    }
                }
            }
            let power: i32 = red * green * blue;
            println!("power {}", power.to_string());
            power_a.push(power);
        }
    }
    power_a
}
fn possible_ID(data: Vec<String>) -> Vec<i32> {
    let game_ID_re = Regex::new(r"Game (\d+):").unwrap();
    let colorz_re = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    let mut possible_val: Vec<i32> = Vec::new();
    for line in data {
        println!("{}", line);
        if let Some(game_id_cap) = game_ID_re.captures(&line) {
            let game_id: i32 = game_id_cap[1]
                .parse::<i32>()
                .expect("couldnt parse game_ID");
            let color_set_str = &line[game_id_cap[0].len()..];
            let color_set = color_set_str.split(";").filter(|s| !s.is_empty());
            for set in color_set {
                //println!("set = {}", set.to_string());
                for color_c in colorz_re.captures_iter(set) {
                    let value = color_c[1].parse::<i32>().unwrap();
                    let color = color_c[2].to_string();
                    //println!(
                    //    "color = {} value = {}",
                    //    color.to_string(),
                    //    value.to_string()
                    //);
                    if (color == "red".to_string() && value > 12)
                        || (color == "green".to_string() && value > 13)
                        || (color == "blue".to_string() && value > 14)
                    {
                        possible_val.push(game_id);
                        println!("not possible game found {}", game_id.to_string());
                    }
                }
            }
        }
    }
    possible_val.dedup();
    possible_val
}
fn get_possible(data: Vec<String>) -> Vec<i32> {
    let re = Regex::new(r"Game (\d+):((?: \d+ (?:red|green|blue), ?)+)").unwrap();
    let colorz_re = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    let mut possible_val: Vec<i32> = Vec::new();
    for line in data {
        println!("{}", line);
        for c in re.captures_iter(&line) {
            let game_id: i32 = c[1].parse::<i32>().expect("couldnt parse");
            for set in c[2].split(";") {
                println!("set = {}", set.to_string());
                for color_set in set.split(", ") {
                    for color_c in colorz_re.captures_iter(&color_set) {
                        let value = color_c[1].parse::<i32>().unwrap();
                        let color = color_c[2].to_string();
                        println!(
                            "color = {} value = {}",
                            color.to_string(),
                            value.to_string()
                        );
                        if (color == "red".to_string() && value > 12)
                            || (color == "green".to_string() && value > 13)
                            || (color == "blue".to_string() && value > 14)
                        {
                            possible_val.push(game_id);
                            println!("not possible game found {}", game_id.to_string());
                        }
                    }
                }
            }
        }
    }
    possible_val
}

fn extract_cubes(line: String) -> HashMap<String, Vec<HashMap<String, i32>>> {
    let re = Regex::new(r"Game (\d+):((?: \d+ (?:red|green|blue), ?)+)").unwrap();
    let colorz_re = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    let mut game_data = HashMap::new();
    for c in re.captures_iter(&line) {
        let game_id = c[1].to_string();
        let mut colors = Vec::new();
        for set in c[2].split(";") {
            let mut color_map = HashMap::new();
            for color_c in colorz_re.captures_iter(set) {
                let value = color_c[1].parse::<i32>().unwrap();
                let color = color_c[2].to_string();
                color_map.insert(color, value);
            }
            colors.push(color_map);
        }
        game_data.insert(game_id, colors);
    }
    game_data
}
