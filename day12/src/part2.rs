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
struct WaypointPos {
    horizontal: Horizontal,
    vertical: Vertical,
}

#[derive(Debug)]
struct Vertical {
    units: usize,
    direction: VerticalDirection,
}

#[derive(Debug)]
struct Part2 {
    pub ship: ShipPos,
    pub waypoint: WaypointPos,
}

#[derive(Debug)]
struct ShipPos {
    horizontal: Horizontal,
    vertical: Vertical,
    current: CurrentDirection,
}

impl WaypointPos {
    pub fn new() -> Self {
        Self {
            horizontal: Horizontal {
                units: 10,
                direction: HorizontalDirection::East,
            },
            vertical: Vertical {
                units: 1,
                direction: VerticalDirection::North,
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

    fn rotate_right(&mut self) {
        let x_before = match self.horizontal.direction {
            HorizontalDirection::East => self.horizontal.units as isize,
            HorizontalDirection::West => -(self.horizontal.units as isize),
        };
        let y_before = match self.vertical.direction {
            VerticalDirection::North => self.vertical.units as isize,
            VerticalDirection::South => -(self.vertical.units as isize),
            VerticalDirection::None => 0,
        };

        let x_after = y_before;
        let y_after = -x_before;

        if x_after >= 0 {
            self.horizontal.direction = HorizontalDirection::East;
            self.horizontal.units = x_after as usize;
        } else {
            self.horizontal.direction = HorizontalDirection::West;
            self.horizontal.units = (-x_after) as usize;
        }

        if y_after > 0 {
            self.vertical.direction = VerticalDirection::North;
            self.vertical.units = y_after as usize;
        } else if y_after < 0 {
            self.vertical.direction = VerticalDirection::South;
            self.vertical.units = (-y_after) as usize;
        } else {
            self.vertical.direction = VerticalDirection::None;
            self.vertical.units = 0;
        }
    }

    fn rotate_left(&mut self) {
        let x_before = match self.horizontal.direction {
            HorizontalDirection::East => self.horizontal.units as isize,
            HorizontalDirection::West => -(self.horizontal.units as isize),
        };
        let y_before = match self.vertical.direction {
            VerticalDirection::North => self.vertical.units as isize,
            VerticalDirection::South => -(self.vertical.units as isize),
            VerticalDirection::None => 0,
        };

        let x_after = -y_before;
        let y_after = x_before;

        if x_after >= 0 {
            self.horizontal.direction = HorizontalDirection::East;
            self.horizontal.units = x_after as usize;
        } else {
            self.horizontal.direction = HorizontalDirection::West;
            self.horizontal.units = (-x_after) as usize;
        }

        if y_after > 0 {
            self.vertical.direction = VerticalDirection::North;
            self.vertical.units = y_after as usize;
        } else if y_after < 0 {
            self.vertical.direction = VerticalDirection::South;
            self.vertical.units = (-y_after) as usize;
        } else {
            self.vertical.direction = VerticalDirection::None;
            self.vertical.units = 0;
        }
    }
}

impl Part2 {
    pub fn new() -> Self {
        Part2 {
            ship: ShipPos::new(),
            waypoint: WaypointPos::new(),
        }
    }
    pub fn forward(&mut self, units: usize) {
        match self.waypoint.vertical.direction {
            VerticalDirection::North => {
                self.ship.move_north(self.waypoint.vertical.units * units);
            }
            VerticalDirection::South => {
                self.ship.move_south(self.waypoint.vertical.units * units);
            }
            _ => {}
        }
        match self.waypoint.horizontal.direction {
            HorizontalDirection::West => {
                self.ship.move_west(self.waypoint.horizontal.units * units);
            }
            HorizontalDirection::East => {
                self.ship.move_east(self.waypoint.horizontal.units * units);
            }
        }
    }
}

impl ShipPos {
    pub fn new() -> Self {
        ShipPos {
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

pub fn part2(input: &str) -> usize {
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);

    let mut path = Part2::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let (direction, rest) = line.split_at(1);
        let units = rest.parse::<usize>().unwrap();

        match direction {
            "N" => path.waypoint.move_north(units),
            "S" => path.waypoint.move_south(units),
            "E" => path.waypoint.move_east(units),
            "W" => path.waypoint.move_west(units),
            "F" => path.forward(units),
            "R" => {
                let turns = (units / 90) % 4;
                for _ in 0..turns {
                    path.waypoint.rotate_right();
                }
            }
            "L" => {
                let turns = (units / 90) % 4;
                for _ in 0..turns {
                    path.waypoint.rotate_left();
                }
            }
            _ => eprintln!("Unknown command: {}", direction),
        }
        println!("Waypoint: {:?}", path.waypoint);
        println!("Ship: {:?}", path.ship);
    }
    path.ship.vertical.units + path.ship.horizontal.units
}
