fn parse(line: &str) -> u64 {
    let mut b = line.to_string();

    while let Some(res) = solve_inner(&b) {
        b = res;
    }

    let res = solve_outer(&b);
    res.unwrap()
}

fn solve_outer(line: &str) -> Option<u64> {
    let x = line.split_whitespace().collect::<Vec<&str>>();

    let mut result = x[0].parse::<u64>().ok()?;

    let mut iter = x.iter().skip(1);
    while let Some(op) = iter.next() {
        if let Some(next_num_str) = iter.next() {
            let next_num = next_num_str.parse::<u64>().ok()?;
            match *op {
                "+" => result += next_num,
                "-" => result -= next_num,
                "*" => result *= next_num,
                _ => return None,
            }
        }
    }

    Some(result)
}

fn solve_addition(line: &str) -> Option<String> {
    let tokens = line.split_whitespace().collect::<Vec<&str>>();
    let mut new_tokens: Vec<String> = Vec::new();

    let mut iter = tokens.iter();
    while let Some(&token) = iter.next() {
        if token == "+" {
            if let (Some(lhs), Some(rhs)) = (new_tokens.pop(), iter.next()) {
                let lhs_num = lhs.parse::<u64>().ok()?;
                let rhs_num = rhs.parse::<u64>().ok()?;
                new_tokens.push((lhs_num + rhs_num).to_string());
            } else {
                return None;
            }
        } else {
            new_tokens.push(token.to_string());
        }
    }

    Some(new_tokens.join(" "))
}

fn solve_multiplication(line: &str) -> Option<u64> {
    let tokens = line.split_whitespace().collect::<Vec<&str>>();
    let mut result = tokens[0].parse::<u64>().ok()?;

    let mut iter = tokens.iter().skip(1);
    while let Some(&op) = iter.next() {
        if op == "*" {
            if let Some(&rhs) = iter.next() {
                let rhs_num = rhs.parse::<u64>().ok()?;
                result *= rhs_num;
            }
        }
    }

    Some(result)
}

fn solve_inner(line: &str) -> Option<String> {
    let index1 = line.rfind("(")?;
    let index2 = line[index1..].find(")")? + index1;

    let inner_expression = &line[index1 + 1..index2];
    let addition_resolved = solve_addition(inner_expression)?;

    let result = solve_multiplication(&addition_resolved)?;

    let before = &line[..index1];
    let after = &line[index2 + 1..];
    let new = format!("{}{}{}", before, result, after);

    Some(new)
}

pub fn part1() -> u64 {
    0
}
// pub fn part1() -> u64 {
//     let input = include_str!("./../input.txt");
//     let mut total = 0;
//     for line in input.lines() {
//         let res = parse(line) as u64;
//         total += res;
//     }
//     total
// }
pub fn part2() -> u64 {
    let input = include_str!("./../input.txt");
    let mut total = 0;

    for line in input.lines() {
        let mut expr = line.to_string();

        while let Some(res) = solve_inner(&expr) {
            expr = res;
        }
        if let Some(res_add) = solve_addition(&expr) {
            expr = res_add;
        }
        if let Some(result) = solve_multiplication(&expr) {
            total += result;
        }
    }

    total
}
