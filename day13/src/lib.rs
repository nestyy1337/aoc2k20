use gcd::binary_usize;
use std::fs;

pub fn part1() -> usize {
    let timestamp = 1003240;
    let file = fs::read_to_string("./input.txt").unwrap();
    let mut min: usize = usize::MAX;
    let mut res = 0;
    file.split(',')
        .filter_map(|x| x.trim_ascii().parse::<usize>().ok())
        .for_each(|x| {
            let wait_time = x - (timestamp % x);
            if wait_time < min {
                min = wait_time;
                res = min * x;
            }
            println!("busid: {}, min: {}, res: {}", x, min, res);
        });

    res
}

pub fn part2() -> usize {
    let file = fs::read_to_string("./input.txt").unwrap();
    let mut timestamp = 0;
    let mut step = 1;
    let buses = file
        .split(',')
        .enumerate()
        .filter_map(|(idx, val)| match val {
            "x" => None,
            _ => {
                let bus_id = val.trim_ascii().parse::<usize>().unwrap();
                Some((idx, bus_id))
            }
        })
        .collect::<Vec<(usize, usize)>>();

    for (offset, id) in buses {
        while (timestamp + offset) % id != 0 {
            timestamp += &step
        }
        step = lcm(&step, &id);
    }

    timestamp
}

fn lcm(a: &usize, b: &usize) -> usize {
    a * b / binary_usize(*a, *b)
}
