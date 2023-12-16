use std::collections::{HashSet, VecDeque};
use std::io::stdin;

fn main() {
    let mut lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    if lines[lines.len() - 1].trim().is_empty() {
        lines.pop();
    }
    let inputs: Vec<_> = lines
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();
    let mut part2 = 0;
    for r in 0..inputs.len() as isize {
        let part1 = f(r, 0, (0, 1), &inputs);
        if r == 0 {
            dbg!(part1);
        }
        part2 = part2.max(part1);
        part2 = part2.max(f(r, inputs[0].len() as isize - 1, (0, -1), &inputs));
    }
    for c in 0..inputs[0].len() as isize {
        part2 = part2.max(f(0, c, (1, 0), &inputs));
        part2 = part2.max(f(inputs.len() as isize - 1, c, (-1, 0), &inputs));
    }
    dbg!(part2);
}

fn f(r: isize, c: isize, d: (isize, isize), inputs: &[Vec<char>]) -> usize {
    let mut q: VecDeque<((isize, isize), (isize, isize))> = VecDeque::new();
    q.push_back(((r, c), d));
    let mut memo = HashSet::new();
    let mut energized = HashSet::new();
    let mut first = true;
    while let Some((pos, dir)) = q.pop_front() {
        energized.insert(pos);
        if memo.contains(&(pos, dir)) {
            continue;
        }
        let next = if first {
            first = false;
            pos
        } else {
            memo.insert((pos, dir));
            (pos.0 + dir.0, pos.1 + dir.1)
        };
        if next.0 >= inputs.len() as isize
            || next.1 >= inputs[0].len() as isize
            || next.0 < 0
            || next.1 < 0
        {
            continue;
        }
        let (ri, ci) = next;
        let r = ri as usize;
        let c = ci as usize;
        let ch = inputs[r][c];
        if ch == '.' || (ch == '-' && dir.0 == 0) || (ch == '|' && dir.1 == 0) {
            q.push_back(((ri, ci), dir));
        } else if ch == '-' {
            q.push_back(((ri, ci), (0, -1)));
            q.push_back(((ri, ci), (0, 1)));
        } else if ch == '|' {
            q.push_back(((ri, ci), (-1, 0)));
            q.push_back(((ri, ci), (1, 0)));
        } else if ch == '\\' {
            q.push_back(((ri, ci), (dir.1, dir.0)));
        } else if ch == '/' {
            q.push_back(((ri, ci), (-dir.1, -dir.0)));
        }
    }
    energized.len()
}
