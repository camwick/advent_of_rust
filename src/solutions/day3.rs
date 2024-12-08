use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn run_part1() {
    let file_name = "./input/y2024/day3/part1_input.txt";
    // first attempt too low: 223
    let file = File::open(file_name).expect("Unable to open file");
    let reader = BufReader::new(file);

    
    let mut antennas: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in reader.lines().enumerate() {
        let line = line.expect("Unable to read line");
        max_x = line.len() as i32;
        max_y = y as i32 + 1;
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                antennas
                    .entry(char)
                    .or_insert_with(HashSet::new)
                    .insert((x as i32, y as i32));
            }
        }
    }

    let mut antinodes = HashSet::new();

    for (_a, positions) in antennas.iter() {
        for p in positions.iter().combinations(2) {
            let p1 = *p[0];
            let p2 = *p[1];

            let (np1, np2) = generate_antinodes(p1, p2);

            if is_valid(np1, max_x, max_y) {
                antinodes.insert(np1);
            }
            if is_valid(np2, max_x, max_y) {
                antinodes.insert(np2);
            }
        }
    }

    println!("Unique antinode locations: {}", antinodes.len());
}

fn generate_antinodes(p1: (i32, i32), p2: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;

    let np1 = (p1.0 - dx, p1.1 - dy);
    let np2 = (p2.0 + dx, p2.1 + dy);
    
    (np1, np2)
}

fn is_valid(p: (i32, i32), max_x: i32, max_y: i32) -> bool {
    0 <= p.0 && p.0 < max_x && 0 <= p.1 && p.1 < max_y
}

pub fn run_part2() {
    let file_name = "./input/y2024/day3/part1_input.txt";
    let file = File::open(file_name).expect("Unable to open file");
    let reader = BufReader::new(file);

    
    let mut antennas: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in reader.lines().enumerate() {
        let line = line.expect("Unable to read line");
        max_x = line.len() as i32;
        max_y = y as i32 + 1;
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                antennas
                    .entry(char)
                    .or_insert_with(HashSet::new)
                    .insert((x as i32, y as i32));
            }
        }
    }

    let mut antinodes = HashSet::new();

    for (_a, positions) in antennas.iter() {
        for p in positions.iter().combinations(2) {
            let p1 = *p[0];
            let p2 = *p[1];

            for antinode in generate_all_antinodes(p1, p2, max_x, max_y) {
                antinodes.insert(antinode);
            }
        }
    }

    println!("Unique antinode locations: {}", antinodes.len());
}

fn generate_all_antinodes(
    p1: (i32, i32),
    p2: (i32, i32),
    max_x: i32,
    max_y: i32,
) -> HashSet<(i32, i32)> {
    let mut antinodes = HashSet::new();
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    for y3 in 0..max_y {
        for x3 in 0..max_x {
            if (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)).abs() == 0 {
                antinodes.insert((x3, y3));
            }
        }
    }

    antinodes
}
