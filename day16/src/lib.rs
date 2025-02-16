use std::collections::{HashMap, HashSet};

use regex::Regex;

pub fn part1() {
    let input = include_str!(".././reqs.txt");
    let mut map = HashMap::new();
    for line in input.lines() {
        let re = Regex::new(r"^([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        if let Some(caps) = re.captures(line) {
            let rule_name = caps[1].to_string();
            let range1_start = caps[2].parse::<u32>().unwrap();
            let range1_end = caps[3].parse::<u32>().unwrap();
            let range2_start = caps[4].parse::<u32>().unwrap();
            let range2_end = caps[5].parse::<u32>().unwrap();

            map.insert(
                rule_name,
                ((range1_start, range1_end), (range2_start, range2_end)),
            );
        }
    }
    let tickets = include_str!(".././input.txt");

    let x = tickets
        //this fucker
        .replace('\n', ",")
        .split(",")
        .filter_map(|g| g.parse::<u32>().ok())
        .filter(|&num| {
            !map.values()
                .any(|k| (num >= k.0.0 && num <= k.0.1) || (num >= k.1.0 && num <= k.1.1))
        })
        .sum::<u32>();
    println!("x: {}", x);
}

pub fn part1x() {
    let input = include_str!(".././reqs.txt");
    let mut map = HashMap::new();
    for line in input.lines() {
        let re = Regex::new(r"^([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        if let Some(caps) = re.captures(line) {
            let rule_name = caps[1].to_string();
            let range1_start = caps[2].parse::<u32>().unwrap();
            let range1_end = caps[3].parse::<u32>().unwrap();
            let range2_start = caps[4].parse::<u32>().unwrap();
            let range2_end = caps[5].parse::<u32>().unwrap();
            map.insert(
                rule_name,
                ((range1_start, range1_end), (range2_start, range2_end)),
            );
        }
    }

    let tickets = include_str!(".././input.txt");
    let result = tickets
        .lines()
        .flat_map(|line| line.split(',').filter_map(|num| num.parse::<u32>().ok()))
        .filter(|&num| {
            !map.values()
                .any(|k| (num >= k.0.0 && num <= k.0.1) || (num >= k.1.0 && num <= k.1.1))
        })
        .sum::<u32>();

    println!("result: {}", result);
}

pub fn part2() {
    let input = include_str!(".././reqs.txt");
    let mut map = HashMap::new();
    for line in input.lines() {
        //why did i do tis
        let re = Regex::new(r"^(departure [a-z]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        if let Some(caps) = re.captures(line) {
            let rule_name = caps[1].to_string();
            let range1_start = caps[2].parse::<u32>().unwrap();
            let range1_end = caps[3].parse::<u32>().unwrap();
            let range2_start = caps[4].parse::<u32>().unwrap();
            let range2_end = caps[5].parse::<u32>().unwrap();
            map.insert(
                rule_name,
                ((range1_start, range1_end), (range2_start, range2_end)),
            );
        }
    }

    let tickets = include_str!(".././input.txt");
    let mut valid_tickets: Vec<Vec<u32>> = tickets
        .lines()
        .filter(|line| {
            let numbers: Vec<u32> = line
                .split(",")
                .filter_map(|num| num.parse::<u32>().ok())
                .collect();

            numbers.iter().all(|&num| {
                map.values()
                    .any(|k| (num >= k.0.0 && num <= k.0.1) || (num >= k.1.0 && num <= k.1.1))
            })
        })
        .map(|valid| {
            valid
                .split(",")
                .filter_map(|num| num.parse::<u32>().ok())
                .collect()
        })
        .collect();
    let _ = valid_tickets.pop();
    println!("valid_tickets: {:?}", valid_tickets);

    let mut possible_pos: HashMap<usize, HashSet<String>> = HashMap::new();

    for pos in 0..valid_tickets[0].len() {
        let column_values: Vec<u32> = valid_tickets.iter().map(|ticket| ticket[pos]).collect();

        for (rule_name, ranges) in &map {
            let matches = column_values.iter().all(|&num| {
                (num >= ranges.0.0 && num <= ranges.0.1) || (num >= ranges.1.0 && num <= ranges.1.1)
            });

            if matches {
                possible_pos
                    .entry(pos)
                    .or_default()
                    .insert(rule_name.clone());
            }
        }
    }
    let my_ticket = include_str!(".././ticket.txt");

    let ticket = my_ticket
        .lines()
        .next()
        .unwrap()
        .split(",")
        .filter_map(|num| num.parse::<u32>().ok())
        .collect::<Vec<u32>>();
}
