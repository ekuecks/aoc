use std::collections::HashMap;

fn main() {
    let mut numbers = HashMap::new();
    numbers.insert(6, 1);
    numbers.insert(19, 2);
    numbers.insert(0, 3);
    numbers.insert(5, 4);
    numbers.insert(7, 5);
    numbers.insert(13, 6);
    let mut next = 1;
    for i in 7..30_000_000 {
        if numbers.contains_key(&next) {
            let diff = i - numbers.get(&next).unwrap().clone();
            numbers.insert(next, i);
            next = diff;
        } else {
            numbers.insert(next, i);
            next = 0;
        }
        println!("{}", next);
    }
}
