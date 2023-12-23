use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::stdin;

type Point = (usize, usize);

fn main() {
    let mut lines = stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|s| s.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut part1 = 0;
    calc_part1(&mut lines, 0, 1, &mut part1, 0);
    dbg!(part1);
    let part2 = calc_part2(&mut lines);
    dbg!(part2);
}

fn calc_part1(cs: &mut [Vec<char>], r: usize, c: usize, opt: &mut usize, sofar: usize) {
    if r == cs.len() - 1 {
        let o = *opt;
        *opt = o.max(sofar);
    }
    let prev = cs[r][c];
    cs[r][c] = '#';
    let adj = &[
        (r.wrapping_sub(1), c),
        (r + 1, c),
        (r, c.wrapping_sub(1)),
        (r, c + 1),
    ] as &[Point];
    let mut possibles = Vec::new();
    for &(mut y, mut x) in adj {
        if y >= cs.len() || x >= cs.len() {
            continue;
        }
        let mut v = cs[y][x];
        if v == '#' {
            continue;
        } else if v == '.' {
            possibles.push((y, x, sofar));
        } else {
            let mut sofar = sofar;
            'outer: loop {
                if v == 'v' {
                    y += 1;
                    sofar += 1;
                } else if v == '<' {
                    x -= 1;
                    sofar += 1;
                } else if v == '^' {
                    y -= 1;
                    sofar += 1;
                } else if v == '>' {
                    x += 1;
                    sofar += 1;
                } else if v == '#' {
                    break 'outer;
                } else {
                    possibles.push((y, x, sofar));
                    break 'outer;
                }
                v = cs[y][x];
            }
        }
    }
    for (y, x, sofar) in possibles {
        calc_part1(cs, y, x, opt, sofar + 1);
    }
    cs[r][c] = prev;
}

fn calc_part2(inputs: &mut [Vec<char>]) -> usize {
    let mut graph = HashMap::new();
    let mut nodes = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (r, row) in inputs.iter().enumerate() {
        for (c, &elem) in row.iter().enumerate() {
            if elem == '#' {
                continue;
            }
            if r == 0 {
                start = (r, c);
            }
            if r == inputs.len() - 1 {
                end = (r, c);
            }
            let adj = [
                (r.wrapping_sub(1), c),
                (r + 1, c),
                (r, c.wrapping_sub(1)),
                (r, c + 1),
            ];
            let mut possible = 0;
            for (r, c) in adj {
                if r >= inputs.len() || c >= inputs[0].len() {
                    continue;
                }
                if inputs[r][c] != '#' {
                    possible += 1;
                }
            }
            // Junctions, start, and end
            if possible != 2 {
                nodes.push((r, c));
            }
        }
    }
    for i in 0..nodes.len() {
        let (r, c) = nodes[i];
        nodes[i] = (inputs.len(), inputs.len());
        graph.insert((r, c), find_adj(inputs, r, c, &nodes));
        nodes[i] = (r, c);
    }
    let mut seen = HashSet::new();
    let mut opt = 0;
    optimize(&graph, start, end, &mut opt, &mut seen, 0);
    opt
}

fn find_adj(inputs: &[Vec<char>], r: usize, c: usize, possible: &[Point]) -> Vec<(Point, usize)> {
    let mut q = VecDeque::new();
    q.push_back(((r, c), (inputs.len(), inputs.len()), 0));
    let mut result = HashMap::new();
    while let Some(((r, c), (pr, pc), d)) = q.pop_front() {
        if possible.contains(&(r, c)) {
            result.insert((r, c), d);
            continue;
        }
        let adj = [
            (r.wrapping_sub(1), c),
            (r + 1, c),
            (r, c.wrapping_sub(1)),
            (r, c + 1),
        ];
        for (nr, nc) in adj {
            if nr >= inputs.len() || nc >= inputs[0].len() {
                continue;
            }
            if nr == pr && nc == pc {
                continue;
            }
            if inputs[r][c] != '#' {
                q.push_back(((nr, nc), (r, c), d + 1));
            }
        }
    }
    result.into_iter().collect()
}

fn optimize(
    graph: &HashMap<Point, Vec<(Point, usize)>>,
    cur: Point,
    end: Point,
    opt: &mut usize,
    seen: &mut HashSet<Point>,
    sofar: usize,
) {
    if cur == end {
        let o = *opt;
        *opt = o.max(sofar);
        return;
    }
    let adjs = graph.get(&cur).unwrap();
    for &(adj, dist) in adjs {
        if seen.contains(&adj) {
            continue;
        }
        seen.insert(adj);
        optimize(graph, adj, end, opt, seen, sofar + dist);
        seen.remove(&adj);
    }
}
