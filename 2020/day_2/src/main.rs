use std::convert::TryFrom;
use std::io::stdin;

#[derive(Debug)]
struct Policy {
    first: usize,
    second: usize,
    letter: char,
}

impl Policy {
    fn valid(&self, password: &str) -> bool {
        let first = password.len() > self.first
            && password.chars().into_iter().nth(self.first).unwrap() == self.letter;
        let second = password.len() > self.second
            && password.chars().into_iter().nth(self.second).unwrap() == self.letter;
        let result = first != second;
        if result {
            println!("{} passes {:?}", password, self);
        } else {
            println!("{} fails {:?}", password, self);
        }
        result
    }
}

impl TryFrom<String> for Policy {
    type Error = String;

    fn try_from(policy_str: String) -> Result<Self, Self::Error> {
        let parts: Vec<String> = policy_str
            .split(" ")
            .into_iter()
            .map(ToString::to_string)
            .collect();
        if parts.len() != 2 {
            panic!("Invalid policy: {}", policy_str);
        }
        let range_parts = parts[0]
            .clone()
            .split("-")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        if range_parts.len() != 2 {
            panic!("Invalid policy: {}", policy_str);
        }
        let letter = parts[1].trim().chars().collect::<Vec<char>>()[0];
        Ok(Self {
            first: range_parts[0],
            second: range_parts[1],
            letter,
        })
    }
}

struct Input {
    policy: Policy,
    password: String,
}

impl TryFrom<String> for Input {
    type Error = String;

    fn try_from(line: String) -> Result<Self, Self::Error> {
        let parts: Vec<String> = line
            .trim()
            .split(":")
            .into_iter()
            .map(ToString::to_string)
            .collect();
        if parts.len() != 2 {
            panic!("Invalid line: {}", line);
        }
        Ok(Input {
            policy: Policy::try_from(parts[0].clone())?,
            password: parts[1].clone(),
        })
    }
}

fn main() {
    let input = stdin();
    let mut inputs = Vec::new();
    loop {
        let mut buf = String::new();
        match input.read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => inputs.push(Input::try_from(buf).unwrap()),
            e => panic!("Failed to parse: {:?}", e),
        }
    }
    let valid = inputs.into_iter().fold(0, |count, i| {
        if i.policy.valid(&i.password) {
            count + 1
        } else {
            count
        }
    });
    println!("{}", valid);
}
