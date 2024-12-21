use anyhow::{Context, Result, anyhow};
use std::fs;

pub fn parse_str(path: &str) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("Failed to read file from path: {:?}", path))
}

pub fn analyze_line(line: &str) -> Result<bool> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 3 {
        return Err(anyhow!("Invalid input"));
    }
    let range = parts[0];
    let letter = parts[1].trim_end_matches(":").chars().next().unwrap();
    let pass = parts[2];

    let range_parts: Vec<&str> = range.split("-").collect();
    let min: usize = range_parts[0].parse()?;
    let max: usize = range_parts[1].parse()?;
    let range = (min, max);

    // part 1
    // if check_letter_count(pass, letter, range) {
    //     Ok(true)
    // } else {
    //     Ok(false)
    // }
    //

    // part 2
    if check_letter_pos(pass, letter, range) {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn check_letter_count(password: &str, letter: char, count_range: (usize, usize)) -> bool {
    let mut char_count = 0;
    password.chars().for_each(|chara| {
        if chara == letter {
            char_count += 1;
        }
    });

    if char_count >= count_range.0 && char_count <= count_range.1 {
        true
    } else {
        false
    }
}

fn check_letter_pos(password: &str, letter: char, positions: (usize, usize)) -> bool {
    let chars: Vec<char> = password.chars().collect();
    let pos1 = positions.0 - 1;
    let pos2 = positions.1 - 1;

    let matches_pos1 = pos1 < chars.len() && chars[pos1] == letter;
    let matches_pos2 = pos2 < chars.len() && chars[pos2] == letter;

    matches_pos1 ^ matches_pos2
}
