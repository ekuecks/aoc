use std::convert::TryFrom;
use std::fmt;
use std::io::stdin;

enum Tile {
    Empty,
    Tree,
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Tree),
            _ => Err(format!("Invalid char '{}'", c)),
        }
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("."),
            Self::Tree => f.write_str("#"),
        }
    }
}

fn check_slope(board: &[Vec<Tile>], down: usize, right: usize) -> u64 {
    let mut index = 0;
    let mut trees = 0;
    let mut vertical_counter = down - 1;
    for row in board {
        vertical_counter += 1;
        if vertical_counter != down {
            continue;
        }
        vertical_counter = 0;
        match row[index] {
            Tile::Tree => trees += 1,
            _ => {}
        }
        index += right;
        if index >= row.len() {
            index -= row.len();
        }
    }
    trees
}

fn main() {
    let input = stdin();
    let mut board = Vec::new();
    loop {
        let mut buf = String::new();
        let mut row = Vec::new();
        match input.read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                for c in buf.trim().chars() {
                    row.push(Tile::try_from(c).unwrap());
                }
            }
            _ => panic!("Failed to read stdin"),
        }
        board.push(row);
    }
    let result = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .iter()
        .fold(1, |accum, (down, right)| {
            accum * check_slope(&board, *down, *right)
        });
    println!("{}", result);
}
