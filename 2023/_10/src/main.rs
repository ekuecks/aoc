use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::stdin;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum S {
    Empty,
    Vert,
    Horiz,
    L,
    J,
    Seven,
    F,
    Start,
}

impl From<char> for S {
    fn from(c: char) -> Self {
        match c {
            '.' => S::Empty,
            '|' => S::Vert,
            '-' => S::Horiz,
            'L' => S::L,
            'J' => S::J,
            '7' => S::Seven,
            'F' => S::F,
            'S' => Self::Start,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    let inputs: Vec<Vec<S>> = lines
        .into_iter()
        .map(|l| l.trim().chars().map(S::from).collect())
        .collect();
    let mut start = (0, 0);
    'outer: for (r, l) in inputs.iter().enumerate() {
        for (c, s) in l.iter().enumerate() {
            if *s == S::Start {
                start = (r, c);
                break 'outer;
            }
        }
    }
    let mut memo = HashMap::new();
    memo.insert(start, 0);
    let mut q = VecDeque::new();
    q.push_back((start, 0));
    let mut max = 0;
    while let Some(((r, c), d)) = q.pop_front() {
        max = max.max(d);
        let v = inputs[r][c];
        let adj = match v {
            S::Empty => Vec::new(),
            S::Start => vec![(r, c - 1), (r - 1, c)],
            S::Vert => vec![(r - 1, c), (r + 1, c)],
            S::Horiz => vec![(r, c - 1), (r, c + 1)],
            S::L => vec![(r - 1, c), (r, c + 1)],
            S::J => vec![(r - 1, c), (r, c - 1)],
            S::F => vec![(r + 1, c), (r, c + 1)],
            S::Seven => vec![(r + 1, c), (r, c - 1)],
        };
        for p in adj {
            if memo.get(&p).is_some() {
                continue;
            }
            memo.insert(p, d + 1);
            q.push_back((p, d + 1));
        }
    }
    dbg!(max);
    let mut grid = vec![vec![false; inputs[0].len()]; inputs.len()];
    for &(r, c) in memo.keys() {
        grid[r][c] = true;
    }
    #[derive(Clone, Copy, Debug)]
    enum E {
        Empty,
        Pipe,
        Fake,
    }

    let mut es: Vec<Vec<E>> = vec![vec![E::Fake; 2 * inputs[0].len() + 1]; 2 * inputs.len() + 1];
    for (r, row) in grid.clone().into_iter().enumerate() {
        for (c, elem) in row.into_iter().enumerate() {
            if elem {
                let s = inputs[r][c];
                es[2 * r + 1][2 * c + 1] = E::Pipe;
                match s {
                    S::Empty => {
                        es[2 * r + 1][2 * c + 1] = E::Empty;
                    }
                    S::Vert => {
                        es[2 * r][2 * c + 1] = E::Pipe;
                        es[2 * r + 2][2 * c + 1] = E::Pipe;
                    }
                    S::Horiz => {
                        es[2 * r + 1][2 * c] = E::Pipe;
                        es[2 * r + 1][2 * c + 2] = E::Pipe;
                    }
                    S::L => {
                        es[2 * r + 1][2 * c + 1] = E::Pipe;
                        es[2 * r + 1][2 * c + 2] = E::Pipe;
                    }
                    S::J | S::Start => {
                        es[2 * r + 1][2 * c + 1] = E::Pipe;
                        es[2 * r + 1][2 * c] = E::Pipe;
                    }
                    S::Seven => {
                        es[2 * r + 2][2 * c + 1] = E::Pipe;
                        es[2 * r + 1][2 * c] = E::Pipe;
                    }
                    S::F => {
                        es[2 * r + 2][2 * c + 1] = E::Pipe;
                        es[2 * r + 1][2 * c + 2] = E::Pipe;
                    }
                }
            } else {
                es[2 * r + 1][2 * c + 1] = E::Empty;
            }
        }
    }
    let mut memo = HashMap::new();
    for (r, row) in es.clone().into_iter().enumerate() {
        for (c, elem) in row.into_iter().enumerate() {
            match elem {
                E::Fake | E::Pipe => continue,
                E::Empty => {
                    let mut seen = HashSet::new();
                    seen.insert((r, c));
                    let mut q = VecDeque::new();
                    q.push_back((r, c));
                    let mut inside = true;
                    'outer: while let Some((r, c)) = q.pop_front() {
                        if r == 0 || c == 0 || r == es.len() - 1 || c == es[0].len() - 1 {
                            inside = false;
                            break 'outer;
                        }
                        let adj = [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)];
                        for (x, y) in adj {
                            if x == 0 || y == 0 || x == es.len() - 1 || y == es[0].len() {
                                inside = false;
                                break 'outer;
                            }
                            let e = es[x][y];
                            match e {
                                E::Fake | E::Empty => {
                                    if seen.contains(&(x, y)) {
                                        continue;
                                    }
                                    if let Some(&b) = memo.get(&(x, y)) {
                                        inside = b;
                                        break 'outer;
                                    }
                                    seen.insert((x, y));
                                    q.push_back((x, y));
                                }
                                _ => {}
                            }
                        }
                    }
                    for (r, c) in seen {
                        memo.insert((r, c), inside);
                    }
                }
            }
        }
    }
    let mut part2 = 0;
    for (r, row) in grid.into_iter().enumerate() {
        for c in 0..row.len() {
            let r = 2 * r + 1;
            let c = 2 * c + 1;
            if memo.get(&(r, c)) == Some(&true) {
                part2 += 1;
            }
        }
    }
    dbg!(part2);
}
