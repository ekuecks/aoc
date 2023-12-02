use std::collections::HashMap;
use std::io::stdin;

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Clone)]
enum Entry {
    Number(f64),
    Expr(String, Operation, String),
    Humn,
}

impl Operation {
    fn apply(self, x: f64, y: f64) -> f64 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
            Self::Multiply => x * y,
            Self::Divide => x / y,
        }
    }
}

fn main() {
    let mut entries = HashMap::new();
    for line in stdin().lines() {
        let line = line.unwrap();
        let l = line.trim();
        let parts: Vec<_> = l.split(": ").collect();
        let name = parts[0].to_string();
        let expr: Vec<_> = parts[1].split(' ').collect();
        if expr.len() == 1 {
            if &name == "humn" {
                entries.insert(name, Entry::Humn);
            } else {
                entries.insert(name, Entry::Number(expr[0].parse::<f64>().unwrap()));
            }
        } else {
            assert_eq!(expr.len(), 3);
            let mut operation = match expr[1] {
                "+" => Operation::Add,
                "-" => Operation::Subtract,
                "*" => Operation::Multiply,
                "/" => Operation::Divide,
                s => panic!("Invalid expr '{s}'"),
            };
            if &name == "root" {
                operation = Operation::Subtract;
            }

            entries.insert(
                name,
                Entry::Expr(expr[0].to_string(), operation, expr[2].to_string()),
            );
        }
    }
    let goal = resolve("vlzj".to_string(), &mut entries);
    let mut n: u64 = 0;
    let mut m: u64 = 1;
    loop {
        let num = (m + n) as f64;
        let mut entries = entries.clone();
        entries.insert("humn".to_string(), Entry::Number(num));
        let diff = goal - resolve("rnsd".to_string(), &mut entries);
        if diff == 0.0 {
            dbg!(num);
            break;
        }
        if diff > 0.0 {
            n += m >> 1;
            m = 1;
        } else {
            m <<= 1;
        }
    }
}

fn resolve(name: String, entries: &mut HashMap<String, Entry>) -> f64 {
    let (left, operation, right) = match entries.get(&name) {
        Some(Entry::Number(num)) => return *num,
        Some(Entry::Expr(left, operation, right)) => (left.clone(), *operation, right.clone()),
        _ => panic!("Invalid key {name}"),
    };
    let left = resolve(left, entries);
    let right = resolve(right, entries);
    let ans = operation.apply(left, right);
    entries.insert(name, Entry::Number(ans));
    ans
}
