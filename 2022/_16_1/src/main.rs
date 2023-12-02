use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::stdin;

#[derive(Debug)]
struct RawRoom {
    adj: Vec<String>,
    flow: u64,
}

#[derive(Debug)]
struct Room {
    adj: Vec<usize>,
    flow: u64,
}

#[derive(Debug)]
struct PortalRoom {
    adj: Vec<(usize, u64)>,
    flow: u64,
}

fn bfs(rooms: &[Room], start: usize) -> Vec<(usize, u64)> {
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
    // println!("{:?}", portal_graph);
    // println!("{:?}", graph);
    let start = "AA".to_string();
    // println!("{}", dfs(&start, &raw_graph, 0, 29, &mut seen));
    let max_flow = graph.iter().fold(0, |accum, room| accum + room.flow);
    dbg!(max_flow);
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
            max_flow,
            true,
            0,
            0,
        )
    );
}

fn is_on(on: u64, index: usize) -> bool {
    (on & (1 << index)) != 0
}

fn turn_on(on: u64, index: usize) -> u64 {
    assert!(!is_on(on, index));
    let on = on | (1 << index);
    assert!(is_on(on, index));
    on
}

fn turn_off(on: u64, index: usize) -> u64 {
    assert!(is_on(on, index));
    let on = on ^ (1 << index);
    assert!(!is_on(on, index));
    on
}

fn dual_dfs(
    cur1: usize,
    delay1: u64,
    cur2: usize,
    delay2: u64,
    graph: &[PortalRoom],
    flow: u64,
    left: u64,
    mut on: u64,
    max_flow: u64,
    start: bool,
    max: u64,
    cur: u64,
) -> u64 {
    if left == 0 {
        return max;
    }
    assert!(delay1.min(delay2) == 0);
    assert!(flow <= max_flow);
    if cur + (max_flow - flow) * left <= max {
        return max;
    }
    if flow == max_flow {
        return max;
    }
    let room1 = graph.get(cur1).unwrap();
    let room2 = graph.get(cur2).unwrap();
    let mut optimal = max;
    // turn on room1
    if delay1 == 0 && room1.flow > 0 && !is_on(on, cur1) {
        on = turn_on(on, cur1);
        // turn on room1 and room2
        if delay2 == 0 && room2.flow > 0 && !is_on(on, cur2) {
            on = turn_on(on, cur2);
            let cur = cur + (room1.flow + room2.flow) * left;
            optimal = optimal.max(cur);
            optimal = optimal.max(dual_dfs(
                cur1,
                0,
                cur2,
                0,
                graph,
                flow + room1.flow + room2.flow,
                left - 1,
                on,
                max_flow,
                false,
                optimal,
                cur,
            ));
            return optimal;
        }
        // move to new room2
        let mut found = false;
        if delay2 == 0 && (room2.flow == 0 || is_on(on, cur2)) {
            for &(adj2, dist2) in &room2.adj {
                if is_on(on, adj2) {
                    continue;
                }
                found = true;
                let cur = cur + room1.flow * left;
                optimal = optimal.max(cur);
                optimal = optimal.max(dual_dfs(
                    cur1,
                    0,
                    adj2,
                    dist2 - 1,
                    graph,
                    flow + room1.flow,
                    left - 1,
                    on,
                    max_flow,
                    false,
                    optimal,
                    cur,
                ));
            }
            if !found {
                let cur = cur + room1.flow * left;
                optimal = optimal.max(cur);
                optimal = optimal.max(dual_dfs(
                    cur1,
                    0,
                    cur2,
                    0,
                    graph,
                    flow + room1.flow,
                    left - 1,
                    on,
                    max_flow,
                    false,
                    optimal,
                    cur,
                ));
            }
        }
        // still travelling to room2
        if delay2 != 0 {
            let cur = cur + room1.flow * left;
            optimal = optimal.max(cur);
            optimal = optimal.max(dual_dfs(
                cur1,
                0,
                cur2,
                delay2 - 1,
                graph,
                flow + room1.flow,
                left - 1,
                on,
                max_flow,
                false,
                optimal,
                cur,
            ));
        }
        return optimal;
    }
    // move from room1
    if delay1 == 0 {
        let mut found = false;
        for &(adj1, dist1) in &room1.adj {
            // skip room thats already on
            if is_on(on, adj1) {
                continue;
            }
            found = true;
            if delay2 == 0 && room2.flow > 0 && !is_on(on, cur2) {
                on = turn_on(on, cur2);
            let cur = cur + room2.flow * left;
            optimal = optimal.max(cur);
                optimal = optimal.max(dual_dfs(
                    adj1,
                    dist1 - 1,
                    cur2,
                    0,
                    graph,
                    flow + room2.flow,
                    left - 1,
                    on,
                    max_flow,
                    false,
                    optimal,
                    cur,
                ));
                on = turn_off(on, cur2);
            }
            // move from room 2
            if delay2 == 0 && (room2.flow == 0 || is_on(on, cur2)) {
                for &(adj2, dist2) in &room2.adj {
                    // skip room thats already on
                    if is_on(on, adj2) {
                        continue;
                    }
                    let min_delay = dist1.min(dist2);
                    let actual = min_delay.min(left);
                    let new_left = left - actual;
                    optimal = optimal.max(dual_dfs(
                        adj1,
                        dist1 - min_delay,
                        adj2,
                        dist2 - min_delay,
                        graph,
                        flow,
                        new_left,
                        on,
                        max_flow,
                        false,
                        optimal,
                        cur,
                    ));
                }
            // still moving to room 2
            } else {
                let min_delay = dist1.min(delay2);
                let actual = min_delay.min(left);
                let new_left = left - actual;
                optimal = optimal.max(dual_dfs(
                    adj1,
                    dist1 - min_delay,
                    cur2,
                    delay2 - min_delay,
                    graph,
                    flow,
                    new_left,
                    on,
                    max_flow,
                    false,
                    optimal,
                    cur,
                ));
            }
        }
        if !found {
            // last one
            assert!(delay2 == 0 && room2.flow > 0 && !is_on(on, cur2));
            on = turn_on(on, cur2);
            let cur = cur + room2.flow * left;
            optimal = optimal.max(cur);
            optimal = optimal.max(dual_dfs(
                cur1,
                0,
                cur2,
                0,
                graph,
                flow + room2.flow,
                left - 1,
                on,
                max_flow,
                false,
                optimal,
                cur,
            ));
            on = turn_off(on, cur2);
        }
    }
    // still moving to room1
    if delay1 != 0 {
        // turn on room2
        assert!(delay2 == 0);
        if room2.flow > 0 && !is_on(on, cur2) {
            on = turn_on(on, cur2);
            let cur = cur + room2.flow * left;
            optimal = optimal.max(cur);
            optimal = optimal.max(dual_dfs(
                cur1,
                delay1 - 1,
                cur2,
                0,
                graph,
                flow + room2.flow,
                left - 1,
                on,
                max_flow,
                false,
                optimal,
                cur,
            ));
            on = turn_off(on, cur2);
        }
        // move to new room2
        if room2.flow == 0 || is_on(on, cur2) {
            for &(adj2, dist2) in &room2.adj {
                if is_on(on, adj2) {
                    continue;
                }
                let min_delay = delay1.min(dist2);
                let actual = min_delay.min(left);
                let new_left = left - actual;
                optimal = optimal.max(dual_dfs(
                    cur1,
                    delay1 - min_delay,
                    adj2,
                    dist2 - min_delay,
                    graph,
                    flow,
                    new_left,
                    on,
                    max_flow,
                    false,
                    optimal,
                    cur,
                ));
            }
        }
        if start {
            println!("DONE WITH ITERATION");
        }
    }
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
