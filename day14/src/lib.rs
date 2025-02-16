use anyhow::Error;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader},
};

fn parse(path: &str) -> Result<Vec<Vec<String>>, anyhow::Error> {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);

    let mut blocks = Vec::new();
    let mut current_blocks = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("mask =") {
            if !current_blocks.is_empty() {
                blocks.push(current_blocks);
                current_blocks = Vec::new();
            }
            current_blocks.push(line);
        } else {
            current_blocks.push(line);
        }
    }
    if !current_blocks.is_empty() {
        blocks.push(current_blocks);
    }
    Ok(blocks)
}

pub fn part1() -> usize {
    let re = Regex::new(r#"^mem\[(\d+)\] = (\d+)$"#).unwrap();
    let path = "./input.txt";
    let input = parse(path).unwrap();
    let mut mem = HashMap::new();

    for block in input {
        let mask_line = &block[0];
        let mask = mask_line
            .split(" = ")
            .nth(1)
            .unwrap()
            .bytes()
            .rev()
            .enumerate()
            .fold((usize::MAX, 0), |(and_mask, or_mask), (i, b)| match b {
                b'1' => (and_mask, or_mask | (1 << i)),
                b'0' => (and_mask & !(1 << i), or_mask),
                _ => (and_mask, or_mask),
            });

        for ins in &block[1..] {
            let captures = re.captures(ins).unwrap();
            mem.insert(
                captures[1].parse::<usize>().unwrap(),
                captures[2].parse::<usize>().unwrap() & mask.0 | mask.1,
            );
        }
    }
    mem.values().sum::<usize>()
}

pub fn part2() -> usize {
    let re = Regex::new(r#"^mem\[(\d+)\] = (\d+)$"#).unwrap();
    let path = "./input.txt";
    let input = parse(path).unwrap();
    let mut mem = HashMap::new();

    for block in input {
        let mask_line = &block[0];
        let mask = mask_line.split(" = ").nth(1).unwrap();

        let (x_positions, or_mask): (Vec<usize>, usize) = mask.bytes().rev().enumerate().fold(
            (Vec::new(), 0),
            |(mut xs, or_mask), (i, b)| match b {
                b'X' => {
                    xs.push(i);
                    (xs, or_mask)
                }
                b'1' => (xs, or_mask | (1 << i)),
                _ => (xs, or_mask),
            },
        );

        for ins in &block[1..] {
            let captures = re.captures(ins).unwrap();
            let base_addr = captures[1].parse::<usize>().unwrap() | or_mask;
            let value = captures[2].parse::<usize>().unwrap();

            for combo in 0..1 << x_positions.len() {
                let final_addr =
                    x_positions
                        .iter()
                        .enumerate()
                        .fold(base_addr, |addr, (i, &pos)| {
                            if (combo & (1 << i)) != 0 {
                                addr | (1 << pos)
                            } else {
                                addr & !(1 << pos)
                            }
                        });
                mem.insert(final_addr, value);
            }
        }
    }

    mem.values().sum()
}
