use std::collections::HashMap;
use std::io::stdin;
use std::str::FromStr;

enum E {
    R(isize),
    L(isize),
    U(isize),
    D(isize),
}

impl FromStr for E {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s[1..].parse::<isize>().unwrap();
        Ok(match &s[0..1] {
            "R" => Self::R(len),
            "L" => Self::L(len),
            "U" => Self::U(len),
            "D" => Self::D(len),
            _ => unreachable!(),
        })
    }
}

fn main() {
    let mut lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    if lines[lines.len() - 1].trim().is_empty() {
        lines.pop();
    }
    let inputs: Vec<Vec<_>> = lines
        .into_iter()
        .map(|s| {
            s.trim()
                .split(',')
                .map(|s| E::from_str(s.trim()).unwrap())
                .collect()
        })
        .collect();
    let mut a = HashMap::new();
    let mut b = HashMap::new();
    let mut minx = 0;
    let mut miny = 0;
    let mut maxx = 0;
    let mut maxy = 0;
    for (inputs, grid) in [(&inputs[0], &mut a), (&inputs[1], &mut b)] {
        let mut point = (0, 0);
        let mut steps = 0;
        for input in inputs {
            let points: Vec<_> = match input {
                E::R(len) => (1..=*len).map(|i| (point.0 + i, point.1)).collect(),
                E::L(len) => (1..=*len).map(|i| (point.0 - i, point.1)).collect(),
                E::U(len) => (1..=*len).map(|i| (point.0, point.1 + i)).collect(),
                E::D(len) => (1..=*len).map(|i| (point.0, point.1 - i)).collect(),
            };
            point = points[points.len() - 1];
            for point in points {
                minx = minx.min(point.0);
                miny = miny.min(point.1);
                maxx = maxx.max(point.0);
                maxy = maxy.max(point.1);
                steps += 1;
                grid.entry(point).or_insert(steps);
            }
        }
    }
    let mut part2 = 99999999999isize;
    for (p, d) in a {
        if let Some(d2) = b.get(&p) {
            part2 = part2.min(d + d2);
        }
    }
    dbg!(part2);
}
