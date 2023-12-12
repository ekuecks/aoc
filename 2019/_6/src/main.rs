use std::collections::{HashMap, HashSet, VecDeque};
use std::io::stdin;

fn main() {
    let mut lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    if lines[lines.len() - 1].trim().is_empty() {
        lines.pop();
    }
    let mut orbits = HashMap::new();
    let mut orbited: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in lines.iter() {
        let (a, b) = line.split_once(')').unwrap();
        orbits.insert(b, a);
        orbited.entry(a).or_default().push(b);
    }
    let mut memo = HashMap::new();
    let objs: Vec<_> = orbits.keys().copied().collect();
    let mut part1 = 0;
    for obj in objs {
        part1 += compute(&orbits, obj, &mut memo);
    }
    dbg!(part1);
    let start = orbits.get("YOU").unwrap();
    let mut q = VecDeque::new();
    q.push_back((start, 0));
    let mut seen = HashSet::new();
    while let Some((p, d)) = q.pop_front() {
        if *p == "SAN" {
            dbg!(d - 1);
            break;
        }
        if let Some(o) = orbits.get(p) {
            if !seen.contains(&o) {
                q.push_back((o, d + 1));
                seen.insert(o);
            }
        }
        if let Some(ps) = orbited.get(p) {
            for p in ps.iter() {
                if !seen.contains(&p) {
                    q.push_back((p, d + 1));
                    seen.insert(p);
                }
            }
        }
    }
}

fn compute(orbits: &HashMap<&str, &str>, obj: &str, memo: &mut HashMap<&str, usize>) -> usize {
    if let Some(c) = memo.get(&obj) {
        return *c;
    }
    match orbits.get(&obj) {
        Some(orbit) => 1 + compute(orbits, orbit, memo),
        None => 0,
    }
}
