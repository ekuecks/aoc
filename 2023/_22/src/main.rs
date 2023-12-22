use std::cmp::Eq;
use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::collections::{BinaryHeap, HashSet};
use std::io::stdin;

#[derive(PartialEq, Eq, Clone)]
struct Brick {
    vals: Vec<(usize, usize, usize)>,
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        let l = self.vals.iter().map(|t| t.2).min().unwrap();
        let r = other.vals.iter().map(|t| t.2).min().unwrap();
        r.cmp(&l)
    }
}

fn main() {
    let mut lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    if lines[lines.len() - 1].trim().is_empty() {
        lines.pop();
    }
    let mut bricks: Vec<Brick> = Vec::new();
    let mut xmax = 0;
    let mut ymax = 0;
    for line in lines {
        let line = line.trim();
        let (l, r) = line.split_once('~').unwrap();
        let l: Vec<_> = l.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
        let r: Vec<_> = r.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
        let mut brick = Vec::new();
        xmax = xmax.max(l[0]);
        ymax = ymax.max(l[1]);
        xmax = xmax.max(r[0]);
        ymax = ymax.max(r[1]);
        if l[0] != r[0] {
            assert!(l[1] == r[1] && l[2] == r[2]);
            for a in l[0]..=r[0] {
                brick.push((a, l[1], l[2]));
            }
        } else if l[1] != r[1] {
            assert!(l[0] == r[0] && l[2] == r[2]);
            for a in l[1]..=r[1] {
                brick.push((l[0], a, l[2]));
            }
        } else {
            assert!(l[0] == r[0] && l[1] == r[1]);
            for a in l[2]..=r[2] {
                brick.push((l[0], l[1], a));
            }
        }
        bricks.push(Brick { vals: brick });
    }
    let mut q = BinaryHeap::new();
    for brick in bricks {
        q.push(brick);
    }
    let mut occupied = HashSet::new();
    for x in 0..=xmax {
        for y in 0..=ymax {
            occupied.insert((x, y, 0));
        }
    }
    let mut bricks = Vec::new();
    while let Some(mut brick) = q.pop() {
        'outer: loop {
            for &(x, y, z) in brick.vals.iter() {
                let z = z - 1;
                if brick.vals.contains(&(x, y, z)) {
                    continue;
                }

                if occupied.contains(&(x, y, z)) {
                    break 'outer;
                }
            }
            for t in brick.vals.iter_mut() {
                t.2 -= 1;
            }
        }
        for &(x, y, z) in brick.vals.iter() {
            occupied.insert((x, y, z));
        }
        bricks.push(brick);
    }
    let mut part1 = 0;
    let mut part2 = 0;
    for i in 0..bricks.len() {
        let brick = bricks[i].clone();
        let mut occupied = occupied.clone();
        for &(x, y, z) in brick.vals.iter() {
            assert!(occupied.remove(&(x, y, z)));
        }
        let mut can_remove = true;
        for brick in bricks.iter().skip(i + 1) {
            let mut brick = brick.clone();
            let vals = brick.vals.clone();
            let mut can_fall = false;
            'outer: loop {
                for &(x, y, z) in brick.vals.iter() {
                    let z = z - 1;
                    if brick.vals.contains(&(x, y, z)) {
                        continue;
                    }

                    if occupied.contains(&(x, y, z)) {
                        break 'outer;
                    }
                }
                for t in brick.vals.iter_mut() {
                    can_fall = true;
                    t.2 -= 1;
                }
            }
            for (x, y, z) in vals {
                assert!(occupied.remove(&(x, y, z)));
            }
            for &(x, y, z) in brick.vals.iter() {
                occupied.insert((x, y, z));
            }
            if can_fall {
                can_remove = false;
                part2 += 1;
            }
        }
        if can_remove {
            part1 += 1;
        }
    }
    dbg!(part1);
    dbg!(part2);
}
