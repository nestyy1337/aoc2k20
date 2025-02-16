fn parse_input(path: &str) -> Vec<usize> {
    let bytes = std::fs::read_to_string(&path).expect("Failed to read the file");

    bytes
        .lines()
        .map(|line| line.parse::<usize>().expect("Failed to parse usize"))
        .collect()
}

pub fn parse_ints(path: &str) -> usize {
    let input = parse_input(path);
    let preamble_len = 25;
    for index in preamble_len..input.len() {
        let target = input[index];
        let preamble = &input[index - preamble_len..index];
        if !has_two_sum(preamble, target) {
            println!("T: {}", target);
            return target;
        }
    }
    0
}

fn has_two_sum(numbers: &[usize], target: usize) -> bool {
    for (i, &num1) in numbers.iter().enumerate() {
        for &num2 in &numbers[i + 1..] {
            if num1 + num2 == target {
                return true;
            }
        }
    }
    false
}

pub fn part2(path: &str) -> usize {
    let input = parse_input(path);
    let target = 373803594;

    for i in 0..input.len() {
        println!("Checking for index: {}", i);
        if let Some(result) = sum_till(i, &input, target) {
            return result.0 + result.1;
        }
    }
    0
}

fn sum_till(start: usize, numbers: &[usize], target: usize) -> Option<(usize, usize)> {
    let mut sum = 0;
    let mut end = start;

    while end < numbers.len() && sum <= target {
        sum += numbers[end];
        println!("Current sum: {}", sum);

        if sum == target {
            let range = &numbers[start..=end];
            let min = *range.iter().min().unwrap();
            let max = *range.iter().max().unwrap();
            println!("min: {}, max: {}", min, max);
            return Some((min, max));
        }

        end += 1;
    }

    None
}
