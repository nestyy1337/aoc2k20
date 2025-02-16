pub fn part1() {
    let input = parse_tiles();
    let cornerss = input
        .iter()
        .filter_map(|b| {
            let mut corners = Vec::new();
            corners.push(b.1[0].clone());
            corners.push(b.1[b.1.len() - 1].clone());
            let left: String =
                b.1.iter()
                    .map(|l| l.chars().next().unwrap().to_string())
                    .collect();
            let right: String =
                b.1.iter()
                    .map(|l| l.chars().last().unwrap().to_string())
                    .collect();
            corners.push(left);
            corners.push(right);
            let matching_sides = input
                .iter()
                .filter(|(other_id, other_tile)| b.0 != *other_id)
                .fold(0, |acc, (_, other)| {
                    let other_borders = vec![
                        other[0].to_string(),
                        other[other.len() - 1].to_string(),
                        other
                            .iter()
                            .map(|l| l.chars().next().unwrap())
                            .collect::<String>(),
                        other
                            .iter()
                            .map(|l| l.chars().last().unwrap())
                            .collect::<String>(),
                    ];

                    let matches: usize = corners
                        .iter()
                        .filter(|&border| {
                            other_borders.iter().any(|other_border| {
                                border == other_border
                                    || border == &other_border.chars().rev().collect::<String>()
                            })
                        })
                        .count();

                    acc + matches
                });
            if matching_sides == 2 {
                return Some(b.0);
            } else {
                return None;
            };
        })
        .collect::<Vec<_>>();
    println!("corners: {:?}", cornerss);
    println!("COR: {:?}", cornerss.iter().copied().product::<usize>());
}
