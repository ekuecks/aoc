use std::io::stdin;
use std::collections::HashSet;

fn main() {
    let mut count = 0;
    let mut total = 0;
    let mut common = HashSet::new();
    for line in stdin().lines() {
        let line = line.unwrap();
        let next: HashSet<_> = line.chars().collect();
        if count == 0 {
            common = next;
        } else {
            common = common.intersection(&next).into_iter().copied().collect();
        }
        count += 1;
        if count == 3 {
            assert_eq!(common.len(), 1);
            let b = common.into_iter().next().unwrap() as usize;
            let a = 'A' as usize;
            let z = 'Z' as usize;
            let lower_a = 'a' as usize;
            if b <= z {
                total += b - a + 27;
            } else {
                total += b - lower_a + 1;
            }
            common = HashSet::new();
            count = 0;
        }
    }
    dbg!(total);
}

fn _p1() {
    let mut total = 0;
    for line in stdin().lines() {
        let line = line.unwrap();
        let line: Vec<char> = line.chars().collect();
        let mid = line.len() / 2;
        let a: HashSet<char> = (&line[0..mid]).iter().copied().collect();
        let b: HashSet<char> = (&line[mid..line.len()]).iter().copied().collect();
        let intersection: Vec<char> = a.intersection(&b).into_iter().copied().collect();
        for c in intersection {
            let a = 'A' as usize;
            let z = 'Z' as usize;
            let lower_a = 'a' as usize;
            let c = c as usize;
            if c <= z {
                total += c - a + 27;
            } else {
                total += c - lower_a + 1;
            }
        }
    }
    dbg!(total);
}
