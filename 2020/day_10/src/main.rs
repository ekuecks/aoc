use std::collections::HashMap;
use std::io::stdin;

fn main() {
    let input = stdin();
    let mut jolts = Vec::new();
    loop {
        let mut buf = String::new();
        input.read_line(&mut buf).unwrap();
        if buf.is_empty() {
            break;
        }
        jolts.push(buf.trim().parse::<u64>().unwrap());
    }
    jolts.sort();
    let mut memo = HashMap::new();
    println!("{}", part_two(&jolts, 0, &mut memo));
}

fn _part_one(jolts: Vec<u64>) {
    let mut ones = 0;
    let mut threes = 1;
    let mut current = 0;
    for jolt in jolts {
        if jolt - current == 1 {
            ones += 1;
        } else if jolt - current == 3 {
            threes += 1;
        }
        current = jolt;
    }
    println!("{}", ones * threes);
}

fn part_two(jolts: &[u64], current: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if let Some(count) = memo.get(&current) {
        return *count;
    }
    if jolts.len() == 1 {
        return 1;
    } else if jolts.len() == 0 {
        return 0;
    }
    let mut count = 0;
    for (i, jolt) in jolts.iter().enumerate() {
        if jolt - current <= 3 {
            count += part_two(&jolts[i + 1..jolts.len()], *jolt, memo);
        } else {
            break;
        }
    }
    memo.insert(current, count);
    count
}
