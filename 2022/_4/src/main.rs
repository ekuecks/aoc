use std::io::stdin;

fn get_range(s: &str) -> (usize, usize) {
    let split: Vec<_> = s.split('-').collect();
    (split[0].parse().unwrap(), split[1].parse().unwrap())
}

fn main() {
    let mut count = 0;
    for line in stdin().lines() {
        let line = line.unwrap();
        let split: Vec<_> = line.split(',').collect();
        let a = get_range(split[0]);
        let b = get_range(split[1]);
        if (a.0 >= b.0 && a.0 <= b.1) || (a.1 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.0 <= a.1) || (b.1 >= a.0 && b.1 <= a.1) {
            count += 1;
        } 
    }
    dbg!(count);
}
