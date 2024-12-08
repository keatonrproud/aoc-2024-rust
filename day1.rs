mod utils;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_data(file_path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let reader: BufReader<File> = utils::file_reader(file_path)?;
    let mut l1 = Vec::new();
    let mut l2 = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let line_data: Vec<&str> = line.split_whitespace().collect();

        if let (Ok(first), Ok(second)) = (line_data[0].parse::<i32>(), line_data[1].parse::<i32>()) {
            l1.push(first);
            l2.push(second);
        } else {
            eprintln!("Skipping invalid line: {}", line);
        }
    }
    Ok((l1, l2))
}

fn get_min(vector: &Vec<i32>) -> Option<i32> {
    return vector.iter().min().copied();
}

fn remove_num(vector: &mut Vec<i32>, int: i32) {
    if let Some(index) = vector.iter().position(|&x| x == int) {
        vector.remove(index);
    }
}

/// check the distances between a vector of ints
fn check_num_dist(ints: Vec<i32>) -> Result<i32, String> {
    if ints.len() != 2 {
        return Err("Can only operate on vectors of two ints.".to_string());
    }

    let diff = ints[1] - ints[0];

    println!("The non-abs dist for {:?} is {}", ints, diff);

    Ok(diff.abs())

}

fn check_vec_dist(mut vecs: Vec<Vec<i32>>) -> i32 {
    let mut total_dist = 0;
    for _ in 0..vecs[0].len() {
        let mut mins = Vec::new();
        for vec in vecs.iter_mut() {
            let min = get_min(vec).expect("Failed to find a minimum for a vector.");
            remove_num(vec, min);
            mins.push(min);
        }
        match check_num_dist(mins) {
            Ok(dist) => total_dist += dist,
            Err(e) => eprintln!("Num Dist error: {}", e),
        }
    }
    total_dist
}

fn count_num_in_vec(vec: &Vec<i32>, num: i32) -> i32 {
    vec.iter().filter(|&&x| x == num).count() as i32
}

fn check_vec_similarity(vec1: Vec<i32>, vec2: Vec<i32>) -> i32 {

    let mut total_sim = 0;

    for v in vec1.iter() {
        total_sim += count_num_in_vec(&vec2, *v) * *v;
    }

    total_sim

}


fn main() {
    match read_data("input/day1_1.txt") {
        Ok((col1, col2)) =>
        {
//             println!("Column distance: {}", check_vec_dist(vec![col1, col2]));
            println!("Column similarity: {}", check_vec_similarity(col1, col2));
        },
        Err(e) => eprintln!("Error reading file: {}", e)
    }
}
