use std::collections::{HashSet, VecDeque};
use std::io::stdin;

fn main() {
    let input = stdin();
    let mut p1 = VecDeque::new();
    let mut p2 = VecDeque::new();
    let mut player = false;
    loop {
        let mut buf = String::new();
        let bytes = input.read_line(&mut buf).unwrap();
        if bytes == 0 {
            break;
        }
        if buf.trim().is_empty() {
            player = true;
            continue;
        }
        if buf.contains("layer") {
            continue;
        }
        if player {
            p2.push_back(buf.trim().parse::<usize>().unwrap());
        } else {
            p1.push_back(buf.trim().parse::<usize>().unwrap());
        }
    }
    println!("{:?}\n{:?}", p1, p2);
    let mut seen = HashSet::new();
    let result = play(p1, p2, &mut seen);
    println!("{:?}", result);
    println!("Hello, world!");
}

fn play(
    mut p1: VecDeque<usize>,
    mut p2: VecDeque<usize>,
    seen: &mut HashSet<(VecDeque<usize>, VecDeque<usize>)>,
) -> (usize, bool) {
    while !p1.is_empty() && !p2.is_empty() {
        let p1_a = p1.pop_front().unwrap();
        let p2_a = p2.pop_front().unwrap();
        let key = (p1.clone(), p2.clone());
        if seen.contains(&key) {
            return (score(&p1), true);
        }
        seen.insert(key);
        if p1_a <= p1.len() && p2_a <= p2.len() {
            let p1_wins = winner(
                p1.iter().take(p1_a).cloned().collect(),
                p2.iter().take(p2_a).cloned().collect(),
                &mut HashSet::new(),
            );
            if p1_wins {
                p1.push_back(p1_a);
                p1.push_back(p2_a);
            } else {
                p2.push_back(p2_a);
                p2.push_back(p1_a);
            }
        } else if p1_a > p2_a {
            p1.push_back(p1_a);
            p1.push_back(p2_a);
        } else {
            p2.push_back(p2_a);
            p2.push_back(p1_a);
        }
    }
    let answer = if p1.is_empty() {
        (score(&p2), false)
    } else {
        (score(&p1), true)
    };
    answer
}

fn score(v: &VecDeque<usize>) -> usize {
    let mut result = 0;
    let len = v.len();
    for (i, c) in v.iter().enumerate() {
        result += *c * (len - i as usize);
    }
    result
}

fn winner(
    mut p1: VecDeque<usize>,
    mut p2: VecDeque<usize>,
    seen: &mut HashSet<(VecDeque<usize>, VecDeque<usize>)>,
) -> bool {
    println!("winner: {:?}, {:?}", p1, p2);
    let p1_m = *p1.iter().max().unwrap();
    if p1_m >= p1.len() + p2.len() && p1_m > *p2.iter().max().unwrap() {
        println!("short-circuit");
        return true;
    }
    let key = (p1.clone(), p2.clone());
    if seen.contains(&key) {
        return true;
    }
    seen.insert(key);
    while !p1.is_empty() && !p2.is_empty() {
        let p1_m = *p1.iter().max().unwrap();
        if p1_m >= p1.len() + p2.len() && p1_m > *p2.iter().max().unwrap() {
            return true;
        }
        let p1_a = p1.pop_front().unwrap();
        let p2_a = p2.pop_front().unwrap();
        let key = (p1.clone(), p2.clone());
        if seen.contains(&key) {
            let result = true;
            return result;
        }
        seen.insert(key);
        if p1_a <= p1.len() && p2_a <= p2.len() {
            let p1_wins = winner(
                p1.iter().take(p1_a).cloned().collect(),
                p2.iter().take(p2_a).cloned().collect(),
                &mut HashSet::new(),
            );
            if p1_wins {
                p1.push_back(p1_a);
                p1.push_back(p2_a);
            } else {
                p2.push_back(p2_a);
                p2.push_back(p1_a);
            }
        } else if p1_a > p2_a {
            p1.push_back(p1_a);
            p1.push_back(p2_a);
        } else {
            p2.push_back(p2_a);
            p2.push_back(p1_a);
        }
    }
    p2.is_empty()
}
