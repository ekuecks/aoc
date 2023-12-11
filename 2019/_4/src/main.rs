use std::io::stdin;

fn main() {
    let lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    let line = lines[0].clone();
    let (l, r) = line.trim().split_once('-').unwrap();
    let l: usize = l.parse().unwrap();
    let r: usize = r.parse().unwrap();
    let mut part1 = 0;
    for x in l..=r {
        let s = x.to_string();
        let mut prev = '\0';
        let mut pair = false;
        let mut locked = false;
        let mut in_seq = false;
        let mut inc = true;
        for c in s.chars() {
            if c == prev {
                if in_seq {
                    if !locked {
                        pair = false;
                    }
                } else {
                    pair = true;
                }
                in_seq = true;
            } else {
                in_seq = false;
                if pair {
                    locked = true;
                }
            }
            if (c as usize) < (prev as usize) {
                inc = false;
                break;
            }
            prev = c;
        }
        if pair && inc {
            part1 += 1;
        }
    }
    dbg!(part1);
}
