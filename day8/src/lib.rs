use advent2k20::parse_input;
use std::{collections::HashSet, usize};

pub fn part1(input_path: &str) -> i32 {
    let input = parse_input(input_path).unwrap();
    let mut seen = HashSet::new();

    let mut line_num: i32 = 0;
    let mut accumulator = 0;
    while !seen.contains(&line_num) {
        seen.insert(line_num.clone());
        let line = input.iter().nth(line_num as usize).unwrap();
        let (action, val_str) = line.split_once(" ").unwrap();
        let value = val_str
            .trim_start_matches('+')
            .parse::<i32>()
            .expect("Invalid integer");

        match action {
            "jmp" => line_num += value,
            "acc" => {
                accumulator += value;
                line_num += 1;
            }
            "nop" => line_num += 1,
            _ => unreachable!(),
        }
    }

    accumulator
}

pub fn part2(input_path: &str) -> i32 {
    let input = parse_input(input_path).unwrap();

    for i in 0..input.len() {
        let mut cloned_input = input.clone();

        if input[i].contains("nop") {
            cloned_input[i] = input[i].replace("nop", "jmp");
        } else if input[i].contains("jmp") {
            cloned_input[i] = input[i].replace("jmp", "nop");
        }
        if let Some(acc) = till_end(&cloned_input) {
            return acc;
        }
    }
    0
}

fn till_end(code: &[String]) -> Option<i32> {
    let mut line_num: i32 = 0;
    let mut seen = HashSet::new();
    let mut acc = 0;

    while (line_num as usize) < code.len() {
        if !seen.insert(line_num) {
            return None;
        }

        let line = code.iter().nth(line_num as usize).unwrap();
        let (action, val_str) = line.split_once(" ").unwrap();
        let value = val_str
            .trim_start_matches('+')
            .parse::<i32>()
            .expect("Invalid integer");

        match action {
            "jmp" => line_num += value,
            "acc" => {
                acc += value;
                line_num += 1;
            }
            "nop" => line_num += 1,
            _ => unreachable!(),
        }
    }

    Some(acc)
}
