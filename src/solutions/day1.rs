use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run_part1() {
    let filename = "./input/y2024/day1/part1_input.txt";

    let mut list_one: Vec<i64> = Vec::new();
    let mut list_two: Vec<i64> = Vec::new();
    get_lists(filename, &mut list_one, &mut list_two);

    list_one.sort();
    list_two.sort();
    
    let mut total_distance = 0;
    for (index, list_one_entry) in list_one.iter().enumerate() {
        let list_two_entry = list_two[index];

        total_distance += (list_one_entry - list_two_entry).abs();
    }

    println!("Total distance: {}", total_distance);
}

pub fn run_part2() {
    let filename = "./input/y2024/day1/part1_input.txt";
    // incorrect attempt: 2403431

    let mut list_one: Vec<i64> = Vec::new();
    let mut list_two: Vec<i64> = Vec::new();
    get_lists(filename, &mut list_one, &mut list_two);

    // create a frequency map to total the occurences of numbers in list two
    let mut frequency_map: HashMap<i64, i64> = HashMap::new();
    for &num in &list_two {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    // calculate the similarity score by finding the frequency of a number from list 1 in list 2
    let mut similarity_score: i64 = 0;
    for &num in &list_one {
        if let Some(&count) = frequency_map.get(&num) {
            similarity_score += num * count;
        }
    }

    println!("Similarity score: {}", similarity_score);
}

fn get_lists(input_file: &str, list_one: &mut Vec<i64>, list_two: &mut Vec<i64>) {
    let file = File::open(input_file).expect("Unable to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let parts: Vec<&str> = line.split(' ').collect();

        // rust's split leaves the white space... sigh
        list_one.push(parts[0].parse::<i64>().unwrap());
        list_two.push(parts[3].parse::<i64>().unwrap());
    }
}
