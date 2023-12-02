use std::io::stdin;

#[derive(Debug, PartialEq, Eq)]
enum Space {
    Void,
    Open,
    Wall,
}

impl Space {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Open,
            '#' => Self::Wall,
            ' ' => Self::Void,
            _ => panic!("Invalid char '{c}'"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

fn main() {
    let lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    let mut grid = Vec::new();
    for line in lines.iter().take(lines.len() - 2) {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(Space::from_char(c));
        }
        grid.push(row);
    }
    let mut instructions = lines[lines.len() - 1].clone().trim().to_string();
    instructions.push('X');
    let mut facing = Facing::Right;
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    for i in 0..grid[0].len() {
        if grid[0][i] == Space::Open {
            x = i as i64;
            break;
        }
    }
    dbg!(x);
    let mut len = grid[0].len();
    for row in &grid {
        len = len.max(row.len());
    }
    for row in grid.iter_mut() {
        for _ in 0..(len - row.len()) {
            row.push(Space::Void);
        }
        assert_eq!(row.len(), len);
    }
    let mut num = 0;
    for c in instructions.trim().chars() {
        match c {
            'R' | 'L' | 'X' => {
                // move
                for _ in 0..num {
                    let (dx, dy) = match facing {
                        Facing::Right => (1, 0),
                        Facing::Down => (0, 1),
                        Facing::Left => (-1, 0),
                        Facing::Up => (0, -1),
                    };
                    let mut new_x = x;
                    let mut new_y = y;
                    let mut new_facing = facing;
                    new_x += dx;
                    new_y += dy;
                    //  AB
                    //  C
                    // DE
                    // F
                    //
                    // A UP => F RIGHT
                    // B UP => F UP
                    // B RIGHT => E LEFT
                    // D LEFT => A RIGHT
                    // F DOWN => B DOWN
                    // F LEFT => A DOWN
                    if new_y == grid.len() as i64 {
                        assert_eq!(facing, Facing::Down);
                        new_y = 0;
                        // F DOWN => B DOWN
                        new_x += 100;
                    } else if new_y == -1 {
                        assert_eq!(facing, Facing::Up);
                        if x < 100 {
                            // A UP => F RIGHT
                            new_facing = Facing::Right;
                            new_y = 150 + new_x - 50;
                            new_x = 0;
                        } else {
                            // B UP => F UP
                            new_y = 199;
                            new_x -= 100;
                        }
                    }
                    if new_x == len as i64 {
                        assert_eq!(facing, Facing::Right);
                        // B RIGHT => E LEFT
                        new_x = 99;
                        new_y = 100 + (49 - new_y);
                        new_facing = Facing::Left;
                    } else if new_x == -1 {
                        assert_eq!(facing, Facing::Left);
                        if new_y < 150 {
                            // D LEFT => A RIGHT
                            new_facing = Facing::Right;
                            new_y = 149 - new_y;
                            new_x = 50;
                        } else {
                            // F LEFT => A DOWN
                            new_facing = Facing::Down;
                            new_x = 50 + new_y - 150;
                            new_y = 0;
                        }
                    }
                    if grid[new_y as usize][new_x as usize] == Space::Void {
                        //  AB
                        //  C
                        // DE
                        // F
                        //
                        // A LEFT => D RIGHT
                        // B DOWN => C LEFT
                        // C LEFT => D DOWN
                        // C RIGHT => B UP
                        // E RIGHT => B LEFT
                        // E DOWN => F LEFT
                        // D UP => C RIGHT
                        // F RIGHT => E UP
                        if facing == Facing::Left {
                            if new_y < 50 {
                                // A LEFT => D RIGHT
                                new_y = 100 + (49 - new_y);
                                new_x = 0;
                                new_facing = Facing::Right;
                            } else {
                                assert!(new_y < 100);
                                // C LEFT => D DOWN
                                new_x = new_y - 50;
                                new_y = 100;
                                new_facing = Facing::Down;
                            }
                        } else if facing == Facing::Right {
                            if new_y < 100 {
                                // C RIGHT => B UP
                                new_x = new_y + 50;
                                new_y = 49;
                                new_facing = Facing::Up;
                            } else if new_y < 150 {
                                // E RIGHT => B LEFT
                                new_y = 149 - new_y;
                                new_x = 149;
                                new_facing = Facing::Left;
                            } else {
                                // F RIGHT => E UP
                                new_x = 50 + new_y - 150;
                                new_y = 149;
                                new_facing = Facing::Up;
                            }
                        } else if facing == Facing::Up {
                            // D UP => C RIGHT
                            new_y = 50 + new_x;
                            new_x = 50;
                            new_facing = Facing::Right;
                        } else if new_x < 100 {
                            // E DOWN => F LEFT
                            new_y = 150 + new_x - 50;
                            new_x = 49;
                            new_facing = Facing::Left;
                        } else {
                            // B DOWN => C LEFT
                            new_y = 50 + new_x - 100;
                            new_x = 99;
                            new_facing = Facing::Left;
                        }
                    }
                    if grid[new_y as usize][new_x as usize] == Space::Wall {
                        break;
                    }
                    x = new_x;
                    y = new_y;
                    facing = new_facing;
                }
                if c == 'R' {
                    facing = match facing {
                        Facing::Right => Facing::Down,
                        Facing::Down => Facing::Left,
                        Facing::Left => Facing::Up,
                        Facing::Up => Facing::Right,
                    };
                } else if c == 'L' {
                    facing = match facing {
                        Facing::Right => Facing::Up,
                        Facing::Up => Facing::Left,
                        Facing::Left => Facing::Down,
                        Facing::Down => Facing::Right,
                    };
                }
                num = 0;
            }
            _ => {
                num *= 10;
                num += c.to_digit(10).unwrap();
            }
        }
    }
    let dir_val = match facing {
        Facing::Right => 0,
        Facing::Down => 1,
        Facing::Left => 2,
        Facing::Up => 3,
    };
    dbg!(1000 * (y + 1) + 4 * (x + 1) + dir_val);
}
