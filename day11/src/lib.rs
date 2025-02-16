use std::isize;

fn parse(path: &str) -> Vec<Vec<char>> {
    let file = std::fs::read_to_string(path).unwrap();
    file.lines().map(|line| line.chars().collect()).collect()
}

pub fn part1(path: &str) -> usize {
    let mut room = parse(path);
    let width = room[0].len();
    let height = room.len();

    loop {
        let (updated_room, changes) = simulate_round(&room, width, height);
        if changes == 0 {
            break;
        }
        room = updated_room;
    }

    room.iter().flatten().filter(|&&seat| seat == '#').count()
}

pub fn part2(path: &str) -> usize {
    let mut room = parse(path);
    let width = room[0].len();
    let height = room.len();

    loop {
        let (updated_room, changes) = simulate_round(&room, width, height);
        if changes == 0 {
            break;
        }
        room = updated_room;
    }

    room.iter().flatten().filter(|&&seat| seat == '#').count()
}

fn simulate_round(room: &[Vec<char>], width: usize, height: usize) -> (Vec<Vec<char>>, usize) {
    let mut new_room = room.to_vec();
    let mut changes = 0;

    for y in 0..height {
        for x in 0..width {
            let seat = room[y][x];
            if seat == 'L' && can_occupy((x, y), room, width, height) {
                new_room[y][x] = '#';
                changes += 1;
            } else if seat == '#' && should_empty((x, y), room, width, height) {
                new_room[y][x] = 'L';
                changes += 1;
            }
        }
    }

    (new_room, changes)
}

fn can_occupy(seat: (usize, usize), room: &[Vec<char>], width: usize, height: usize) -> bool {
    get_first_inline(seat, room, width, height)
        .iter()
        .all(|&status| status != '#')
}

fn should_empty(seat: (usize, usize), room: &[Vec<char>], width: usize, height: usize) -> bool {
    get_first_inline(seat, room, width, height)
        .iter()
        .filter(|&&status| status == '#')
        .count()
        >= 5
}

fn get_adjacent_seats(
    (x, y): (usize, usize),
    room: &[Vec<char>],
    width: usize,
    height: usize,
) -> Vec<char> {
    let mut seats = Vec::new();

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue; //skip the seat itself
            }

            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                seats.push(room[ny as usize][nx as usize]);
            }
        }
    }

    seats
}

fn get_first_inline(
    (x, y): (usize, usize),
    room: &[Vec<char>],
    width: usize,
    height: usize,
) -> Vec<char> {
    let directions = [
        (-1, -1), // Top-left
        (-1, 0),  // Top
        (-1, 1),  // Top-right
        (0, -1),  // Left
        (0, 1),   // Right
        (1, -1),  // Bottom-left
        (1, 0),   // Bottom
        (1, 1),   // Bottom-right
    ];
    let mut visibie_seats = Vec::new();

    for (dx, dy) in directions.iter() {
        let mut nx = x as isize;
        let mut ny = y as isize;
        loop {
            nx += dx;
            ny += dy;

            if nx < 0 || nx >= width as isize || ny < 0 || ny >= height as isize {
                break;
            }

            match room[ny as usize][nx as usize] {
                '#' | 'L' => {
                    visibie_seats.push(room[ny as usize][nx as usize]);
                    break;
                }
                '.' => continue,
                _ => break,
            }
        }
    }
    visibie_seats
}
