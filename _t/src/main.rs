use std::collections::{HashSet, HashMap, VecDeque, BinaryHeap};
use std::io::stdin;
use std::str::FromStr;

struct S {}

impl FromStr for S {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(S{

        })
    }
}

fn main() {
    let mut lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    if lines[lines.len() - 1].trim().is_empty() {
        lines.pop();
    }
    let mut inputs: Vec<_> = lines.into_iter().map(|s| S::from_str(s.trim()).unwrap()).collect();
}
