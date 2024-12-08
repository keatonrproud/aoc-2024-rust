use std::io::BufRead;
use std::io;
mod utils;

fn read_data(file_path: &str) -> io::Result<String> {
    let mut data: String = String::new();

    let reader = utils::file_reader(file_path)?;
    for line in reader.lines() {
        let line = line?;
        data += &line;
    }
    Ok(data)
}

fn is_int(string: char) -> bool{
    match string.to_digit(10) {
        Some(_) => true,
        None => false,
    }
}

fn is_comma(string: char) -> bool{
    string == ','
}

fn can_add_int(mul: &String) -> bool {
    if mul.len() < 4 {
        false
    } else if mul == "mul(" {
        true
    } else if vec![',', '('].contains(&mul.chars().last().unwrap()) {
        true
    } else if is_int(mul.chars().nth_back(0).unwrap()) {
        println!("{}", mul);
        let last3 = mul.chars().rev().take(3).collect::<String>();
        println!("{}", last3);
        if last3.contains(',') {
            true
        } else if is_int(last3.chars().nth(2).unwrap()) {
            false
        } else {
            true
        }
    } else {
        false
    }
}

fn can_add_comma(mul: &String) -> bool {
    is_int(mul.chars().last().unwrap())
}

fn find_valid_muls(string: &String) -> Vec<String> {
    let mut muls: Vec<String> = Vec::new();
    let mut mul: String = String::new();

    let mut enabled: bool = true;

    for (index, s) in string.chars().enumerate() {
        println!("{}, {:?}", s, mul);
        if s == 'm' && mul == "" {
            mul += &s.to_string();
            continue
        } else if s == 'u' && mul == "m" {
            mul += &s.to_string();
            continue
        } else if s == 'l' && mul == "mu" {
            mul += &s.to_string();
            continue
        } else if s == '(' && mul == "mul" {
            mul += &s.to_string();
            continue
        } else if is_int(s) && can_add_int(&mul) {
            mul += &s.to_string();
            continue
        } else if s == ',' && mul.len() > 0 && can_add_comma(&mul) {
            mul += &s.to_string();
            continue
        } else if s == ')' && mul.contains(',') && is_int(mul.chars().last().unwrap()) {
            mul += &s.to_string();
            println!("Final mul {}", mul);
            if enabled {
                muls.push(mul);
            }
            mul = String::new();
        } else {
            if s == 'd' {
                if string[index..index+4].to_string() == "do()" {
                    enabled = true;
                } else if string[index..index+7].to_string() == "don't()" {
                    enabled = false;
                }
            }
            mul = String::new();
        }
    }

    println!("Valid muls: {:?}", muls);
    muls
}

fn calc_sum(muls: Vec<String>) -> i32 {
    let mut sum: i32 = 0;
    for mul in muls {
        let parts: Vec<&str> = mul.split(',').collect();
        let part1 = parts[0].replace("mul(", "");
        let part2 = parts[1].replace(")","");
        let ints: Vec<i32> = vec![part1, part2]
            .iter()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        sum += ints[0] * ints[1];
    }

    println!("Sum: {}", sum);
    sum
}

fn main() {
    match read_data("input/day3_1.txt") {
        Ok(data) => {
            let muls = find_valid_muls(&data);
            calc_sum(muls);
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}