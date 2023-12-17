use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::{BinaryHeap, HashSet};
use std::io::stdin;

fn main() {
    let mut lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    if lines[lines.len() - 1].trim().is_empty() {
        lines.pop();
    }
    let inputs: Vec<Vec<isize>> = lines
        .into_iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as isize)
                .collect::<Vec<_>>()
        })
        .collect();
    let part1 = calculate(&inputs, 1, 3);
    let part2 = calculate(&inputs, 4, 10);
    dbg!(part1);
    dbg!(part2);
}

#[derive(PartialEq, Eq, Debug)]
struct Entry {
    sofar: isize,
    r: isize,
    c: isize,
    dir: (isize, isize),
}

impl Entry {
    fn new(r: isize, c: isize, dir: (isize, isize), sofar: isize) -> Self {
        Self { r, c, sofar, dir }
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Entry) -> Ordering {
        other
            .sofar
            .cmp(&self.sofar)
            .then((self.r + self.c).cmp(&(other.r + other.c)))
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn calculate(inputs: &[Vec<isize>], min: isize, max: isize) -> Option<isize> {
    let mut q = BinaryHeap::new();
    let mut seen = HashSet::new();
    q.push(Entry::new(0, 0, (0, 1), 0));
    while let Some(entry) = q.pop() {
        let Entry {
            r,
            c,
            dir,
            sofar: mut value,
        } = entry;
        if seen.contains(&(r, c, dir)) {
            continue;
        }
        seen.insert((r, c, dir));
        if r == inputs.len() as isize - 1 && c == inputs[0].len() as isize - 1 {
            return Some(value);
        }
        for i in 1..=max {
            let ri = r + dir.0 * i;
            let ci = c + dir.1 * i;
            if ri < 0 || ri >= inputs.len() as isize || ci < 0 || ci >= inputs[0].len() as isize {
                break;
            }
            value += inputs[ri as usize][ci as usize];
            if i < min {
                continue;
            }
            let next_dirs = if dir.0 == 0 {
                [(1, 0), (-1, 0)]
            } else {
                [(0, 1), (0, -1)]
            };
            for dir in next_dirs {
                q.push(Entry::new(ri, ci, dir, value));
            }
        }
    }
    // No solution
    None
}
