use std::collections::HashMap;
use std::collections::HashSet;
use std::io::stdin;
use std::collections::VecDeque;

#[derive(Debug)]
struct RawRoom {
    adj: Vec<String>,
    flow: u64,
}

struct Room {
    adj: Vec<usize>,
    flow: u64,
}

struct PortalRoom {
    adj: Vec<(usize, usize)>,
    flow: u64,
}

fn bfs(rooms: &[Room], start: usize) -> Vec<(usize, usize)> {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let mut ans = Vec::new();
    queue.push_back((start, 0));
    while let Some((cur, len)) = queue.pop_front() {
        let room = rooms.get(cur).unwrap();
        if room.flow > 0 && cur != start {
            ans.push((cur, len));
        }
        for &adj in &room.adj {
            if !seen.contains(&adj) {
                seen.insert(adj);
                queue.push_back((adj, len + 1));
            }
        }
    }
    ans
}

fn main() {
    let mut raw_graph: HashMap<String, RawRoom> = HashMap::new();
    for line in stdin().lines() {
        let line = line.unwrap();
        let l = line.trim();
        let segments: Vec<_> = l.split(' ').collect();
        let name = segments[1].to_string();
        let flow: u64 = segments[4][5..]
            .trim_end_matches(|c| c == ';')
            .parse()
            .unwrap();
        let adjacents: Vec<String> = segments[9..]
            .iter()
            .map(|s| s.trim_end_matches(|c| c == ',').to_string())
            .collect();
        raw_graph.insert(
            name,
            RawRoom {
                adj: adjacents,
                flow,
            },
        );
    }
    let mut remap = HashMap::new();
    let mut names = Vec::new();
    for (i, name) in raw_graph.keys().enumerate() {
        remap.insert(name.clone(), i);
        names.push(name.clone());
    }
    let mut graph = Vec::new();
    for name in names {
        let raw = raw_graph.get(&name).unwrap();
        let new_adj = raw
            .adj
            .iter()
            .map(|name| *remap.get(name).unwrap())
            .collect();
        graph.push(Room {
            adj: new_adj,
            flow: raw.flow,
        });
    }
    let mut portal_graph = Vec::new();
    for index in 0..graph.len() {
        let new_adjs = bfs(&graph, index);
        let flow = graph.get(index).unwrap().flow;
        portal_graph.push(PortalRoom {
            adj: new_adjs,
            flow,
        })
    }
    // println!("{:?}", graph);
    let start = "AA".to_string();
    let mut seen = HashMap::new();
    // println!("{}", dfs(&start, &raw_graph, 0, 29, &mut seen));
    let max_flow = graph.iter().fold(0, |accum, room| accum + room.flow);
    println!(
        "{}",
        dual_dfs(
            *remap.get(&start).unwrap(),
            0,
            *remap.get(&start).unwrap(),
            0,
            &portal_graph,
            0,
            25,
            0,
            &mut seen,
            max_flow,
        )
    );
}

fn dual_dfs(
    cur1: usize,
    delay1: usize,
    cur2: usize,
    delay2: usize,
    graph: &[PortalRoom],
    flow: u64,
    left: usize,
    mut on: u64,
    seen: &mut HashMap<(usize, usize, usize, u64), u64>,
    max_flow: u64,
) -> u64 {
    if left == 0 {
        return 0;
    }
    if flow == max_flow {
        return flow * left as u64;
    }
    let memo_key = if cur1 < cur2 {
        (cur1, cur2, left, on)
    } else {
        (cur2, cur1, left, on)
    };
    if let Some(&cached) = seen.get(&memo_key) {
        return cached;
    }
    let room1 = graph.get(cur1).unwrap();
    let room2 = graph.get(cur2).unwrap();
    let mut optimal = 0;
    if delay1 == 0 && room1.flow > 0 && on & (1 << cur1) == 0 {
        on |= 1 << cur1;
        if delay2 == 0 && room2.flow > 0 && on & (1 << cur2) == 0 {
            on |= 1 << cur2;
            optimal = optimal.max(
                flow + room1.flow
                    + room2.flow
                    + dual_dfs(
                        cur1,
                        0,
                        cur2,
                        0,
                        graph,
                        flow + room1.flow + room2.flow,
                        left - 1,
                        on,
                        seen,
                        max_flow,
                    ),
            );
            on ^= 1 << cur2;
        }
        if delay2 == 0 && (room2.flow == 0 || on & (1 << cur2) == 1) {
            for &(adj2, delay2) in &room2.adj {
                if on & (1 << adj2) == 1 {
                    continue;
                }
                optimal = optimal.max(
                    flow + room1.flow
                        + dual_dfs(cur1, 0, adj2, delay2 - 1, graph, flow + room1.flow, left - 1, on, seen, max_flow),
                );
            }
        }
        on ^= 1 << cur1;
    }
    if delay1 == 0 && (room1.flow == 0 || on & (1 << cur1) == 1) {
        for &(adj1, dist1) in &room1.adj {
            if on & (1 << adj1) == 1 {
                continue;
            }
            if delay2 == 0 && room2.flow > 0 && on & (1 << cur2) == 0 {
                on |= 1 << cur2;
                optimal = optimal.max(
                    flow + room2.flow
                        + dual_dfs(adj1, dist1 - 1, cur2, 0, graph, flow + room2.flow, left - 1, on, seen, max_flow),
                );
                on ^= 1 << cur2;
            }
            if delay2 == 0 && (room2.flow == 0 || on & (1 << cur2) == 1) {
                for &(adj2, dist2) in &room2.adj {
                    if on & (1 << adj2) == 1 {
                        continue;
                    }
                    let min_delay = dist1.min(dist2);
                    let left = if min_delay > left {
                        0
                    } else {
                        left - min_delay
                    };
                    optimal = optimal.max(flow + dual_dfs(adj1, delay1 - min_delay, adj2, delay2 - min_delay, graph, flow, left, on, seen, max_flow));
                }
            }
        }
    }
    seen.insert(memo_key, optimal);
    optimal
}

fn dfs(
    cur: &String,
    graph: &HashMap<String, RawRoom>,
    on: HashSet<String>,
    flow: u64,
    left: usize,
    seen: &mut HashMap<(String, u64, usize), u64>,
) -> u64 {
    let key = (cur.clone(), flow, left);
    if let Some(cached) = seen.get(&key) {
        return *cached;
    }
    if left == 0 {
        return 0;
    }
    let room = graph.get(cur).unwrap();
    let mut optimal = 0;
    if room.flow > 0 && !on.contains(cur) {
        let mut new_on = on.clone();
        new_on.insert(cur.clone());
        optimal = flow + room.flow + dfs(cur, graph, new_on, flow + room.flow, left - 1, seen);
    }
    for adj in &room.adj {
        optimal = optimal.max(flow + dfs(adj, graph, on.clone(), flow, left - 1, seen));
    }
    seen.insert(key, optimal);
    optimal
}
