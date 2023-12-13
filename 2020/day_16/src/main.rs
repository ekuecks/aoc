use std::collections::{HashMap, HashSet};
use std::io::stdin;
use std::mem::drop;

enum ParseState {
    Rules,
    Your,
    Nearby,
}

fn parse_rule(line: String) -> (String, Vec<(usize, usize)>) {
    let mut iter = line.trim().split(": ");
    let name = iter.next().unwrap().to_owned();
    let right = iter.next().unwrap();
    let right_iter = right.split(" or ");
    let mut ranges = Vec::new();
    for elem in right_iter {
        let mut elem_iter = elem.split("-");
        let lower: usize = elem_iter.next().unwrap().parse().unwrap();
        let upper: usize = elem_iter.next().unwrap().parse().unwrap();
        ranges.push((lower, upper));
    }
    (name, ranges)
}

fn parse_ticket(line: String) -> Vec<usize> {
    line.trim()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn main() {
    let input = stdin();
    let mut map = HashMap::new();
    let mut indices = Vec::new();
    let mut state = ParseState::Rules;
    let mut nearby = Vec::new();
    let mut my = Vec::new();

    loop {
        let mut buf = String::new();
        input.read_line(&mut buf).unwrap();
        if buf.is_empty() {
            break;
        }
        if buf.trim().is_empty() {
            match state {
                ParseState::Rules => {
                    state = ParseState::Your;
                }
                ParseState::Your => {
                    state = ParseState::Nearby;
                }
                _ => {}
            }
            continue;
        }
        if buf.contains("ticket") {
            continue;
        }
        match state {
            ParseState::Rules => {
                let tup = parse_rule(buf);
                map.insert(tup.0, tup.1);
            }
            ParseState::Your => {
                my = parse_ticket(buf);
            }
            ParseState::Nearby => nearby.push(parse_ticket(buf)),
        }
    }
    let all_names: HashSet<String> = map.iter().map(|(k, _)| k.clone()).collect();
    nearby = nearby
        .into_iter()
        .filter(|ticket| {
            for entry in ticket {
                let mut valid = false;
                for (_, ranges) in map.iter() {
                    for (low, high) in ranges {
                        if entry >= low && entry <= high {
                            valid = true;
                            break;
                        }
                    }
                }
                if !valid {
                    return false;
                }
            }
            return true;
        })
        .collect();
    for _ in map.iter() {
        indices.push(all_names.clone());
    }
    for ticket in nearby.into_iter() {
        for (i, entry) in ticket.into_iter().enumerate() {
            let all_potential = indices.get_mut(i).unwrap();
            for potential in all_potential.clone() {
                let ranges = map.get(&potential).unwrap();
                let mut valid = false;
                for (low, high) in ranges {
                    if entry >= *low && entry <= *high {
                        valid = true;
                        break;
                    }
                }
                if !valid {
                    all_potential.remove(&potential);
                }
                if all_potential.len() == 1 {
                    let copy = all_potential.clone();
                    drop(all_potential);
                    let remaining = copy.iter().next().unwrap().clone();
                    for j in 0..indices.len() {
                        if j == i {
                            continue;
                        }
                        let potentials = indices.get_mut(j).unwrap();
                        potentials.remove(&remaining);
                    }
                    break;
                }
            }
        }
    }
    let mut processed = HashSet::new();
    loop {
        let mut any_found = false;
        for i in 0..indices.len() {
            if processed.contains(&i) {
                continue;
            }
            let potentials = indices.get(i).unwrap();
            if potentials.len() == 1 {
                let remaining = potentials.iter().next().unwrap().clone();
                for j in 0..indices.len() {
                    if i == j {
                        continue;
                    }
                    let other = indices.get_mut(j).unwrap();
                    other.remove(&remaining);
                }
                processed.insert(i);
                any_found = true;
            }
        }
        if !any_found {
            break;
        }
    }
    let mut ans = 1;
    let mut num = 0;
    for (i, value) in my.into_iter().enumerate() {
        let field = indices[i].iter().next().unwrap();
        if field.starts_with("departure") {
            ans *= value;
            num += 1;
        }
    }
    println!("{:?} {}", ans, num);
}
