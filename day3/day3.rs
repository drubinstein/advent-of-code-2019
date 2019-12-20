use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::num::ParseIntError;
use std::ops::Add;
use std::option::Option;
use std::str::FromStr;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct WirePath {
    direction: char,
    distance: i64,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Coordinate {
    x: i64,
    y: i64,
}

#[derive(Debug, Copy, Clone)]
struct WalkingCoordinate {
    coordinate: Coordinate,
    walk: i64,
}

impl PartialEq for WalkingCoordinate {
    fn eq(&self, other: &Self) -> bool {
        self.coordinate == other.coordinate
    }
}

impl Eq for WalkingCoordinate {}

impl Hash for WalkingCoordinate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        &self.coordinate.hash(state);
    }
}

impl FromStr for WirePath {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(WirePath {
            direction: s.chars().next().unwrap(),
            distance: s[1..].parse::<i64>().unwrap(),
        })
    }
}

impl fmt::Display for WirePath {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "WirePath: (direction: {}, distance: {})",
            self.direction, self.distance
        )
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Coordinate: (x: {}, y: {})", self.x, self.y)
    }
}

impl fmt::Display for WalkingCoordinate {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "WalkingCoordinate: (coordinate: {}, walk: {})",
            self.coordinate, self.walk
        )
    }
}

impl Add<Coordinate> for Coordinate {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<&WirePath> for WalkingCoordinate {
    type Output = Self;

    fn add(self, wire: &WirePath) -> Self {
        match wire.direction {
            'U' => Self {
                coordinate: Coordinate {
                    x: self.coordinate.x,
                    y: self.coordinate.y + wire.distance,
                },
                walk: self.walk + wire.distance,
            },
            'D' => Self {
                coordinate: Coordinate {
                    x: self.coordinate.x,
                    y: self.coordinate.y - wire.distance,
                },
                walk: self.walk + wire.distance,
            },
            'L' => Self {
                coordinate: Coordinate {
                    x: self.coordinate.x - wire.distance,
                    y: self.coordinate.y,
                },
                walk: self.walk + wire.distance,
            },
            'R' => Self {
                coordinate: Coordinate {
                    x: self.coordinate.x + wire.distance,
                    y: self.coordinate.y,
                },
                walk: self.walk + wire.distance,
            },
            _ => panic!("Invalid direction parsed!"),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let wire_paths: Vec<Vec<WirePath>> =
        parse_text(&fs::read_to_string("input.txt").expect("Unable to read input"));

    assert_eq!(
        WalkingCoordinate {
            coordinate: Coordinate { x: 0, y: 1 },
            walk: 0,
        },
        WalkingCoordinate {
            coordinate: Coordinate { x: 0, y: 1 },
            walk: 1,
        },
    );

    assert_eq!(part1(&parse_text("R8,U5,L5,D3\nU7,R6,D4,L4")), Some(6));
    assert_eq!(
        part1(&parse_text(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
        )),
        Some(159)
    );
    assert_eq!(
        part1(&parse_text(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
        )),
        Some(135)
    );
    println!("Solution for part 1: {:?}", part1(&wire_paths));

    assert_eq!(part2(&parse_text("R8,U5,L5,D3\nU7,R6,D4,L4")), Some(30));
    assert_eq!(
        part2(&parse_text(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
        )),
        Some(610)
    );
    assert_eq!(
        part2(&parse_text(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
        )),
        Some(410)
    );
    println!("Solution for part 2: {:?}", part2(&wire_paths));
    Ok(())
}

fn parse_text(text: &str) -> Vec<Vec<WirePath>> {
    text.split("\n")
        .map(|x| {
            x.split(',')
                .map(|wire| wire.parse::<WirePath>().unwrap())
                .collect::<Vec<WirePath>>()
        })
        .collect()
}

fn part1(wires: &Vec<Vec<WirePath>>) -> Option<i64> {
    hits(wires[0].as_slice())
        .intersection(&hits(wires[1].as_slice()))
        .map(|walk| walk.coordinate.x.abs() + walk.coordinate.y.abs())
        .min()
}

fn part2(wires: &Vec<Vec<WirePath>>) -> Option<i64> {
    let hits1 = hits(wires[0].as_slice());
    let hits2 = hits(wires[1].as_slice());
    let intersections = hits1.intersection(&hits2);
    intersections
        .map(|walk| hits1.get(walk).unwrap().walk + hits2.get(walk).unwrap().walk)
        .min()
}

fn hits(wires: &[WirePath]) -> HashSet<WalkingCoordinate> {
    let mut state = WalkingCoordinate {
        coordinate: Coordinate { x: 0, y: 0 },
        walk: 0,
    };

    wires
        .into_iter()
        .map(|wire| {
            let coordinates: Vec<WalkingCoordinate> = match wire.direction {
                'U' => (1..=wire.distance)
                    .map(|v| WalkingCoordinate {
                        coordinate: Coordinate {
                            x: state.coordinate.x,
                            y: state.coordinate.y + v,
                        },
                        walk: state.walk + v,
                    })
                    .collect(),
                'D' => (1..=wire.distance)
                    .map(|v| WalkingCoordinate {
                        coordinate: Coordinate {
                            x: state.coordinate.x,
                            y: state.coordinate.y - v,
                        },
                        walk: state.walk + v,
                    })
                    .collect(),
                'L' => (1..=wire.distance)
                    .map(|v| WalkingCoordinate {
                        coordinate: Coordinate {
                            x: state.coordinate.x - v,
                            y: state.coordinate.y,
                        },
                        walk: state.walk + v,
                    })
                    .collect(),
                'R' => (1..=wire.distance)
                    .map(|v| WalkingCoordinate {
                        coordinate: Coordinate {
                            x: state.coordinate.x + v,
                            y: state.coordinate.y,
                        },
                        walk: state.walk + v,
                    })
                    .collect(),
                _ => panic!("Invalid direction parsed! {}", wire),
            };
            state = state + wire;
            coordinates
        })
        .flatten()
        .collect()
}
