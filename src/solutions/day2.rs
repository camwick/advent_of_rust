use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run_part1() {
    let file_name = "./input/y2024/day2/part1_input.txt";
    let mut reports: Vec<Vec<i64>> = Vec::new();
    parse_input(file_name, &mut reports);

    let mut safe_reports = 0;
    for report in reports {
        // check if all increasing
        let mut safe = false;
        for (i, data) in report.iter().enumerate() {
           if i == report.len() - 1 {
               safe = true;
               break;
           }
           else {
                let next_data = report.get(i + 1).expect("Can't access array index");
                if data > next_data {
                    break; // not increassing
                }
                else {
                    let difference = (data - next_data).abs();
                    if difference >= 1 && difference <= 3 {
                        continue;
                    }
                    else {
                        break;
                    }
                }
           }
        }
        
        // check if all decreasing -- can probably just do this all together... too lazy for that
        if !safe {
            for (i, data) in report.iter().enumerate() {
               if i == report.len() - 1 {
                   safe = true;
                   break;
               }
               else {
                    let next_data = report.get(i + 1).expect("Can't access array index");
                    if data < next_data {
                        break; // not increassing
                    }
                    else {
                        let difference = (data - next_data).abs();
                        if difference >= 1 && difference <= 3 {
                            continue;
                        }
                        else {
                            break;
                        }
                    }
               }
            }
        }

        if safe {
            safe_reports += 1;
        }
    }

    println!("Number of safe reports: {}", safe_reports);
}

fn parse_input(input_file: &str, reports: &mut Vec<Vec<i64>>) {
    let file = File::open(input_file).expect("Unable to open file");
    let reader = BufReader::new(file);

    for line in reader.lines()
    {
        let line = line.expect("Unable to read line");

        let report: Vec<i64> = line.split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        reports.push(report);
    }
}

// not gonna lie, needed extra help on this solution.
// Rust's ownership system was really hurting me on part1 and adjusting my part1 solution for part
// 2 was near impossible with borrowing errors.
// Found a nice day 2 rust tutorial that put some perspective on my Rust knowledge... need to
// revisit the Rust Book for sure.
pub fn run_part2() {
    let file_name = "./input/y2024/day2/part1_input.txt";
    let file = File::open(file_name).expect("Unable to open file");
    let reader = BufReader::new(file);

    let result: usize = reader
        .lines()
        .filter(|line| {
            let report = line
                .as_ref()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            report_is_tolerable(&report)
        })
        .count();

    println!("Number of safe reports: {}", result as u32)
}

fn report_is_tolerable(report: &Vec<u32>) -> bool {
    if report_is_valid(report) {
        return true;
    }
    for i in 0..report.len() {
        let report_copy = [&report[0..i], &report[i + 1..]].concat();
        if report_is_valid(&report_copy) {
            return true;
        }
    }
    false
}

fn report_is_valid(report: &Vec<u32>) -> bool {
    if report.len() == 1 {
        return true;
    }
    // check ascending or descending order
    let is_ascending = report.windows(2).all(|w| w[0] <= w[1]);
    let is_descending = report.windows(2).all(|w| w[0] >= w[1]);
    if !is_ascending && !is_descending {
        return false;
    }

    // check diff is within range [1,3]
    let is_valid_range = report
        .windows(2)
        .map(|w| w[1] as i32 - w[0] as i32)
        .all(|x| x.abs() >= 1 && x.abs() <= 3);

    is_valid_range
}
