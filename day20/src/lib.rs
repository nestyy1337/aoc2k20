use std::collections::{HashMap, HashSet};

pub fn parse_tiles<'a>() -> HashSet<Tile> {
    let input = include_str!("./../input.txt");
    let v: Vec<&str> = input.split("\n\n").map(|b| b.trim()).collect();

    // vec of id and all the lines
    let vecs: Vec<(usize, Vec<String>)> = v
        .iter()
        .map(|b| {
            let lines: Vec<String> = b.lines().map(|l| l.to_string()).collect();
            let id = lines[0]
                .split_whitespace()
                .nth(1)
                .unwrap()
                .trim_end_matches(":")
                .parse::<usize>()
                .unwrap();

            let rest = lines[1..].to_vec();
            (id, rest)
        })
        .collect();

    let mut map = HashSet::new();
    for (index, l) in vecs {
        map.insert(Tile::new(index, l.clone()));
    }
    map
}

type TileConents = Vec<String>;
type Position = (usize, usize);

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Tile {
    id: usize,
    content: TileConents,
}

impl Tile {
    fn new(id: usize, contents: TileConents) -> Self {
        Tile {
            id: id,
            content: contents,
        }
    }

    fn get_left_border(&self) -> String {
        self.content
            .iter()
            .map(|l| l.chars().next().unwrap().to_string())
            .collect()
    }

    fn get_upper_border(&self) -> String {
        self.cont2012ent[0].clone()
    }

    fn rotate90(&self) -> Self {
        let grid_len = self.content.len();
        let mut rotated = vec![vec![' '; grid_len]; grid_len];

        for (y, row) in self.content.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                rotated[x][grid_len - 1 - y] = ch;
            }
        }

        Tile {
            id: self.id,
            content: rotated
                .iter()
                .map(|row| row.iter().collect::<String>())
                .collect(),
        }
    }

    fn flip_horizontal(&self) -> Self {
        Tile {
            id: self.id,
            content: self
                .content
                .iter()
                .map(|row| row.chars().rev().collect())
                .collect(),
        }
    }

    fn flip_vertical(&self) -> Self {
        Tile {
            id: self.id,
            content: self.content.iter().rev().cloned().collect(),
        }
    }

    fn all_orientations(&self) -> Vec<Tile> {
        let mut orientations = Vec::with_capacity(8);
        let mut current = self.clone();

        for _ in 0..4 {
            orientations.push(current.clone());
            current = current.rotate90();
        }

        for i in 0..4 {
            current = orientations[i].clone();
            orientations.push(current.flip_horizontal());
        }

        orientations
    }

    fn get_right_border(&self) -> String {
        self.content
            .iter()
            .map(|l| l.chars().last().unwrap())
            .collect()
    }

    fn get_lower_border(&self) -> String {
        self.content.last().unwrap().clone()
    }
}

#[derive(Debug, Clone)]
struct Grid {
    tiles: HashMap<(usize, usize), Tile>,
    width: usize,
}

impl Grid {
    fn new(total_tiles: usize) -> Self {
        Grid {
            tiles: HashMap::new(),
            width: total_tiles,
        }
    }

    fn place(&mut self, pos: (usize, usize), tile: Tile) {
        self.tiles.insert(pos, tile);
    }

    fn get_neighbors(&self, pos: &Position) -> Vec<Option<&Tile>> {
        let mut v = Vec::new();
        if pos.1 > 0 {
            v.push(self.tiles.get(&(pos.0, pos.1 - 1)));
        } else {
            v.push(None);
        }
        if pos.0 > 0 {
            v.push(self.tiles.get(&(pos.0 - 1, pos.1)));
        } else {
            v.push(None);
        }

        v
    }

    fn get_next_pos(&self, current_pos: &Position) -> Position {
        let (x, y) = *current_pos;
        if x + 1 >= self.width {
            (0, y + 1)
        } else {
            (x + 1, y)
        }
    }

    fn fit_new_tile(&mut self, pos: Position, tile: Tile) -> Vec<Tile> {
        let orientations = tile.all_orientations();
        let neighbors = self.get_neighbors(&pos);

        orientations
            .into_iter()
            .filter(|oriented_tile| {
                if let Some(upper) = neighbors[0] {
                    if oriented_tile.get_upper_border() != upper.get_lower_border() {
                        return false;
                    }
                }
                if let Some(left) = neighbors[1] {
                    if oriented_tile.get_left_border() != left.get_right_border() {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    fn remove_borders(&mut self) {
        for ((x, y), tile) in &mut self.tiles {
            //remove borders
            tile.content.remove(0);
            tile.content.remove(tile.content.len() - 1);
            tile.content
                .iter_mut()
                .map(|l| l.remove(0))
                .collect::<String>();
            tile.content
                .iter_mut()
                .map(|l| l.remove(l.len() - 1))
                .collect::<String>();
        }
    }
    fn build_image(&self) -> Vec<String> {
        let tile_size = self.tiles.values().next().unwrap().content.len() - 2;
        let dimension = self.width;
        let full_size = tile_size * dimension;

        let mut big_image = vec![vec![' '; full_size]; full_size];

        for y_t in 0..dimension {
            for x_t in 0..dimension {
                let tile = &self.tiles[&(x_t, y_t)];

                for row_in_tile in 1..tile_size + 1 {
                    let row_str = &tile.content[row_in_tile];
                    let inner = &row_str[1..row_str.len() - 1];

                    let global_row = y_t * tile_size + (row_in_tile - 1);
                    let global_col_start = x_t * tile_size;
                    for (dx, ch) in inner.chars().enumerate() {
                        big_image[global_row][global_col_start + dx] = ch;
                    }
                }
            }
        }

        big_image
            .into_iter()
            .map(|row_of_chars| row_of_chars.into_iter().collect())
            .collect()
    }
}

fn solve_arrangement(
    pos: (usize, usize),
    current_map: Grid,
    available_tiles: &mut HashSet<Tile>,
) -> Option<Grid> {
    if available_tiles.is_empty() {
        return Some(current_map);
    }

    let mut transformed_gird = current_map.clone();

    for tile in available_tiles.iter() {
        let possible = transformed_gird.fit_new_tile(pos, tile.clone());

        for possible_tile in possible {
            transformed_gird.place(pos, possible_tile);
            let next_pos = transformed_gird.get_next_pos(&pos);

            let mut new_available = available_tiles.clone();
            new_available.remove(tile);

            if let Some(solution) =
                solve_arrangement(next_pos, transformed_gird.clone(), &mut new_available)
            {
                return Some(solution);
            }
        }
    }
    None
}

pub fn part2() {
    let mut tiles = parse_tiles();

    let mut arranged_grid = solve_arrangement((0, 0), Grid::new(12), &mut tiles).unwrap();
    let image = arranged_grid.build_image();
    let roughness = find_correct_roughness(image);
    println!("Part2 => Water Roughness = {}", roughness);
}

fn find_monsters(image: &Vec<String>) -> usize {
    // indices for the sea monster
    static SEA_MONSTER: &[(usize, usize)] = &[
        (0, 18),
        (1, 0),
        (1, 5),
        (1, 6),
        (1, 11),
        (1, 12),
        (1, 17),
        (1, 18),
        (1, 19),
        (2, 1),
        (2, 4),
        (2, 7),
        (2, 10),
        (2, 13),
        (2, 16),
    ];

    let max_row = image.len() - 3;
    let max_col = image[0].len() - 20;

    let mut count = 0;
    for r in 0..=max_row {
        for c in 0..=max_col {
            if SEA_MONSTER
                .iter()
                .all(|(dr, dc)| image[r + dr].chars().nth(c + dc) == Some('#'))
            {
                count += 1;
            }
        }
    }
    count
}

fn rotate_image_90(image: &Vec<String>) -> Vec<String> {
    let h = image.len();
    let w = image[0].len();
    let mut rotated = vec![vec![' '; h]; w];

    for r in 0..h {
        for c in 0..w {
            rotated[c][h - 1 - r] = image[r].chars().nth(c).unwrap();
        }
    }
    rotated
        .into_iter()
        .map(|row| row.into_iter().collect())
        .collect()
}

fn flip_image_horiz(image: &Vec<String>) -> Vec<String> {
    image
        .iter()
        .map(|line| line.chars().rev().collect())
        .collect()
}

fn all_transformations(mut image: Vec<String>) -> Vec<Vec<String>> {
    let mut results = Vec::new();

    for _ in 0..4 {
        results.push(image.clone());
        image = rotate_image_90(&image);
    }
    let to_flip = results.clone();
    for img in to_flip {
        results.push(flip_image_horiz(&img));
    }

    results
}

fn find_correct_roughness(image: Vec<String>) -> usize {
    let candidates = all_transformations(image);
    for oriented_image in candidates {
        let monster_count = find_monsters(&oriented_image);
        if monster_count > 0 {
            let total_hashes = oriented_image
                .iter()
                .map(|l| l.matches('#').count())
                .sum::<usize>();
            let sea_monster_hashes = 15 * monster_count;
            return total_hashes - sea_monster_hashes;
        }
    }
    0
}
