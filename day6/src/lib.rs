use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_group() -> Result<u32> {
    let f = File::open("./input.txt").context("Failed to open file")?;
    let mut reader = BufReader::new(f);
    let mut buff = String::new();
    let mut count = 0;

    while reader.read_line(&mut buff)? > 0 {
        if buff.contains("\n\n") {
            let mut int: usize = 0b0000000000000000000000000;
            println!("New group found: {}", buff);

            for line in buff.lines() {
                if line.is_empty() {
                    continue;
                }
                // println!("Processing line: {}", line);
                let mut local_int = 0;
                line.trim_ascii().chars().for_each(|c| {
                    let val = (c as u8 - b'a') as usize;
                    // println!("Character: {}, val (bit index): {}", c, val);
                    local_int |= 1 << val;
                    // println!("local_int (after |=): {:026b}", local_int);
                });
                if int == 0 {
                    int = local_int;
                    // println!("int initialized to local_int: {:026b}", int);
                } else {
                    int &= local_int;
                    // println!("int (after &=): {:026b}", int);
                    // println!("local_int (after &=): {:026b}", local_int);
                }
            }

            let num_of_answers = int.count_ones();
            println!("Number of common answers in group: {}", num_of_answers);
            println!("current count: {}", count);
            count += num_of_answers;
            buff.clear();
        }
    }
    println!("Total count of answers: {}", count);
    Ok(count)
}
