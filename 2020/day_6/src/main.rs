use std::collections::HashSet;
use std::io::stdin;

fn process_group(group: &[String]) -> usize {
    let sets: Vec<_> = group
        .iter()
        .map(|person| person.chars().collect::<HashSet<char>>())
        .collect();
    let mut set: HashSet<char> = sets[0].clone();
    for other in sets[1..sets.len()].into_iter() {
        set = set.intersection(other).cloned().collect();
    }
    set.len()
}

fn main() {
    let input = stdin();
    let mut group = Vec::new();
    let mut count = 0;
    loop {
        let mut line = String::new();
        match input.read_line(&mut line) {
            Ok(0) => {
                count += process_group(&group);
                break;
            }
            Ok(_) => {}
            e => panic!("Failed to read input: {:?}", e),
        }
        if line.trim().is_empty() {
            count += process_group(&group);
            group = Vec::new();
        } else {
            group.push(line.trim().to_owned());
        }
    }
    println!("{}", count);
}
