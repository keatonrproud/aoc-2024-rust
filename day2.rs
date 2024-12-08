mod utils;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_data(file_path: &str) -> io::Result<Vec<Vec<i32>>> {
    let mut data: Vec<Vec<i32>> = Vec::new();

    let reader: BufReader<File> = utils::file_reader(file_path)?;
    for line in reader.lines() {
        let line: String = line?;

        let line_numbers: Result<Vec<i32>, _> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect();

        match line_numbers {
            Ok(report_data) => data.push(report_data),
            Err(e) => {
                return Err(io::Error::new(io::ErrorKind::InvalidData, e));
            }
        }
    }
    Ok(data)
}

/// Reports are safe if the Vector is entirely increasing or decreasing,
/// and the difference for each step is between 1 and 3 (inclusive).
fn assess_report_safety(report: Vec<i32>) -> io::Result<bool> {
    let mut last_level: i32 = report[0];
    let report_increases: bool = report[1] > last_level;

    for level in report.iter().skip(1) {
        let diff: i32 = level - last_level;

        if diff.abs() > 3 || diff.abs() < 1 {
            return Ok(false);
        }

        let currently_increasing: bool = diff > 0;

        if currently_increasing != report_increases {
            return Ok(false)
        }

        last_level = *level;
    }
    Ok(true)
}

fn assess_reports(reports: Vec<Vec<i32>>) -> io::Result<i32> {
    let mut safe_count: i32 = 0;
    for report in reports {
        if let Ok(true) = assess_report_safety(report) {
            safe_count += 1;
        }
    }
    Ok(safe_count)
}

fn main() {
    match read_data("input/day2_1.txt") {
        Ok(reports) => {
            println!("Reports: {:?}", reports);
            match assess_reports(reports) {
                Ok(safe_count) => println!("Safe count: {}", safe_count),
                Err(e) => eprintln!("Error: {}", e),
            };
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}