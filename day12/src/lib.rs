pub mod part2;

use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum HorizontalDirection {
    West,
    East,
}

#[derive(Debug)]
enum VerticalDirection {
    North,
    South,
    None,
}

#[derive(Debug)]
enum CurrentDirection {
    West,
    East,
    North,
    South,
}

#[derive(Debug)]
struct Horizontal {
    units: usize,
    direction: HorizontalDirection,
}

#[derive(Debug)]
struct Vertical {
    units: usize,
    direction: VerticalDirection,
}

#[derive(Debug)]
struct Path {
    horizontal: Horizontal,
    vertical: Vertical,
    current: CurrentDirection,
}

impl Path {
    pub fn new() -> Self {
        Path {
            horizontal: Horizontal {
                units: 0,
                direction: HorizontalDirection::East,
            },
            vertical: Vertical {
                units: 0,
                direction: VerticalDirection::None,
            },
            current: CurrentDirection::East,
        }
    }

    pub fn forward(&mut self, units: usize) {
        match self.current {
            CurrentDirection::West => match self.horizontal.direction {
                HorizontalDirection::West => self.horizontal.units += units,
                HorizontalDirection::East => {
                    if self.horizontal.units >= units {
                        self.horizontal.units -= units;
                    } else {
                        self.horizontal.direction = HorizontalDirection::West;
                        self.horizontal.units = units - self.horizontal.units;
                    }
                }
            },
            CurrentDirection::East => match self.horizontal.direction {
                HorizontalDirection::East => self.horizontal.units += units,
                HorizontalDirection::West => {
                    if self.horizontal.units >= units {
                        self.horizontal.units -= units;
                    } else {
                        self.horizontal.direction = HorizontalDirection::East;
                        self.horizontal.units = units - self.horizontal.units;
                    }
                }
            },
            CurrentDirection::North => match self.vertical.direction {
                VerticalDirection::North => self.vertical.units += units,
                VerticalDirection::South => {
                    if self.vertical.units >= units {
                        self.vertical.units -= units;
                    } else {
                        self.vertical.direction = VerticalDirection::North;
                        self.vertical.units = units - self.vertical.units;
                    }
                }
                VerticalDirection::None => {
                    self.vertical.direction = VerticalDirection::North;
                    self.vertical.units = units;
                }
            },
            CurrentDirection::South => match self.vertical.direction {
                VerticalDirection::South => self.vertical.units += units,
                VerticalDirection::North => {
                    if self.vertical.units >= units {
                        self.vertical.units -= units;
                    } else {
                        self.vertical.direction = VerticalDirection::South;
                        self.vertical.units = units - self.vertical.units;
                    }
                }
                VerticalDirection::None => {
                    self.vertical.direction = VerticalDirection::South;
                    self.vertical.units = units;
                }
            },
        }
    }

    pub fn move_north(&mut self, units: usize) {
        match self.vertical.direction {
            VerticalDirection::North => {
                self.vertical.units += units;
            }
            VerticalDirection::South => {
                if self.vertical.units >= units {
                    self.vertical.units -= units;
                } else {
                    self.vertical.direction = VerticalDirection::North;
                    self.vertical.units = units - self.vertical.units;
                }
            }
            VerticalDirection::None => {
                self.vertical.direction = VerticalDirection::North;
                self.vertical.units = units;
            }
        }
    }

    pub fn move_south(&mut self, units: usize) {
        match self.vertical.direction {
            VerticalDirection::South => {
                self.vertical.units += units;
            }
            VerticalDirection::North => {
                if self.vertical.units >= units {
                    self.vertical.units -= units;
                } else {
                    self.vertical.direction = VerticalDirection::South;
                    self.vertical.units = units - self.vertical.units;
                }
            }
            VerticalDirection::None => {
                self.vertical.direction = VerticalDirection::South;
                self.vertical.units = units;
            }
        }
    }

    pub fn move_east(&mut self, units: usize) {
        match self.horizontal.direction {
            HorizontalDirection::East => {
                self.horizontal.units += units;
            }
            HorizontalDirection::West => {
                if self.horizontal.units >= units {
                    self.horizontal.units -= units;
                } else {
                    self.horizontal.direction = HorizontalDirection::East;
                    self.horizontal.units = units - self.horizontal.units;
                }
            }
        }
    }

    pub fn move_west(&mut self, units: usize) {
        match self.horizontal.direction {
            HorizontalDirection::West => {
                self.horizontal.units += units;
            }
            HorizontalDirection::East => {
                if self.horizontal.units >= units {
                    self.horizontal.units -= units;
                } else {
                    self.horizontal.direction = HorizontalDirection::West;
                    self.horizontal.units = units - self.horizontal.units;
                }
            }
        }
    }

    pub fn rotate_right(&mut self) {
        match self.current {
            CurrentDirection::West => self.current = CurrentDirection::North,
            CurrentDirection::North => self.current = CurrentDirection::East,
            CurrentDirection::East => self.current = CurrentDirection::South,
            CurrentDirection::South => self.current = CurrentDirection::West,
        }
    }

    pub fn rotate_left(&mut self) {
        match self.current {
            CurrentDirection::West => self.current = CurrentDirection::South,
            CurrentDirection::South => self.current = CurrentDirection::East,
            CurrentDirection::East => self.current = CurrentDirection::North,
            CurrentDirection::North => self.current = CurrentDirection::West,
        }
    }
}

pub fn part1(input: &str) -> usize {
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);

    let mut path = Path::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let (direction, rest) = line.split_at(1);
        let units = rest.parse::<usize>().unwrap();

        match direction {
            "N" => path.move_north(units),
            "S" => path.move_south(units),
            "E" => path.move_east(units),
            "W" => path.move_west(units),
            "F" => path.forward(units),
            "R" => {
                let turns = (units / 90) % 4;
                for _ in 0..turns {
                    path.rotate_right();
                }
            }
            "L" => {
                let turns = (units / 90) % 4;
                for _ in 0..turns {
                    path.rotate_left();
                }
            }
            _ => eprintln!("Unknown command: {}", direction),
        }
    }
    path.vertical.units + path.horizontal.units
}
