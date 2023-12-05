use std::io::stdin;
use std::str::FromStr;
use std::ops::Range;

#[allow(dead_code)]
struct Mapping {
    from: String,
    to: String,
    maps: Vec<Map>,
}

struct Map {
    source: Range<usize>,
    dest: Range<usize>
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dest, rest) = s.split_once(' ').unwrap();
        let dest = dest.parse::<usize>().unwrap();
        let (source, rest) = rest.split_once(' ').unwrap();
        let source = source.parse::<usize>().unwrap();
        let len: usize = rest.parse().unwrap();
        Ok(Map {
            source: (source..(source+len)),
            dest: (dest..(dest+len)),
        })
    }
}

fn main() {
    let mut part1 = 9999999999;
    let lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    let (_, seeds) = lines[0].trim().split_once(": ").unwrap();
    let initial_seeds: Vec<_> = seeds.split(' ').map(|s| s.parse::<usize>().unwrap()).collect();
    let mut i = 2;
    let mut mappings = Vec::new();
    while i < lines.len() {
        let line = lines[i].trim();
        let (from, rest) = line.split_once("-to-").unwrap();
        let (to, _) = rest.split_once(' ').unwrap();
        i += 1;
        let mut maps = Vec::new();
        while i < lines.len() && !lines[i].trim().is_empty() {
            let line = lines[i].trim();
            let map = Map::from_str(line).unwrap();
            maps.push(map);
            i += 1;
        }
        mappings.push(Mapping {
            from: from.to_string(),
            to: to.to_string(),
            maps,
        });
        i += 1;
    }
    for seed in &initial_seeds {
        let mut pos = *seed;
        for mapping in &mappings {
            for map in &mapping.maps {
                if map.source.contains(&pos) {
                    pos = map.dest.start + (pos - map.source.start);
                    break;
                }
            }
        }
        part1 = part1.min(pos);
    }
    dbg!(part1);
    let initial_seeds: Vec<_> = initial_seeds.chunks(2).map(|l| l[0]..(l[0]+l[1])).collect();
    let mut i = 0;
    let mappings: Vec<_> = mappings.into_iter().rev().collect();
    'outer: loop {
        let mut pos = i;
        for mapping in &mappings {
            for map in &mapping.maps {
                if map.dest.contains(&pos) {
                    pos = map.source.start + (pos - map.dest.start);
                    break;
                }
            }
        }
        for r in &initial_seeds {
            if r.contains(&pos) {
                let part2 = i;
                dbg!(part2);
                break 'outer;
            }
        }
        i += 1;
    }
}