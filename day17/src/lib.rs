use std::ops::{Deref, DerefMut};
use std::{collections::HashSet, error::Error, str::FromStr};

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Point4D {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

#[derive(Debug)]
pub struct Grid {
    inner: HashSet<Point4D>,
}
impl Grid {
    fn new() -> Self {
        Grid {
            inner: HashSet::new(),
        }
    }
}

impl Deref for Grid {
    type Target = HashSet<Point4D>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<HashSet<Point4D>> for Grid {
    fn as_ref(&self) -> &HashSet<Point4D> {
        &self.inner
    }
}

impl AsMut<HashSet<Point4D>> for Grid {
    fn as_mut(&mut self) -> &mut HashSet<Point4D> {
        &mut self.inner
    }
}

impl FromStr for Grid {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = HashSet::new();
        for (y, line) in s.lines().enumerate() {
            for (x, cell) in line.chars().enumerate() {
                if cell == '#' {
                    set.insert(Point4D {
                        x: x as i32,
                        y: y as i32,
                        z: 0,
                        w: 0,
                    });
                }
            }
        }
        Ok(Grid { inner: set })
    }
}

impl Point4D {
    fn new() -> Self {
        Point4D {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
        }
    }

    fn neighbors(&self) -> impl Iterator<Item = Point4D> {
        let (x, y, z, w) = (self.x, self.y, self.z, self.w);

        (-1..=1).flat_map(move |dx| {
            (-1..=1).flat_map(move |dy| {
                (-1..=1).flat_map(move |dz| {
                    (-1..=1).filter_map(move |dk| {
                        if dx == 0 && dy == 0 && dz == 0 && dk == 0 {
                            None
                        } else {
                            Some(Point4D {
                                x: x + dx,
                                y: y + dy,
                                z: z + dz,
                                w: w + dk,
                            })
                        }
                    })
                })
            })
        })
    }
}

// pub fn part1() {
//     let input = include_str!("./../input.txt");
//     let mut grid = Grid::from_str(input).unwrap();
//
//     for _ in 0..6 {
//         let next = iterate_once(&grid);
//         grid = next;
//     }
//     let res1 = grid.iter().count();
//     println!("{}", res1);
// }

pub fn part1() {
    let input = include_str!("./../input.txt");
    let mut grid = Grid::from_str(input).unwrap();

    for _ in 0..6 {
        let next = iterate_once(&grid);
        grid = next;
    }
    let res1 = grid.iter().count();
    println!("{}", res1);
}

pub fn iterate_once(old_grid: &Grid) -> Grid {
    let mut new_grid = Grid::new();

    let mut candidates = HashSet::new();
    for &point in old_grid.iter() {
        candidates.insert(point);
    }
    for &point in old_grid.iter() {
        for neighbor in point.neighbors() {
            candidates.insert(neighbor);
        }
    }

    for candidate in candidates {
        let count = candidate
            .neighbors()
            .filter(|&nbr| old_grid.contains(&nbr))
            .count();

        let was_active = old_grid.contains(&candidate);
        if was_active && (count == 2 || count == 3) {
            new_grid.insert(candidate);
        } else if !was_active && count == 3 {
            new_grid.insert(candidate);
        }
    }

    new_grid
}
