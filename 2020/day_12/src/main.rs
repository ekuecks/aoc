use std::io::stdin;
use std::str::FromStr;

enum Direction {
    N(i64),
    E(i64),
    S(i64),
    W(i64),
    F(i64),
    L(i64),
    R(i64),
}

impl Direction {
    fn travel(&self, ship: Position, way_point: Position) -> (Position, Position) {
        match self {
            Self::N(d) => (
                ship,
                Position {
                    x: way_point.x,
                    y: way_point.y + d,
                },
            ),
            Self::E(d) => (
                ship,
                Position {
                    x: way_point.x + d,
                    y: way_point.y,
                },
            ),
            Self::S(d) => (
                ship,
                Position {
                    x: way_point.x,
                    y: way_point.y - d,
                },
            ),
            Self::W(d) => (
                ship,
                Position {
                    x: way_point.x - d,
                    y: way_point.y,
                },
            ),
            Self::F(d) => {
                let dx = way_point.x - ship.x;
                let dy = way_point.y - ship.y;
                (
                    Position {
                        x: ship.x + dx * d,
                        y: ship.y + dy * d,
                    },
                    Position {
                        x: way_point.x + dx * d,
                        y: way_point.y + dy * d,
                    },
                )
            }
            Self::L(deg) => {
                let deg = deg % 360;
                let dx = way_point.x - ship.x;
                let dy = way_point.y - ship.y;
                let (x, y) = match deg {
                    0 => (way_point.x, way_point.y),
                    90 => (ship.x - dy, ship.y + dx),
                    180 => (ship.x - dx, ship.y - dy),
                    270 => (ship.x + dy, ship.y - dx),
                    _ => panic!("Invalid deg:{}", deg),
                };
                (ship, Position { x, y })
            }
            Self::R(deg) => {
                let deg = deg % 360;
                let dx = way_point.x - ship.x;
                let dy = way_point.y - ship.y;
                let (x, y) = match deg {
                    0 => (way_point.x, way_point.y),
                    90 => (ship.x + dy, ship.y - dx),
                    180 => (ship.x - dx, ship.y - dy),
                    270 => (ship.x - dy, ship.y + dx),
                    _ => panic!("Invalid deg:{}", deg),
                };
                (ship, Position { x, y })
            }
        }
    }
}

struct Position {
    pub x: i64,
    pub y: i64,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let s = s.trim();
        let amount: i64 = s[1..s.len()].parse().unwrap();
        match &s[0..1] {
            "N" => Ok(Self::N(amount)),
            "E" => Ok(Self::E(amount)),
            "S" => Ok(Self::S(amount)),
            "W" => Ok(Self::W(amount)),
            "F" => Ok(Self::F(amount)),
            "L" => Ok(Self::L(amount)),
            "R" => Ok(Self::R(amount)),
            _ => Err(format!("invalid direction: {}", s)),
        }
    }
}

fn main() {
    let input = stdin();
    let mut directions = Vec::new();
    loop {
        let mut buf = String::new();
        input.read_line(&mut buf).unwrap();
        if buf.is_empty() {
            break;
        }
        directions.push(Direction::from_str(&buf).unwrap());
    }
    let mut ship = Position { x: 0, y: 0 };
    let mut way_point = Position { x: 10, y: 1 };
    for direction in directions {
        let tup = direction.travel(ship, way_point);
        ship = tup.0;
        way_point = tup.1;
    }
    println!("{}", ship.x.abs() + ship.y.abs());
}
