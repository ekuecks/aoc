use std::collections::HashSet;
use std::io::stdin;

fn main() {
    let lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    let mut grid = Vec::new();
    let mut part1 = 0;
    let mut part2 = 0;
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            let old = solve(&grid, 9999999999);
            part1 += old;
            'outer: for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    let c = grid[i][j];
                    if c == '.' {
                        grid[i][j] = '#';
                    } else {
                        grid[i][j] = '.';
                    }
                    let value = solve(&grid, old);
                    if value != 0 && value != old {
                        part2 += value;
                        break 'outer;
                    }
                    grid[i][j] = c;
                }
            }
            grid = Vec::new();
        }else {
            grid.push(line.chars().collect::<Vec<_>>());
        }
    }
    assert!(grid.is_empty());
    dbg!(part1);
    dbg!(part2);
}

fn can_reflect(elems: &[char], ref_point: usize) -> bool {
    for i in 0..ref_point {
        let left = elems[i];
        let right = elems.get(ref_point * 2 - i - 1);
        if right.is_none() {
            continue;
        }
        if left != *right.unwrap() {
            return false;
        }
    }
    true
}

fn reflection_points(elems: &[char]) -> HashSet<usize> {
    let mut ans = HashSet::new();
    for ref_point in 1..elems.len() {
        if can_reflect(elems, ref_point) {
            ans.insert(ref_point);
        }
    }
    ans
}

fn solve(grid: &[Vec<char>], old: usize) -> usize {
    let mut verts = Vec::new();
    for row in grid.iter() {
        verts.push(reflection_points(row));
    }
    let mut flipped = Vec::new();
    for i in 0..grid[0].len() {
        let mut cols = Vec::new();
        for row in grid.iter() {
            cols.push(row[i]);
        }
        flipped.push(cols);
    }
    let mut horiz = Vec::new();
    for row in flipped.iter() {
        horiz.push(reflection_points(row))
    }
    let mut v = verts[0].clone();
    for other in verts {
        v = v.intersection(&other).copied().collect();
    }
    let mut h = horiz[0].clone();
    for other in horiz{
        h = h.intersection(&other).copied().collect();
    }
    v.remove(&old);
    h.remove(&(old / 100));
    let ans: usize = v.into_iter().sum::<usize>() + h.into_iter().map(|x| x * 100).sum::<usize>();
    ans
}