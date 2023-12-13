use std::collections::HashMap;
use std::io::stdin;
use std::str::FromStr;

struct Mask([Option<bool>; 36]);

impl FromStr for Mask {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let mut buf: [Option<bool>; 36] = [None; 36];
        for (i, c) in s.chars().enumerate() {
            match c {
                '0' => buf[35 - i] = Some(false),
                '1' => buf[35 - i] = Some(true),
                _ => {}
            }
        }
        Ok(Mask(buf))
    }
}

enum Op {
    Mask(Mask),
    Mem(u64, u64),
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let mut iter = s.split(" = ");
        let left = iter.next().unwrap();
        let right = iter.next().unwrap();
        match left {
            "mask" => Ok(Self::Mask(Mask::from_str(right)?)),
            _ => {
                let mut iter = left.split("[");
                iter.next().unwrap();
                let num = iter.next().unwrap();
                let num = &num[0..num.len() - 1]; // Remove ']'
                let pos = num.parse().unwrap();
                let val = right.trim().parse().unwrap();
                Ok(Self::Mem(pos, val))
            }
        }
    }
}

fn _apply_mask_1(num: u64, mask: &Mask) -> u64 {
    let mut num = num;
    let mut m: u64 = 0xFFFFFFFFFFFFFFFE;
    for elem in mask.0.iter() {
        match elem {
            Some(false) => {
                num = num & m;
            }
            Some(true) => {
                num = num | (!m);
            }
            None => {}
        }
        m = m.wrapping_shl(1) | 1;
    }
    num
}

fn _part_1(operations: Vec<Op>) {
    let mut mask = Mask([None; 36]);
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for operation in operations {
        match operation {
            Op::Mask(m) => {
                mask = m;
            }
            Op::Mem(pos, val) => {
                memory.insert(pos, _apply_mask_1(val, &mask));
            }
        }
    }
    let total: u64 = memory.into_iter().fold(0, |total, (_k, v)| total + v);
    println!("{}", total);
}

fn apply_mask_2(pos: u64, mask: &Mask) -> Vec<u64> {
    let mut positions: Vec<u64> = Vec::new();
    positions.push(0);
    let mut m: u64 = 1;
    for elem in mask.0.iter() {
        match elem {
            Some(false) => {
                positions = positions.into_iter().map(|x| x + (pos & m)).collect();
            }
            Some(true) => {
                positions = positions.into_iter().map(|x| x + m).collect();
            }
            None => {
                positions.append(&mut positions.clone().into_iter().map(|x| x + m).collect());
            }
        }
        m <<= 1;
    }
    positions
}

fn part_2(operations: Vec<Op>) {
    let mut mask = Mask([None; 36]);
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for operation in operations {
        match operation {
            Op::Mask(m) => {
                mask = m;
            }
            Op::Mem(pos, val) => {
                for position in apply_mask_2(pos, &mask) {
                    memory.insert(position, val);
                }
            }
        }
    }
    let total: u64 = memory.into_iter().fold(0, |total, (_k, v)| total + v);
    println!("{}", total);
}

fn main() {
    let input = stdin();
    let mut operations = Vec::new();
    loop {
        let mut buf = String::new();
        let bytes = input.read_line(&mut buf).unwrap();
        if bytes == 0 {
            break;
        }
        operations.push(Op::from_str(buf.trim()).unwrap());
    }
    // part_1(operations);
    part_2(operations);
}
