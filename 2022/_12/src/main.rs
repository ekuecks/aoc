use std::io::stdin;
use std::collections::{VecDeque, HashSet};

fn main() {
    let mut grid: Vec<Vec<i32>> = Vec::new();
    let mut end = (0, 0);
    for (r, line) in stdin().lines().enumerate() {
        let line = line.unwrap();
        if let Some(c) = line.find('E') {
            end = (r, c);
        }
        grid.push(line.chars().map(|c| {
            let c = if c == 'S' {
                'a'
            } else  {
                c
            };
            c as i32 - 'a' as i32
        }).collect());
    }
    let end = dbg!(end);
    grid[end.0][end.1] = 25;
    let mut queue = VecDeque::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 0 {
                queue.push_back(((r, c), 0));
            }
        }
    }
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    while let Some((cur, steps)) = queue.pop_front() {
        if cur == end {
            println!("{}", steps);
            return;
        }
        if seen.contains(&cur) {
            continue;
        }
        seen.insert(cur.clone());
        let (r, c) = cur;
        let val = grid[r][c];
        if r > 0 && grid[r-1][c] <= val + 1 {
            queue.push_back(((r-1, c), steps + 1));
        }
        if r < grid.len() - 1 && grid[r+1][c] <= val + 1 {
            queue.push_back(((r+1, c), steps + 1));
        }
        if c > 0 && grid[r][c-1] <= val + 1 {
            queue.push_back(((r, c-1), steps + 1));
        }
        if c < grid[0].len() - 1 && grid[r][c+1] <= val + 1 {
            queue.push_back(((r, c+1), steps + 1));
        }
    }
    println!("Uh-oh!");
}
