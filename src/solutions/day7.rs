use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run_part1() {
    let filename = "./input/y2024/day7/part1_input.txt";
    let mut equations: Vec<Equation> = Vec::new();
    parse_input(filename, &mut equations);

    let mut sum_of_valid_results = 0;
    for equation in &equations {
        let operators = vec!["+", "*"];

        let combinations = generate_combinations(equation.inputs.len() - 1, &operators);
        for combination in combinations {
            if evaluate_combination(&equation.inputs, &combination, true) == equation.result {
                sum_of_valid_results += equation.result;
                break;
            }
        }
    }

    println!("Total calibration result: {}", sum_of_valid_results);
}

pub fn run_part2() {
    let filename = "./input/y2024/day7/part1_input.txt";
    let mut equations: Vec<Equation> = Vec::new();
    parse_input(filename, &mut equations);


    let mut sum_of_valid_results = 0;
    for equation in &equations {
        let operators = vec!["+", "*", "||"];

        let combinations = generate_combinations(equation.inputs.len() - 1, &operators);
        for combination in combinations {
            if evaluate_combination(&equation.inputs, &combination, false) == equation.result {
                sum_of_valid_results += equation.result;
                break;
            }
        }
    }

    println!("Total calibration result: {}", sum_of_valid_results);
}

struct Equation {
    result: i64,
    inputs: Vec<i64>
}

fn parse_input(input_file: &str, equations: &mut Vec<Equation>) {
    let file = File::open(input_file).expect("Unable to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let parts: Vec<String> = line.split(':').map(|e| e.trim().to_string()).collect();

        let result: i64 = parts[0].parse().expect("Failed to parse result");
        let inputs: Vec<i64> = parts[1]
            .split(' ')
            .map(|e| e.parse().expect("Failed to parse result"))
            .collect();

        let equation = Equation { result, inputs };
        equations.push(equation);
    }
}

fn generate_combinations<'a>(length: usize, operators: &'a [&'a str]) -> Vec<Vec<&'a str>> {
    let mut combinations = Vec::new();
    let mut stack = vec![vec![]];

    while let Some(current) = stack.pop() {
        if current.len() == length {
            combinations.push(current);
        }
        else {
            for &op in operators {
                let mut next = current.clone();
                next.push(op);
                stack.push(next);
            }
        }
    }

    combinations
}

fn evaluate_combination(inputs: &[i64], operators: &[&str], is_part1: bool) -> i64 {
    let mut total = inputs[0];
    for (i, &operator) in operators.iter().enumerate() {
        if is_part1 {
            match operator {
                "+" => total += inputs[i + 1],
                "*" => total *= inputs[i + 1],
                _ => unreachable!(),
            }
        }
        else {
            match operator {
                "+" => total += inputs[i + 1],
                "*" => total *= inputs[i + 1],
                "||" => {
                    let concatenated = format!("{}{}", total, inputs[i + 1])
                        .parse::<i64>()
                        .expect("Failed to parse concatenated number");
                    total = concatenated;
                }
                _ => unreachable!(),
            }
        }
    }
    total
}
