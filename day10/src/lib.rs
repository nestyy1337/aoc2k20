fn parse(path: &str) -> Vec<usize> {
    let input = std::fs::read_to_string(path).unwrap();
    let mut parsed = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    parsed.push(0);
    parsed.sort();
    parsed.push(parsed.last().unwrap() + 3);

    return parsed;
}

pub fn part1(path: &str) -> usize {
    let input = parse(path);
    let mut sigle_joints = 0;
    let mut three_joints = 0;
    for (index, val) in input.iter().enumerate() {
        if index + 1 < input.len() {
            let diff = input[index + 1] - val;
            if diff <= 3 {
                if diff == 1 {
                    sigle_joints += 1;
                } else if diff == 3 {
                    three_joints += 1;
                }
            }
        }
    }
    return sigle_joints * three_joints;
}
