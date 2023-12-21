use std::collections::{HashSet, VecDeque};
use std::io::stdin;

fn main() {
    let mut lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    if lines[lines.len() - 1].trim().is_empty() {
        lines.pop();
    }
    let mut grid: Vec<Vec<bool>> = Vec::new();
    let mut start = (0, 0);
    for (i, line) in lines.iter().enumerate() {
        let mut row = Vec::new();
        for (j, c) in line.trim().chars().enumerate() {
            row.push(c != '#');
            if c == 'S' {
                start = (i as isize, j as isize);
            }
        }
        grid.push(row);
    }
    let mut memo = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((start, 0));
    let mut possible = HashSet::new();
    let l = grid.len() as isize;
    let k = grid[0].len() as isize;
    assert_eq!(l, k);
    let mut part1 = HashSet::new();
    while let Some(((r, c), steps)) = q.pop_front() {
        if steps == grid.len() * 2 + grid.len() / 2 {
            possible.insert((r, c));
            continue;
        }
        if steps == 64 {
            part1.insert((r, c));
        }
        if memo.contains(&((r, c), steps)) {
            continue;
        }
        memo.insert(((r, c), steps));
        let moves = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for m in moves {
            let mut ri = r;
            let mut ci = c;
            ri += m.0;
            ci += m.1;
            if grid[(((ri % l) + l) % l) as usize][(((ci % k) + k) % k) as usize] {
                q.push_back(((ri, ci), steps + 1));
            }
        }
    }
    dbg!(part1.len());
    let mut whole: u128 = 0;
    let mut u: u128 = 0;
    let mut up: u128 = 0;
    let mut down: u128 = 0;
    let mut left: u128 = 0;
    let mut right: u128 = 0;
    let mut uul: u128 = 0;
    let mut uur: u128 = 0;
    let mut ddl: u128 = 0;
    let mut ddr: u128 = 0;
    let mut ul: u128 = 0;
    let mut ur: u128 = 0;
    let mut dl: u128 = 0;
    let mut dr: u128 = 0;
    for r in 0..l {
        for c in 0..k {
            if possible.contains(&(r, c)) {
                whole += 1;
            }
            if possible.contains(&(r - 131, c)) {
                u += 1;
            }
            if possible.contains(&(r - 131 * 2, c)) {
                up += 1;
            }
            if possible.contains(&(r + 131 * 2, c)) {
                down += 1;
            }
            if possible.contains(&(r, c - 131 * 2)) {
                left += 1;
            }
            if possible.contains(&(r, c + 131 * 2)) {
                right += 1;
            }
            if possible.contains(&(r - 131 * 2, c - 131)) {
                uul += 1;
            }
            if possible.contains(&(r - 131 * 2, c + 131)) {
                uur += 1;
            }
            if possible.contains(&(r + 131 * 2, c - 131)) {
                ddl += 1;
            }
            if possible.contains(&(r + 131 * 2, c + 131)) {
                ddr += 1;
            }
            if possible.contains(&(r - 131, c - 131)) {
                ul += 1;
            }
            if possible.contains(&(r - 131, c + 131)) {
                ur += 1;
            }
            if possible.contains(&(r + 131, c - 131)) {
                dl += 1;
            }
            if possible.contains(&(r + 131, c + 131)) {
                dr += 1;
            }
        }
    }
    let goal = 26501365;
    let steps = goal / l as u128;
    assert_eq!(steps % 2, 0);
    let mut a = 1;
    let mut b = 0;
    for i in 0..steps {
        if i % 2 == 0 {
            a += i * 4;
        } else {
            b += i * 4;
        }
    }
    let part2 = whole * a
        + u * b
        + (uul + uur + ddl + ddr) * steps
        + (ul + ur + dl + dr) * (steps - 1)
        + up
        + down
        + left
        + right;
    dbg!(part2);
}
