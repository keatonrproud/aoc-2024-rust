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
/// Part 2: A report is safe if after removing one item it meets the above criteria.
fn compare_level_safety(first_level: i32, next_level: i32, report_increases: bool) -> bool {
    println!("First level {}, next level {}, report_increases {}", first_level, next_level, report_increases);
    let diff: i32 = next_level - first_level;
    if diff.abs() > 3 || diff.abs() < 1 {
        println!("False");
        return false;
    }

    let currently_increasing: bool = diff > 0;
    if currently_increasing != report_increases {
        println!("False");
        return false;
    }
    println!("True");
    true
}

fn assess_report_safety(report: Vec<i32>) -> bool {
    let mut report_increases: bool = report[1] > report[0];

    println!("Report: {:?}", report);
    for (i, current_level) in report.iter().enumerate().skip(1) {
        let last_level = report[i-1];
        if let false = compare_level_safety(last_level, *current_level, report_increases) {
            println!("Report is unsafe.");
            return false;
        }
        report_increases = current_level > &last_level;
    }
    println!("Report is safe!");
    true
}

fn assess_safety_with_dampening(report: Vec<i32>) -> bool {
    /// Inspiration from a solution found online, much cleaner than my previous logic.
    /// Checks if by removing any 1 of the items, if the report is safe.
    (0..report.len()).any(|i| {
        let mut tmp = report.clone();
        tmp.remove(i);
        assess_report_safety(tmp)
    })
}

fn assess_reports(reports: Vec<Vec<i32>>) -> io::Result<i32> {
    let mut safe_count: i32 = 0;
    for report in reports {
        if let true = assess_safety_with_dampening(report) {
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