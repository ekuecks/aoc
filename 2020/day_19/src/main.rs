use std::collections::HashMap;
use std::io;

enum Rule {
    Char(char),
    Rules(Vec<u64>),
    Or((Vec<u64>, Vec<u64>)),
}

fn main() {
    let input = io::stdin();
    let mut rules = HashMap::new();
    let mut parsing_rules = true;
    let mut total = 0;
    loop {
        let mut buf = String::new();
        input.read_line(&mut buf).unwrap();
        if buf.is_empty() {
            break;
        }
        let buf = buf.trim();
        if buf.is_empty() {
            parsing_rules = false;
            rules.insert(8, Rule::Or((vec![42], vec![42, 8])));
            rules.insert(11, Rule::Or((vec![42, 31], vec![42, 11, 31])));
            continue;
        }
        if parsing_rules {
            let mut iter = buf.split(":");
            let id = iter.next().unwrap().parse::<u64>().unwrap();
            let rest = iter.next().unwrap().trim();
            if rest.contains("|") {
                let mut iter = rest.split("|");
                let left = iter.next().unwrap().trim();
                let right = iter.next().unwrap().trim();
                let left = {
                    let rules = left.split(" ").map(|s| s.parse().unwrap()).collect();
                    rules
                };
                let right = {
                    let rules = right.split(" ").map(|s| s.parse().unwrap()).collect();
                    rules
                };
                rules.insert(id, Rule::Or((left, right)));
            } else if rest.contains("\"") {
                rules.insert(id, Rule::Char(rest.chars().nth(1).unwrap()));
            } else {
                let inner = rest.split(" ").map(|s| s.parse().unwrap()).collect();
                rules.insert(id, Rule::Rules(inner));
            }
        } else {
            let results = matches(buf, &rules, 0);
            for result in results {
                if result.is_empty() {
                    total += 1;
                    break;
                }
            }
        }
    }
    println!("{}", total);
}

fn matches<'a>(s: &'a str, rules: &HashMap<u64, Rule>, id: u64) -> Vec<&'a str> {
    match rules.get(&id).unwrap() {
        Rule::Char(c) => match s.chars().nth(0) {
            Some(c1) => {
                if *c == c1 {
                    vec![&s[1..s.len()]]
                } else {
                    vec![]
                }
            }
            None => vec![],
        },
        Rule::Rules(inner) => {
            let mut remaining = vec![s];
            let mut next_remaining;
            for rule in inner {
                next_remaining = Vec::new();
                let mut all_failed = true;
                for r in &remaining {
                    let mut next_results = matches(r, rules, *rule);
                    if !next_results.is_empty() {
                        all_failed = false;
                    }
                    next_remaining.append(&mut next_results);
                }
                if all_failed {
                    return vec![];
                }
                remaining = next_remaining;
            }
            remaining
        }
        Rule::Or((left, right)) => {
            let mut remaining = vec![s];
            let mut next_remaining;
            for rule in left {
                next_remaining = Vec::new();
                let mut all_failed = true;
                for r in &remaining {
                    let mut next_results = matches(r, rules, *rule);
                    if !next_results.is_empty() {
                        all_failed = false;
                    }
                    next_remaining.append(&mut next_results);
                }
                if all_failed {
                    remaining = Vec::new();
                    break;
                }
                remaining = next_remaining;
            }
            let mut r_remaining = vec![s];
            let mut next_remaining;
            for rule in right {
                next_remaining = Vec::new();
                let mut all_failed = true;
                for r in &r_remaining {
                    let mut next_results = matches(r, rules, *rule);
                    if !next_results.is_empty() {
                        all_failed = false;
                    }
                    next_remaining.append(&mut next_results);
                }
                if all_failed {
                    r_remaining = Vec::new();
                    break;
                }
                r_remaining = next_remaining;
            }
            remaining.append(&mut r_remaining);
            remaining
        }
    }
}
