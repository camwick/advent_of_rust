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

// pub fn run_part2() {
//
// }

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
