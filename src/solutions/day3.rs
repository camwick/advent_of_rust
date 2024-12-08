use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run_part1() {
    let file_name = "./input/y2024/day3/part1_example.txt";
    let file = File::open(file_name).expect("Unable to open file");
    let mut reader = BufReader::new(file);

    let mut line: String = String::new();
    reader.read_line(&mut line).expect("Unable to parse line");

}

// pub fn run_part2() {
//
// }
