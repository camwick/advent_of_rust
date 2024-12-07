use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run_part1() {
    let filename = "./input/y2024/day1/part1_input.txt";

    let mut list_one: Vec<i32> = Vec::new();
    let mut list_two: Vec<i32> = Vec::new();
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

    let mut list_one: Vec<i32> = Vec::new();
    let mut list_two: Vec<i32> = Vec::new();
    get_lists(filename, &mut list_one, &mut list_two);

    // create hashmap where key is the list_one entry and key is the count existing in list_two
    let mut similarity_scores = HashMap::new();
    for key in list_one.iter() {
       if list_two.contains(key) {
           similarity_scores.entry(key).and_modify(|e| { *e += 1 }).or_insert(1);
       }
    }

    // now calculate the similarity score
    let mut similarity_score: i64 = 0;
    for entry in list_one.iter() {
        let score = entry * similarity_scores.get(entry).copied().unwrap_or(0);
        similarity_score += i64::from(score);
    }

    println!("Similarity score: {}", similarity_score);
}

fn get_lists(input_file: &str, list_one: &mut Vec<i32>, list_two: &mut Vec<i32>) {
    let file = File::open(input_file).expect("Unable to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let parts: Vec<&str> = line.split(' ').collect();

        // rust's split leaves the white space... sigh
        list_one.push(parts[0].parse::<i32>().unwrap());
        list_two.push(parts[3].parse::<i32>().unwrap());
    }
}
