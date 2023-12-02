use std::collections::HashSet;
use std::io::stdin;

fn main() {
    let mut upper_bound = 0;
    let mut occupied = HashSet::new();
    for line in stdin().lines() {
        let line = line.unwrap();
        let l = line.trim();
        let parts: Vec<_> = l.split(" -> ").map(|s| {
            let coords: Vec<_> = s.split(',').collect();
            assert_eq!(coords.len(), 2);
            (coords[0].parse::<usize>().unwrap(), coords[1].parse::<usize>().unwrap())
        }).collect();
        for i in 0..parts.len() - 1 {
            let (sx, sy) = parts[i];
            let (ex, ey) = parts[i + 1];
            if sx == ex {
                for y in (sy.min(ey))..=(sy.max(ey)) {
                    occupied.insert((sx, y));
                }
            } else {
                for x in (sx.min(ex))..=(sx.max(ex)) {
                    occupied.insert((x, sy));
                }
            }
            upper_bound = upper_bound.max(sy);
            upper_bound = upper_bound.max(ey);
        }
    }
    let mut grains = 0;
    let mut coord = (500, 0);
    upper_bound += 2;
    loop {
        let (x, y) = coord;
        if y == upper_bound - 1 {
            occupied.insert((x, y));
            coord = (500, 0);
            grains += 1;
            continue;
        }
        let down = (x, y + 1);
        if !occupied.contains(&down) {
            coord = down;
            continue;
        }
        let dl = (x - 1, y + 1);
        if !occupied.contains(&dl) {
            coord = dl;
            continue;
        }
        let dr = (x + 1, y + 1);
        if !occupied.contains(&dr) {
            coord = dr;
            continue;
        }
        occupied.insert((x, y));
        grains += 1;
        if coord == (500, 0) {
            break;
        }
        coord = (500, 0);
    }
    println!("{grains}");
}
