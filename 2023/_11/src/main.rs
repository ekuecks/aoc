use std::collections::HashSet;
use std::io::stdin;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum S {
    Empty,
    Gal,
}

impl From<char> for S {
    fn from(s: char) -> Self {
        if s == '.' {
            S::Empty
        } else {
            S::Gal
        }
    }
}

fn main() {
    let lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    let inputs: Vec<Vec<_>> = lines
        .into_iter()
        .map(|s| s.trim().chars().map(S::from).collect())
        .collect();
    let mut rows_to_expand = HashSet::new();
    let mut cols_to_expand = HashSet::new();
    for c in 0..inputs[0].len() {
        cols_to_expand.insert(c);
    }
    for (r, row) in inputs.iter().enumerate() {
        rows_to_expand.insert(r);
        for (c, col) in row.iter().enumerate() {
            if *col == S::Gal {
                rows_to_expand.remove(&r);
                cols_to_expand.remove(&c);
            }
        }
    }
    let mut gals = HashSet::new();
    for (i, row) in inputs.iter().enumerate() {
        for (j, space) in row.iter().enumerate() {
            if *space == S::Gal {
                gals.insert((i, j));
            }
        }
    }
    let mut others = gals.clone();
    let mut part1: usize = 0;
    let mut part2: usize = 0;
    for g in gals {
        others.remove(&g);
        for o in others.iter() {
            let mut v1 = 0;
            let mut v2 = 0;
            let (a, b) = g;
            let (c, d) = *o;
            if a != c {
                for row in (a.min(c) + 1)..=a.max(c) {
                    if rows_to_expand.contains(&row) {
                        v1 += 2;
                        v2 += 1_000_000;
                    } else {
                        v1 += 1;
                        v2 += 1;
                    }
                }
            }
            if b != d {
                for col in (b.min(d) + 1)..=b.max(d) {
                    if cols_to_expand.contains(&col) {
                        v1 += 2;
                        v2 += 1_000_000;
                    } else {
                        v1 += 1;
                        v2 += 1;
                    }
                }
            }
            part1 += v1;
            part2 += v2;
        }
    }
    dbg!(part1);
    dbg!(part2);
}
