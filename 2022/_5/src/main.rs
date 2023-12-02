use std::io::stdin;

fn main() {
    let mut stacks = vec![
        vec!['R', 'N', 'F', 'V', 'L', 'J', 'S', 'M'],
        vec!['P', 'N', 'D', 'Z', 'F', 'J', 'W', 'H'],
        vec!['W', 'R', 'C', 'D', 'G'],
        vec!['N', 'B', 'S'],
        vec!['M', 'Z', 'W', 'P', 'C', 'B', 'F', 'N'],
        vec!['P', 'R', 'M', 'W'],
        vec!['R', 'T', 'N', 'G', 'L', 'S', 'W'],
        vec!['Q', 'T', 'H', 'F', 'N', 'B', 'V'],
        vec!['L', 'M', 'H', 'Z', 'N', 'F'],
    ];
    for line in stdin().lines() {
        let line = line.unwrap();
        if !line.contains("move") {
            continue;
        }
        let parts: Vec<_> = line.split(" ").collect();
        let num: usize = parts[1].parse::<usize>().unwrap();
        let from: usize = parts[3].parse::<usize>().unwrap() - 1;
        let to: usize = parts[5].parse::<usize>().unwrap() - 1;
        let mut temp = Vec::new();
        for _ in 0..num {
            let elem = stacks[from].pop().unwrap();
            temp.push(elem);
        }
        while let Some(elem) = temp.pop() {
            stacks[to].push(elem);
        }
    }
    for stack in stacks {
        if stack.is_empty() {
            println!("Empty");
            continue;
        }
        print!("{}", stack[stack.len() - 1]);
    }
}
