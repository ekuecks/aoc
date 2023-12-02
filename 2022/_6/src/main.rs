use std::io::stdin;
use std::collections::VecDeque;
use std::collections::HashSet;

fn main() {
    let line: Vec<char> = stdin().lines().next().unwrap().unwrap().chars().collect();
    let mut queue = VecDeque::new();
    for (i, c) in line.into_iter().enumerate() {
        queue.push_back(c);
        if queue.len() < 14 {
            continue;
        }
        if queue.len() > 14 {
            queue.pop_front().unwrap();
        }
        if queue.iter().collect::<HashSet<_>>().len() == 14 {
            dbg!(i + 1);
            break;
        }
    }
}
