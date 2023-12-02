use std::io::stdin;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut elves = HashSet::new();
    let mut minx = 100000;
    let mut miny = 100000;
    let mut maxx = 0;
    let mut maxy = 0;
    for (y, line) in stdin().lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert((x as i32, y as i32));
            }
        }
    }
    let mut order = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
    for cycle in 1..10000000000000000_u64 {
        let mut proposals: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
        let mut minx = 100000;
        let mut miny = 100000;
        let mut maxx = 0;
        let mut maxy = 0;
        for (x, y) in elves.iter() {
            let x = *x;
            let y = *y;
            let initial_check = vec![
                (x - 1, y - 1),
                (x - 1, y),
                (x - 1, y + 1),
                (x, y - 1),
                (x, y + 1),
                (x + 1, y - 1),
                (x + 1, y),
                (x + 1, y + 1),
            ];
            let mut found = false;
            for pos in initial_check {
                if elves.contains(&pos) {
                    found = true;
                    break;
                }
            }
            if !found {
                // dont move
                continue;
            }
            for (dx, dy) in order.iter() {
                let dx = *dx;
                let dy = *dy;
                let candidates = if dx == 0 {
                    vec![(x - 1, y + dy), (x, y + dy), (x + 1, y + dy)]
                } else {
                    vec![(x + dx, y - 1), (x + dx, y), (x + dx, y + 1)]
                };
                let mut found = false;
                for candidate in candidates {
                    if elves.contains(&candidate) {
                        found = true;
                        break;
                    }
                }
                if !found {
                    proposals.entry((x + dx, y + dy)).or_default().push((x, y));
                    break;
                }
            }
        }
        let mut moved = false;
        for (new, olds) in proposals {
            if olds.len() == 1 {
                moved = true;
                elves.remove(&olds[0]);
                elves.insert(new);
            }
        }
        if !moved {
            dbg!(cycle);
            break;
        }
        let removed = order.remove(0);
        order.push(removed);
    }
    for (x, y) in &elves {
        let x = *x;
        let y = *y;
        miny = miny.min(y);
        maxy = maxy.max(y);
        minx = minx.min(x);
        maxx = maxx.max(x);
    }
    dbg!((maxx - minx + 1) * (maxy - miny + 1) - elves.len() as i32);
}
