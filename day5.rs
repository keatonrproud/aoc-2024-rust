use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::HashMap;

mod utils;

fn read_data(file_path: &str) -> io::Result<(HashMap<String, Vec<String>>, Vec<Vec<String>>)> {
    let mut rules: HashMap<String, Vec<String>> = HashMap::new();
    let mut updates: Vec<Vec<String>> = Vec::new();

    let reader: BufReader<File> = utils::file_reader(file_path)?;
    for line in reader.lines() {
        let line: String = line?;

        if line.contains('|') {
            let parts: Vec<&str> = line.split('|').collect();
            rules.entry(parts[0].to_string())
                .or_insert_with(Vec::new)
                .push(parts[1].to_string())
        }

        if line.contains(',') {
            let update_line: Vec<String> = line.split(',')
                .map(|s| s.to_string()) // Convert each `&str` into a `String`
                .collect();
            updates.push(update_line);
        }

    }

    Ok((rules, updates))
}

fn check_updates(updates: Vec<Vec<String>>, mut rules: HashMap<String, Vec<String>>) {
    for update_line in updates {
        for (index, update) in update_line.iter().enumerate() {
            // let pre_numbers: Vec<String> = rules.entry(*update)
        }
    }
}

fn main() {
    match read_data("input/day5_1.txt") {
        Ok(data) => println!("Data: {:?}", data),
        Err(e) => eprintln!("{}", e)
    }
}