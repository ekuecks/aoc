use std::collections::HashMap;
use std::io::stdin;

fn main() {
    let mut lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    if lines[lines.len() - 1].trim().is_empty() {
        lines.pop();
    }
    let inputs1: Vec<_> = lines
        .iter()
        .map(|s| {
            let (a, b) = s.trim().split_once(' ').unwrap();
            let a: Vec<_> = a.chars().collect();
            let b: Vec<usize> = b.split(',').map(|a| a.parse().unwrap()).collect();
            (a, b)
        })
        .collect();
    let inputs2: Vec<_> = lines
        .iter()
        .map(|s| {
            let (a, b) = s.trim().split_once(' ').unwrap();
            let mut a: Vec<_> = a.chars().collect();
            let _a = a.clone();
            for _ in 0..4 {
                a.push('?');
                a.append(&mut _a.clone());
            }
            let mut b: Vec<usize> = b.split(',').map(|a| a.parse().unwrap()).collect();
            let _b = b.clone();
            for _ in 0..4 {
                b.append(&mut _b.clone());
            }
            (a, b)
        })
        .collect();
    let mut part1 = 0;
    for (a, b) in inputs1.into_iter() {
        let mut memo = HashMap::new();
        part1 += solve(a, &b, 0, 0, 0, &mut memo);
    }
    dbg!(part1);
    let mut part2 = 0;
    for (a, b) in inputs2.into_iter() {
        let mut memo = HashMap::new();
        part2 += solve(a, &b, 0, 0, 0, &mut memo);
    }
    dbg!(part2);
}

fn solve(
    mut a: Vec<char>,
    b: &[usize],
    mut idx: usize,
    mut matches: usize,
    mut sofar: usize,
    memo: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    let mut keys = Vec::new();
    let mut ans = 0;
    let mut check = true;
    let mut broken = b.get(matches).copied();
    while idx < a.len() {
        let key = (idx, matches, sofar);
        if let Some(&ans) = memo.get(&key) {
            for key in keys {
                memo.insert(key, ans);
            }
            return ans;
        }
        keys.push(key);
        let c = a[idx];
        if c == '#' {
            sofar += 1;
            if broken.is_none() || broken.unwrap() < sofar {
                return 0;
            }
        } else if c == '.' && sofar > 0 {
            if Some(sofar) != broken {
                return 0;
            } else {
                matches += 1;
                sofar = 0;
                broken = b.get(matches).copied();
            }
        }
        idx += 1;
        if c == '?' {
            match broken {
                Some(broken_n) => {
                    if sofar > 0 {
                        if broken_n > sofar {
                            a[idx - 1] = '#';
                            sofar += 1;
                            continue;
                        }
                        if broken_n < sofar {
                            unreachable!();
                        } else {
                            a[idx - 1] = '.';
                            matches += 1;
                            broken = b.get(matches).copied();
                            sofar = 0;
                        }
                    } else {
                        let mut with = a.clone();
                        with[idx - 1] = '#';
                        ans += solve(with, b, idx, matches, 1, memo);
                        let mut without = a.clone();
                        without[idx - 1] = '.';
                        ans += solve(without, b, idx, matches, 0, memo);
                        check = false;
                        break;
                    }
                }
                None => a[idx - 1] = '.',
            }
        }
    }
    if check {
        if sofar > 0 {
            if Some(sofar) == broken {
                matches += 1;
                broken = b.get(matches).copied();
            } else {
                return 0;
            }
        }
        if broken.is_none() {
            assert_eq!(ans, 0);
            ans = 1;
        }
    }
    for key in keys {
        memo.insert(key, ans);
    }
    ans
}
