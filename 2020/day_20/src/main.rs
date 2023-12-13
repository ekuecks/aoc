use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::stdin;

#[derive(Clone, Debug)]
struct Tile {
    id: u64,
    borders: Vec<Vec<Square>>,
    squares: Vec<Vec<Square>>,
    top: Option<u64>,
    right: Option<u64>,
    bottom: Option<u64>,
    left: Option<u64>,
}

impl Tile {
    fn flip_vert(&mut self) {
        self.squares = self.squares.iter().cloned().rev().collect();
        let bottom = self.bottom.clone();
        self.bottom = self.top.clone();
        self.top = bottom;
        let tb = self.borders[2].clone();
        self.borders[2] = self.borders[3].clone();
        self.borders[3] = tb;
        self.borders[0] = self.borders[0].clone().into_iter().rev().collect();
        self.borders[1] = self.borders[1].clone().into_iter().rev().collect();
    }

    fn flip_horiz(&mut self) {
        self.squares = self
            .squares
            .iter()
            .cloned()
            .map(|v| v.into_iter().rev().collect())
            .collect();
        let left = self.left.clone();
        self.left = self.right.clone();
        self.right = left;
        let lb = self.borders[0].clone();
        self.borders[0] = self.borders[1].clone();
        self.borders[1] = lb;
        self.borders[2] = self.borders[2].clone().into_iter().rev().collect();
        self.borders[3] = self.borders[3].clone().into_iter().rev().collect();
    }

    fn rotate(&mut self) {
        let mut squares = Vec::new();
        for i in 0..self.squares.len() {
            squares.push(self.squares.iter().map(|v| v[i].clone()).rev().collect());
        }
        self.squares = squares;
        let top = self.top.clone();
        self.top = self.left.clone();
        let right = self.right.clone();
        self.right = top;
        let bottom = self.bottom.clone();
        self.bottom = right;
        self.left = bottom;
        let tb = self.borders[2].clone();
        self.borders[2] = self.borders[0].clone().into_iter().rev().collect();
        let rb = self.borders[1].clone();
        self.borders[1] = tb;
        let bb = self.borders[3].clone();
        self.borders[3] = rb.into_iter().rev().collect();
        self.borders[0] = bb;
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Square {
    Empty,
    Full,
}

impl TryFrom<char> for Square {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Full),
            _ => Err("Invalid char".to_owned()),
        }
    }
}

fn main() {
    let input = stdin();
    let mut prev = String::new();
    let mut line = String::new();
    let mut tiles = Vec::new();
    let mut borders = Vec::new();
    let mut squares = Vec::new();
    let mut id = 0;
    for _ in 0..3 {
        borders.push(Vec::new());
    }
    let mut first_line = false;
    loop {
        let bytes = input.read_line(&mut line).unwrap();
        if bytes == 0 {
            break;
        }
        let trimmed = line.trim();
        if trimmed.contains("Tile") {
            let mut split = trimmed[0..trimmed.len() - 1].split(" ");
            id = split.nth(1).unwrap().parse().unwrap();
            first_line = true;
        } else if trimmed.is_empty() {
            borders.push(
                prev.trim()
                    .chars()
                    .map(|c| Square::try_from(c).unwrap())
                    .collect(),
            );
            let mut tile = Tile {
                id,
                borders: borders.clone(),
                squares: squares.clone(),
                top: None,
                right: None,
                bottom: None,
                left: None,
            };
            tiles.push(tile);
            borders = Vec::new();
            for _ in 0..3 {
                borders.push(Vec::new());
            }
            squares = Vec::new();
        } else {
            borders[0].push(Square::try_from(trimmed.chars().next().unwrap()).unwrap());
            borders[1]
                .push(Square::try_from(trimmed.chars().nth(trimmed.len() - 1).unwrap()).unwrap());
            if first_line {
                borders[2] = trimmed
                    .chars()
                    .map(|c| Square::try_from(c).unwrap())
                    .collect();
                first_line = false;
            }
            squares.push(
                trimmed
                    .chars()
                    .map(|c| Square::try_from(c).unwrap())
                    .collect(),
            );
        }
        prev = line.clone();
        line = String::new();
    }
    // let result = process(tiles);
    // println!("found {}: {:?}", result.len(), result);
    // println!("{}", result.iter().fold(1, |accum, tile| accum * tile.id));
    // println!("Hello, world!");
    let result = build_graph(tiles);
    let mut strings = Vec::new();
    for row in result {
        for r in 1..9 {
            let mut s = Vec::new();
            for tile in row.iter() {
                for c in 1..9 {
                    match tile.squares[r][c] {
                        Square::Empty => {
                            s.push('.');
                        }
                        _ => {
                            s.push('#');
                        }
                    }
                }
            }
            println!("{}", s.iter().collect::<String>());
            strings.push(s);
        }
    }
    check(strings.clone());
    strings = rotate(strings);
    check(strings.clone());
    strings = rotate(strings);
    check(strings.clone());
    strings = rotate(strings);
    strings = flip(strings);
    check(strings.clone());
    strings = rotate(strings);
    check(strings.clone());
    strings = rotate(strings);
    check(strings.clone());
    strings = rotate(strings);
    check(strings.clone());
    strings = rotate(strings);
    check(strings.clone());
    let mut count = 0;
    for row in &strings {
        for c in row {
            if *c == '#' {
                count += 1;
            }
        }
    }
    println!("Starting '#': {}", count);
}

fn rotate(s: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for i in 0..s.len() {
        result.push(s.iter().map(|v| v[i].clone()).rev().collect());
    }
    result
}

fn flip(s: Vec<Vec<char>>) -> Vec<Vec<char>> {
    s.into_iter().rev().collect()
}

fn check(mut strings: Vec<Vec<char>>) {
    //  01234567890123456789
    //0:                  #
    //1:#    ##    ##    ###
    //2: #  #  #  #  #  #
    for r in 0..strings.len() - 2 {
        for c in 0..strings[0].len() - 19 {
            if strings[r][c + 18] == '#'
                && strings[r + 1][c] == '#'
                && strings[r + 1][c + 5] == '#'
                && strings[r + 1][c + 6] == '#'
                && strings[r + 1][c + 11] == '#'
                && strings[r + 1][c + 12] == '#'
                && strings[r + 1][c + 17] == '#'
                && strings[r + 1][c + 18] == '#'
                && strings[r + 1][c + 19] == '#'
                && strings[r + 2][c + 1] == '#'
                && strings[r + 2][c + 4] == '#'
                && strings[r + 2][c + 7] == '#'
                && strings[r + 2][c + 10] == '#'
                && strings[r + 2][c + 13] == '#'
                && strings[r + 2][c + 16] == '#'
            {
                strings[r][c + 18] = '0';
                strings[r + 1][c] = '0';
                strings[r + 1][c + 5] = '0';
                strings[r + 1][c + 6] = '0';
                strings[r + 1][c + 11] = '0';
                strings[r + 1][c + 12] = '0';
                strings[r + 1][c + 17] = '0';
                strings[r + 1][c + 18] = '0';
                strings[r + 1][c + 19] = '0';
                strings[r + 2][c + 1] = '0';
                strings[r + 2][c + 4] = '0';
                strings[r + 2][c + 7] = '0';
                strings[r + 2][c + 10] = '0';
                strings[r + 2][c + 13] = '0';
                strings[r + 2][c + 16] = '0';
            }
        }
    }
    let mut count = 0;
    for row in &strings {
        for c in row {
            if *c == '#' {
                count += 1;
            }
        }
    }
    println!("'#': {}", count);
}

fn process(tiles: Vec<Tile>) -> Vec<Tile> {
    let mut result = Vec::new();
    for tile in tiles.iter() {
        let mut possible_borders = 0;
        for border in tile.borders.iter() {
            let mut found = false;
            for candidate in tiles.iter() {
                if tile.id == candidate.id {
                    continue;
                }
                for candidate_border in candidate.borders.iter() {
                    if border == candidate_border {
                        if found {
                            panic!("Multiple possbile borders");
                        }
                        found = true;
                    }
                    if border
                        == &candidate_border
                            .iter()
                            .cloned()
                            .rev()
                            .collect::<Vec<Square>>()
                    {
                        if found {
                            panic!("Multiple possbile borders");
                        }
                        found = true;
                    }
                }
                if found {
                    break;
                }
            }
            if found {
                possible_borders += 1;
            }
        }
        if possible_borders == 2 {
            result.push(tile.clone());
        }
    }
    result
}

fn build_graph(tiles: Vec<Tile>) -> Vec<Vec<Tile>> {
    let mut tile_map: HashMap<u64, Tile> = HashMap::new();
    let cloned = tiles.clone();
    for tile in tiles.iter() {
        tile_map.insert(tile.id, tile.clone());
    }
    let mut twos = Vec::new();
    let mut threes = Vec::new();
    let mut fours = Vec::new();
    for tile in tiles {
        let mut possible_borders = 0;
        for (i, border) in tile.borders.iter().enumerate() {
            let mut found = false;
            for candidate in cloned.iter() {
                if tile.id == candidate.id {
                    continue;
                }
                for candidate_border in candidate.borders.iter() {
                    if border == candidate_border {
                        if found {
                            panic!("Dupes found");
                        }
                        found = true;
                        if i == 0 {
                            tile_map.get_mut(&tile.id).unwrap().left = Some(candidate.id);
                        } else if i == 1 {
                            tile_map.get_mut(&tile.id).unwrap().right = Some(candidate.id);
                        } else if i == 2 {
                            tile_map.get_mut(&tile.id).unwrap().top = Some(candidate.id);
                        } else {
                            tile_map.get_mut(&tile.id).unwrap().bottom = Some(candidate.id);
                        }
                    } else if border
                        == &candidate_border
                            .iter()
                            .cloned()
                            .rev()
                            .collect::<Vec<Square>>()
                    {
                        if found {
                            panic!("Dupes found");
                        }
                        found = true;
                        if i == 0 {
                            tile_map.get_mut(&tile.id).unwrap().left = Some(candidate.id);
                        } else if i == 1 {
                            tile_map.get_mut(&tile.id).unwrap().right = Some(candidate.id);
                        } else if i == 2 {
                            tile_map.get_mut(&tile.id).unwrap().top = Some(candidate.id);
                        } else {
                            tile_map.get_mut(&tile.id).unwrap().bottom = Some(candidate.id);
                        }
                    }
                }
            }
            if found {
                possible_borders += 1;
            }
        }
        if possible_borders == 2 {
            twos.push(tile_map.get(&tile.id).unwrap().clone());
        }
        if possible_borders == 3 {
            threes.push(tile_map.get(&tile.id).unwrap().clone());
        }
        if possible_borders == 4 {
            fours.push(tile_map.get(&tile.id).unwrap().clone());
        }
    }
    let mut map: Vec<Vec<Option<Tile>>> = Vec::new();
    for _ in 0..12 {
        let mut row = Vec::new();
        for _ in 0..12 {
            row.push(None);
        }
        map.push(row);
    }
    for node in threes.iter() {
        println!(
            "top: {:?}, right: {:?}, bottom: {:?}, left: {:?}",
            node.top, node.right, node.bottom, node.left
        );
    }
    for node in twos.iter_mut() {
        if node.right.is_some() && node.bottom.is_some() {
            println!("tl: {}", node.id);
            map[0][0] = Some(node.clone());
            break;
        }
    }
    for i in 0..12 {
        for j in 0..12 {
            if map[i][j].is_some() {
                if j != 0
                    && map[i][j].clone().unwrap().borders[0]
                        != map[i][j - 1].clone().unwrap().borders[1]
                {
                    panic!("{}, {} failed", i, j);
                }
                if i > 0
                    && map[i][j].clone().unwrap().borders[2]
                        != map[i - 1][j].clone().unwrap().borders[3]
                {
                    panic!("{}, {} failed", i, j);
                }
                continue;
            }
            if i != 0 {
                let top = &map[i - 1][j].clone().unwrap();
                let id = top
                    .bottom
                    .expect(&format!("Failed at ({}, {})\n\n{:?}", i, j, top));
                let tile = tile_map.get_mut(&id).unwrap();
                if tile.top.is_some() && tile.top.unwrap() == top.id {
                    if tile.borders[2] != top.borders[3] {
                        tile.flip_horiz();
                    }
                }
                if tile.right.is_some() && tile.right.unwrap() == top.id {
                    tile.rotate();
                    tile.rotate();
                    tile.rotate();
                    if tile.borders[2] != top.borders[3] {
                        tile.flip_horiz();
                    }
                }
                if tile.bottom.is_some() && tile.bottom.unwrap() == top.id {
                    tile.flip_vert();
                    if tile.borders[2] != top.borders[3] {
                        tile.flip_horiz();
                    }
                }
                if tile.left.is_some() && tile.left.unwrap() == top.id {
                    tile.rotate();
                    if tile.borders[2] != top.borders[3] {
                        tile.flip_horiz();
                    }
                }
                if j != 0 && tile.borders[0] != map[i][j - 1].clone().unwrap().borders[1] {
                    panic!();
                }
                if tile.borders[2] != map[i - 1][j].clone().unwrap().borders[3] {
                    panic!();
                }
                map[i][j] = Some(tile.clone());
            } else {
                let left = &map[i][j - 1].clone().unwrap();
                let id = left
                    .right
                    .expect(&format!("Failed at ({}, {})\n\n{:?}", i, j, left));
                let tile = tile_map.get_mut(&id).unwrap();
                if tile.top.is_some() && tile.top.unwrap() == left.id {
                    tile.rotate();
                    tile.rotate();
                    tile.rotate();
                    if tile.borders[0] != left.borders[1] {
                        tile.flip_vert();
                    }
                }
                if tile.right.is_some() && tile.right.unwrap() == left.id {
                    tile.flip_horiz();
                    if tile.borders[0] != left.borders[1] {
                        tile.flip_vert();
                    }
                }
                if tile.bottom.is_some() && tile.bottom.unwrap() == left.id {
                    tile.rotate();
                    if tile.borders[0] != left.borders[1] {
                        tile.flip_vert();
                    }
                }
                if tile.left.is_some() && tile.left.unwrap() == left.id {
                    if tile.borders[0] != left.borders[1] {
                        tile.flip_vert();
                    }
                }
                if j != 0 && tile.borders[0] != map[i][j - 1].clone().unwrap().borders[1] {
                    panic!();
                }
                map[i][j] = Some(tile.clone());
            }
        }
    }
    map.into_iter()
        .map(|v| v.into_iter().map(Option::unwrap).collect())
        .collect()
}
