use std::convert::TryFrom;
use std::io::stdin;

#[derive(Eq, PartialEq)]
enum Space {
    Floor,
    Empty,
    Occupied,
}

impl TryFrom<char> for Space {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Floor),
            'L' => Ok(Self::Empty),
            '#' => Ok(Self::Occupied),
            _ => Err(format!("Invalid space: {}", c)),
        }
    }
}

impl Space {
    fn mutate(&self, neighbors: u8) -> Self {
        match self {
            Self::Floor => Self::Floor,
            Self::Empty => {
                if neighbors == 0 {
                    Self::Occupied
                } else {
                    Self::Empty
                }
            }
            Self::Occupied => {
                if neighbors >= 5 {
                    Self::Empty
                } else {
                    Self::Occupied
                }
            }
        }
    }
}

fn count_neighbors(spaces: &[Vec<Space>], row: usize, col: usize) -> u8 {
    let mut neighbors = 0;
    // Diagonal
    {
        let mut r = row;
        let mut c = col;
        while r != 0 && c != 0 {
            r -= 1;
            c -= 1;
            if spaces[r][c] == Space::Occupied {
                neighbors += 1;
                break;
            } else if spaces[r][c] == Space::Empty {
                break;
            }
        }
    }
    {
        let mut r = row;
        let mut c = col;
        while r != spaces.len() - 1 && c != spaces[0].len() - 1 {
            r += 1;
            c += 1;
            if spaces[r][c] == Space::Occupied {
                neighbors += 1;
                break;
            } else if spaces[r][c] == Space::Empty {
                break;
            }
        }
    }
    {
        let mut r = row;
        let mut c = col;
        while r != 0 && c != spaces[0].len() - 1 {
            r -= 1;
            c += 1;
            if spaces[r][c] == Space::Occupied {
                neighbors += 1;
                break;
            } else if spaces[r][c] == Space::Empty {
                break;
            }
        }
    }
    {
        let mut r = row;
        let mut c = col;
        while r != spaces.len() - 1 && c != 0 {
            r += 1;
            c -= 1;
            if spaces[r][c] == Space::Occupied {
                neighbors += 1;
                break;
            } else if spaces[r][c] == Space::Empty {
                break;
            }
        }
    }

    // Vertical
    {
        let mut r = row;
        while r != 0 {
            r -= 1;
            if spaces[r][col] == Space::Occupied {
                neighbors += 1;
                break;
            } else if spaces[r][col] == Space::Empty {
                break;
            }
        }
    }
    {
        let mut r = row;
        while r != spaces.len() - 1 {
            r += 1;
            if spaces[r][col] == Space::Occupied {
                neighbors += 1;
                break;
            } else if spaces[r][col] == Space::Empty {
                break;
            }
        }
    }

    // Horizontal
    {
        let mut c = col;
        while c != 0 {
            c -= 1;
            if spaces[row][c] == Space::Occupied {
                neighbors += 1;
                break;
            } else if spaces[row][c] == Space::Empty {
                break;
            }
        }
    }
    {
        let mut c = col;
        while c != spaces[0].len() - 1 {
            c += 1;
            if spaces[row][c] == Space::Occupied {
                neighbors += 1;
                break;
            } else if spaces[row][c] == Space::Empty {
                break;
            }
        }
    }
    neighbors
}

fn main() {
    let input = stdin();
    let mut spaces = Vec::new();
    loop {
        let mut buf = String::new();
        let bytes = input.read_line(&mut buf).unwrap();
        if bytes == 0 {
            break;
        }
        spaces.push(
            buf.trim()
                .chars()
                .into_iter()
                .map(|c| Space::try_from(c).unwrap())
                .collect::<Vec<Space>>(),
        );
    }
    let rows = spaces.len();
    let cols = spaces[0].len();
    loop {
        let mut next = Vec::new();
        for row in 0..rows {
            next.push(
                (0..cols)
                    .map(|col| {
                        let neighbors = count_neighbors(&spaces, row, col);
                        spaces[row][col].mutate(neighbors)
                    })
                    .collect::<Vec<Space>>(),
            );
        }
        if next == spaces {
            let mut occupied = 0;
            for row in spaces {
                for seat in row {
                    if seat == Space::Occupied {
                        occupied += 1;
                    }
                }
            }
            println!("{} occupied", occupied);
            return;
        }
        spaces = next;
    }
}
