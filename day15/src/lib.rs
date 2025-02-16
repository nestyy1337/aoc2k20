use std::{collections::HashMap, mem};

pub fn part1() {
    let input = [9, 3, 1, 0, 8, 4];
    let mut memory = Vec::new();
    memory.extend_from_slice(&input);
    for turn in memory.len()..=2020 {
        let last_num = memory.last().unwrap();
        let last_pos = memory[..memory.len() - 1]
            .iter()
            .rposition(|&x| x == *last_num);

        let next_num = match last_pos {
            Some(previous) => turn - 1 - previous,
            None => 0,
        };
        memory.push(next_num);
    }
    println!("2020th number: {:?}", memory[2019]);
}

pub fn part2() {
    let input = [9, 3, 1, 0, 8, 4];
    let mut last_seen = HashMap::new();
    for (i, num) in input.iter().enumerate() {
        if i < input.len() - 1 {
            last_seen.insert(*num, i);
        }
    }
    // 4
    let mut last_num = *input.last().unwrap();
    for turn in input.len()..30000000 {
        let next_num = match last_seen.get(&last_num) {
            Some(&prev_turn) => turn - 1 - prev_turn,
            None => 0,
        };

        last_seen.insert(last_num, turn - 1);
        last_num = next_num.clone();
    }
    println!("{:?}", last_num);
}
