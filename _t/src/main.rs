use std::collections::{HashSet, HashMap, VecDeque, BinaryHeap as MaxHeap};
use std::io::stdin;
use std::str::FromStr;
use std::cmp::{Ordering, PartialEq, Eq};

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

fn dfs_adj(inputs: &[S], idx: usize) -> Vec<usize> {
    Vec::new()
}

fn dfs(inputs: &[S], idx: usize) {
    for adj in dfs_adj(inputs, idx) {
        dfs(inputs, adj);
    }
}

fn bfs_adj(inputs: &[S], ids: usize) -> Vec<usize> {
    Vec::new()
}

fn bfs(inputs: Vec<S>) {
    let start = 0;
    let mut q = VecDeque::new();
    q.push_back(start);
    while let Some(idx) = q.pop_front() {
        for adj in bfs_adj(&inputs, idx) {
            q.push_back(idx);
        }
    }
}

#[derive(PartialEq, Eq)]
struct HeapElem {
    dist: usize,
    idx: usize,
}

impl PartialOrd for HeapElem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapElem {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reversed
        self.dist.cmp(&other.dist).reverse()
    }
}

fn dijkstra_adj(inputs: &[S], idx: usize, dist: usize) -> Vec<HeapElem> {
    Vec::new()
}

fn dijkstra(inputs: Vec<S>) {
    let mut q = MaxHeap::new();
    q.push(HeapElem { idx: 0, dist: 0 });
    while let Some(HeapElem { idx, dist }) = q.pop() {
        for elem in dijkstra_adj(&inputs, idx, dist) {
            q.push(elem)
        }
    }
}