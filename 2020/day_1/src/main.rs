use std::collections::HashSet;
use std::io::stdin;

fn search(nums: &[u64], nums_set: &HashSet<u64>, sum: u64) -> Option<u64> {
    for num in nums {
        if num > &sum {
            continue;
        }
        let pair = sum - num;
        if nums_set.contains(&pair) {
            return Some(num * pair);
        }
    }
    None
}

fn main() {
    let input = stdin();
    let mut nums = Vec::new();
    loop {
        let mut buf = String::new();
        match input.read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => nums.push(buf.trim().parse::<u64>().unwrap()),
            _ => break,
        }
    }
    let nums_set = nums.clone().into_iter().collect::<HashSet<u64>>();
    for num in nums.iter() {
        if num > &2020 {
            continue;
        }
        if let Some(prod) = search(&nums, &nums_set, 2020 - num) {
            println!("{}", prod * num);
        }
    }
}
