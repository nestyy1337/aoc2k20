pub fn parse_map(path: &str) -> Result<Vec<String>, anyhow::Error> {
    let mut mapped = vec![];
    let parsed = std::fs::read_to_string(path)?;
    for line in parsed.lines() {
        mapped.push(line.to_string());
    }
    Ok(mapped)
}

pub fn walk_map(map: &mut Vec<String>, step_x: usize, step_y: usize) -> i32 {
    let map_len = map[0].len();
    let mut x = 0;
    let mut y = 0;

    let mut tree_count = 0;
    for line in map.iter() {
        if y % step_y != 0 {
            y += 1;
            continue;
        }
        if let Some(char) = line.chars().nth(x % map_len) {
            if char == '#' {
                tree_count += 1;
            }
        }

        x += step_x;
        y += 1;
    }
    tree_count
}
