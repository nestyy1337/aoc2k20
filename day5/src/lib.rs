use anyhow::{Context, Error, Result, anyhow};
use roaring::RoaringBitmap;
// FBFBBFF_RLR
// translate in into bits?
// 0101100
// FBFBBFFRLR

fn translate_bits(input: &str) -> Result<u32, Error> {
    let binary_string: String = input
        .chars()
        .map(|c| match c {
            'B' | 'R' => Ok('1'),
            'F' | 'L' => Ok('0'),
            _ => Err(anyhow!("Invalid character: {}", c)),
        })
        .collect::<Result<String, _>>()?;

    let binary = u32::from_str_radix(&binary_string, 2)
        .map_err(|e| anyhow!("Failed to parse binary: {}", e))?;
    Ok(binary)
}

fn get_seat_id(binary: &u32) -> Result<u32, Error> {
    let column = binary & 0b111;
    let row = (binary & 0b1111111000) >> 3;
    Ok(row * 8 + column)
}

pub fn part1(vec_input: Vec<String>) -> Result<u32, Error> {
    let mut value: u32 = 0;
    for i in vec_input.iter() {
        let bits = translate_bits(&i).unwrap();
        let id = get_seat_id(&bits).unwrap();
        if id > value {
            value = id;
        }
    }
    Ok(value)
}

pub fn part2(vec_input: Vec<String>) -> Result<u32, Error> {
    let mut map = RoaringBitmap::new();
    for i in vec_input.iter() {
        let bits = translate_bits(&i).unwrap();
        let id = get_seat_id(&bits).unwrap();
        map.insert(id);
    }

    // 4 first rows are not available??
    let full_range = 0..849;
    for num in full_range {
        if !map.contains(num) {
            println!("Number {} is missing", num);
        }
    }
    Ok(1)
}
