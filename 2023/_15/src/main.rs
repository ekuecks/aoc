use std::collections::HashMap;
use std::io::stdin;

fn main() {
    let lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    let mut part1 = 0;
    let mut map: HashMap<usize, Vec<(String, usize)>> = HashMap::new();
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        let parts: Vec<_> = line.split(',').collect();
        for part in parts {
            let mut h = 0;
            for c in part.chars() {
                h += (c as u8) as usize;
                h *= 17;
                h %= 256;
            }
            part1 += h;

            if let Some((l, _r)) = part.split_once('-') {
                let mut h = 0;
                for c in l.chars() {
                    h += (c as u8) as usize;
                    h *= 17;
                    h %= 256;
                }
                let l = l.to_string();
                map.entry(h).or_default().retain(|(k, _)| k != &l);
            } else if let Some((l, r)) = part.split_once('=') {
                let mut h = 0;
                for c in l.chars() {
                    h += (c as u8) as usize;
                    h *= 17;
                    h %= 256;
                }
                let r = r.parse::<usize>().unwrap();
                let e = map.entry(h).or_default();
                let l = l.to_string();
                let mut found = false;
                for elem in e.iter_mut() {
                    if elem.0 == l {
                        elem.1 = r;
                        found = true;
                    }
                }
                if !found {
                    map.entry(h).or_default().push((l.to_string(), r));
                }
            } else {
                unreachable!()
            }
        }
    }
    let mut part2 = 0;
    for (b, values) in map {
        for (i, v) in values.into_iter().enumerate() {
            part2 += (1 + b) * (1 + i) * v.1;
        }
    }
    dbg!(part1);
    dbg!(part2);
}
