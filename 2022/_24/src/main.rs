use std::io::stdin;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

fn next_snapshot(snapshot: &HashMap<(usize, usize), Vec<Dir>>, rows: usize, cols: usize) -> HashMap<(usize, usize), Vec<Dir>> {
    let mut new_snapshot: HashMap<(usize, usize), Vec<Dir>> = HashMap::new();
    for (pos, dirs) in snapshot {
        for &dir in dirs {
            let mut col = pos.0;
            let mut row = pos.1;
            match dir {
                Dir::Right => col += 1,
                Dir::Down => row += 1,
                Dir::Left => col -= 1,
                Dir::Up => row -= 1,
            }
            if row == 0 {
                row = rows - 2;
            }
            if row == rows - 1 {
                row = 1;
            }
            if col == 0 {
                col = cols - 2;
            }
            if col == cols - 1 {
                col = 1;
            }
            new_snapshot.entry((col, row)).or_default().push(dir);
        }
    }
    // for row in 0..rows{
    //     for col in 0..cols{
    //         let c = match new_snapshot.get(&(col, row)) {
    //             Some(dirs) => if dirs.len() == 1 {
    //                 match dirs[0] {
    //                     Dir::Right => '>',
    //                     Dir::Down => 'v',
    //                     Dir::Left => '<',
    //                     Dir::Up => '^',
    //                 }
    //             } else {
    //                     char::from_digit(dirs.len() as u32, 10).unwrap()
    //             },
    //             None => '.',
    //         };
    //         print!("{}", c);
    //     }
    //     println!("");
    // }
    // println!("--------------");
    new_snapshot
}


fn main() {
    let mut blizzard_snapshots = Vec::new();
    let mut initial: HashMap<(usize, usize), Vec<Dir>> = HashMap::new();
    let mut cols = 0;
    let mut rows = 0;
    let mut walls = HashSet::new();
    for (r, line) in stdin().lines().enumerate() {
        let line = line.unwrap();
        for (col, c) in line.chars().enumerate() {
            match c {
                '>' => initial.entry((col, r)).or_default().push(Dir::Right),
                'v' => initial.entry((col, r)).or_default().push(Dir::Down),
                '<' => initial.entry((col, r)).or_default().push(Dir::Left),
                '^' => initial.entry((col, r)).or_default().push(Dir::Up),
                '#' => {
                    walls.insert((col, r));
                },
                _ => {},
            }
        }
        if r == 0 {
            cols = line.len();
        }
        rows += 1;
    }
    dbg!("{:?}", (cols, rows));
    blizzard_snapshots.push(initial);
    let start = (1, 0);
    let end = (cols - 2, rows - 1);
    let mut queue = VecDeque::new();
    queue.push_back((start, 0, false, false));
    let mut seen = HashSet::new();
    seen.insert((start, 0, false, false));
    while let Some(((col, row), index, mut reached_end, mut reached_start)) = queue.pop_front() {
        // dbg!(((col, row), index));
        if walls.contains(&(col, row)) {
            panic!("{:?}", (col, row));
        }
        if (col, row) == start && reached_end {
            reached_start = true;
        }
        if (col, row) == end {
            reached_end = true;
        }
        if (col, row) == end && reached_end && reached_start {
            dbg!(index);
            // dbg!(&path);
            break;
        }
        if index == blizzard_snapshots.len() {
            let last = &blizzard_snapshots[blizzard_snapshots.len() - 1];
            let next = next_snapshot(last, rows, cols);
            blizzard_snapshots.push(next);
        }
        let blizzards = &blizzard_snapshots[index];
        if blizzards.contains_key(&(col, row)) {
            // blizzard caught us, invalid
            continue;
        }
        let mut adjs = Vec::new();
        if col > 1 && row != 0 && row != rows - 1 {
            adjs.push((col - 1, row));
        }
        if col < cols - 2 && row != 0 && row != rows - 1 {
            adjs.push((col + 1, row));
        }
        if row > 1 || (row == 1 && col == 1) {
            adjs.push((col, row - 1));

        }
        if row < rows - 2 || (col == cols - 2 && row == rows - 2) {
            adjs.push((col, row + 1));
        }
        adjs.push((col, row));
        for (ac, ar) in adjs {
            if seen.contains(&((ac, ar), index + 1, reached_end, reached_start)) {
                continue;
            }
            seen.insert(((ac, ar), index + 1, reached_end, reached_start));
            queue.push_back(((ac, ar), index + 1, reached_end, reached_start));
        }
    }
}
