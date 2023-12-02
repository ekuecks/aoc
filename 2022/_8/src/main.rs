use std::io::stdin;


fn main() {
    let mut trees = Vec::new();
    for line in stdin().lines() {
        let line = line.unwrap();
        trees.push(line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<_>>());
    }
    //part1(trees)
    part2(trees)
}

fn part2(trees: Vec<Vec<i32>>) {
    let height = trees.len();
    let width = trees[0].len();
    let mut max = 0;
    for r in 0..height {
        for c in 0..width {
            let tree = trees[r][c];
            let mut left = 0;
            for i in (0..c).rev() {
                left += 1;
                if trees[r][i] >= tree {
                    break;
                }
            }
            let mut right = 0;
            for i in c+1..width {
                right += 1;
                if trees[r][i] >= tree {
                    break;
                }
            }
            let mut up = 0;
            for i in (0..r).rev() {
                up += 1;
                if trees[i][c] >= tree {
                    break;
                }
            }
            let mut down = 0;
            for i in r+1..height {
                down += 1;
                if trees[i][c] >= tree {
                    break;
                }
            }

            let score = left * right * up * down;
            max = max.max(score);
        }
    }
    dbg!(max);
}

fn part1(trees: Vec<Vec<i32>>) {
    let mut visible = Vec::new();
    for row in &trees {
        visible.push(vec![false; row.len()]);
    }
    // left
    for (r, row) in trees.iter().enumerate() {
        let mut max = -1;
        for (c, &tree) in row.iter().enumerate() {
            if tree > max {
                max = tree;
                visible[r][c] = true;
            }
        } 
    }
    // right 
    for (r, row) in trees.iter().enumerate() {
        let mut max = -1;
        for (c, &tree) in row.iter().enumerate().rev() {
            if tree > max {
                max = tree;
                visible[r][c] = true;
            }
        } 
    }
    // down 
    for c in 0..trees[0].len() {
        let mut max = -1;
        for r in 0..trees.len() {
            let tree = trees[r][c];
            if tree > max {
                max = tree;
                visible[r][c] = true;
            }
        }
    }
    // up 
    for c in 0..trees[0].len() {
        let mut max = -1;
        for r in (0..trees.len()).rev() {
            let tree = trees[r][c];
            if tree > max {
                max = tree;
                visible[r][c] = true;
            }
        }
    }
    let mut count = 0;
    for row in &visible {
        for &col in row {
            if col {
                count += 1;
            }
        }
    }
    // dbg!(visible);
    dbg!(count);
}
