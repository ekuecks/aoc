use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::stdin;

fn check_heap(heap: &BinaryHeap<(isize, isize)>) -> bool {
    let value = heap.peek().unwrap().0;
    heap.iter().all(|t| t.0 == value)
}

fn check(state: &str) -> bool {
    let s: Vec<_> = state.chars().collect();
    s[s.len() - 1] == 'Z'
}

fn main() {
    let lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    let instructions: Vec<char> = lines[0].trim().chars().collect();
    let mut directions = HashMap::new();
    let mut states = Vec::new();
    for line in lines.iter().skip(2) {
        let (start, rest) = line.trim().split_once(" = ").unwrap();
        let s: Vec<_> = start.chars().collect();
        if s[s.len() - 1] == 'A' {
            states.push(start.to_string());
        }
        let (left, right) = rest.trim().split_once(", ").unwrap();
        let left = left[1..].to_string();
        let right = right[0..3].to_string();
        directions.insert(start.to_string(), (left, right));
    }
    let mut part1 = 0;
    let mut state = "AAA".to_string();
    while state != *"ZZZ" {
        let dirs = directions.get(&state).unwrap();
        let &inst = &instructions[part1 % instructions.len()];
        if inst == 'L' {
            state = dirs.0.clone();
        } else {
            state = dirs.1.clone();
        }
        part1 += 1;
    }
    dbg!(part1);
    let mut cycles = Vec::new();
    let mut terms = Vec::new();
    for mut state in states {
        let mut memo: HashMap<String, HashSet<usize>> = HashMap::new();
        let mut i = 0;
        memo.insert(state.clone(), HashSet::from([i]));
        loop {
            let dirs = directions.get(&state).unwrap();
            let &inst = &instructions[i % instructions.len()];
            if inst == 'L' {
                state = dirs.0.clone();
            } else {
                state = dirs.1.clone();
            }
            i += 1;
            if memo
                .entry(state.clone())
                .or_default()
                .contains(&(i % instructions.len()))
            {
                break;
            }
            memo.get_mut(&state).unwrap().insert(i % instructions.len());
        }
        let repeat = state.clone();
        let cycle_offset = i;
        let mut cycle_length = 0;
        loop {
            let dirs = directions.get(&state).unwrap();
            let &inst = &instructions[(cycle_offset + cycle_length) % instructions.len()];
            if inst == 'L' {
                state = dirs.0.clone();
            } else {
                state = dirs.1.clone();
            }
            cycle_length += 1;
            if check(&state) {
                terms.push(cycle_offset + cycle_length);
            }
            if state == repeat && cycle_length % instructions.len() == 0 {
                break;
            }
        }
        cycles.push((cycle_offset, cycle_length));
    }
    // let mut heap = BinaryHeap::new();
    // for i in 0..cycles.len() {
    //     heap.push((-(terms[i] as isize), -(cycles[i].1 as isize)));
    // }
    // while !check_heap(&heap) {
    //     let (v, incr) = heap.pop().unwrap();
    //     heap.push((v + incr, incr));
    // }
    // let part2 = -heap.peek().unwrap().0;
    // dbg!(part2);
    for i in 0..terms.len() {
        let offset = terms[i];
        let cycle = cycles[i].1;
        // Because this is always 0, then the answer is the LCM
        assert_eq!(offset % cycle, 0);
    }
    let mut part2 = 1;
    for cycle in cycles {
        part2 = num::integer::lcm(part2, cycle.1);
    }
    dbg!(part2);
}
