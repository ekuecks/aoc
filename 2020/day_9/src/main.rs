use std::collections::{HashMap, VecDeque};
use std::io;

fn main() {
    let input = io::stdin();
    let mut nums = Vec::new();
    loop {
        let mut buf = String::new();
        input.read_line(&mut buf).unwrap();
        if buf.is_empty() {
            break;
        }
        let num: u64 = buf.trim().parse().unwrap();
        nums.push(num);
    }
    for i in 0..nums.len() {
        if let Some(sum) = find_sequence(&nums, 36845998, i) {
            println!("{}", sum);
        }
    }
}

fn find_sequence(nums: &[u64], target: u64, mut index: usize) -> Option<u64> {
    let mut sum = 0;
    let starting_index = index.clone();
    while sum < target && index < nums.len() {
        sum += nums[index];
        index += 1;
    }
    if sum == target {
        println!("{:?}", &nums[starting_index..index]);
        let mut min = nums[starting_index];
        let mut max = nums[starting_index];
        for i in starting_index..index {
            if nums[i] < min {
                min = nums[i];
            }
            if nums[i] > max {
                max = nums[i];
            }
        }
        Some(max + min)
    } else {
        None
    }
}

fn _part_1() {
    let input = io::stdin();
    let mut current = VecDeque::new();
    let mut memo = HashMap::new();
    loop {
        let mut buf = String::new();
        input.read_line(&mut buf).unwrap();
        let num: u64 = buf.trim().parse().unwrap();
        if current.len() < 25 {
            current.push_back(num);
            let count = match memo.get(&num) {
                Some(count) => *count,
                None => 0,
            };
            memo.insert(num, count + 1);
            continue;
        }
        let mut pair_found = false;
        for &existing in current.iter() {
            if existing > num {
                continue;
            }
            let to_find = num - existing;
            match memo.get(&to_find) {
                Some(&count) => {
                    if to_find == existing && count == 1 {
                        continue;
                    }
                    pair_found = true;
                    break;
                }
                None => {}
            }
        }
        if !pair_found {
            println!("Unmatched: {}", num);
            break;
        }
        current.push_back(num);
        let count = match memo.get(&num) {
            Some(count) => *count,
            None => 0,
        };
        memo.insert(num, count + 1);
        let removed = current.pop_front().unwrap();
        if let Some(&x) = memo.get(&removed) {
            if x > 1 {
                memo.insert(removed, x - 1);
            } else {
                memo.remove(&removed);
            }
        }
    }
}
