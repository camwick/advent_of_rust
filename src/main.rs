mod solutions;

use std::io;

use solutions::day1;
use solutions::day2;
use solutions::day3;
use solutions::day7;
use solutions::day8;

fn main() {
    println!("Enter number of day to run");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let day_number: i32 = input.trim().parse().expect("Failed to parse day number");

    if day_number == 1 {
        day1::run_part1();
        day1::run_part2();
    }
    if day_number == 2 {
        day2::run_part1();
        day2::run_part2();
    }
    if day_number == 3 {
        day3::run_part1();
    }
    else if day_number == 7 {
        day7::run_part1();
        day7::run_part2();
    }
    if day_number == 8 {
        day8::run_part1();
        day8::run_part2();
    }
}

