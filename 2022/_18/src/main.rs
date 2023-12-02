use std::io::stdin;
use std::collections::HashSet;
use std::collections::VecDeque;

fn bfs(x: i32, y: i32, z: i32, cubes: &HashSet<(i32, i32, i32)>) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back((x, y, z));
    let mut seen = HashSet::new();
    seen.insert((x, y, z));
    while let Some((x, y, z)) = queue.pop_front() {
        if (x, y, z) == (0, 0, 0) {
            return true;
        }
        for adj in [
            (x + 1, y, z),
            (x - 1, y, z),
            (x, y + 1, z),
            (x, y - 1, z),
            (x, y, z + 1),
            (x, y, z - 1),
        ] {
            if adj.0 > 20 || adj.0 < -1 || adj.1 > 20 || adj.1 < -1 || adj.2 > 20 || adj.2 < -1 {
                continue;
            }
            if !seen.contains(&adj) && !cubes.contains(&adj) {
                seen.insert(adj);
                queue.push_back(adj);
            }
        }
    }
    false
}

fn main() {
    let mut cubes = HashSet::new();
    let mut minx = 100000;
    let mut maxx = -1000000;
    let mut miny = 100000;
    let mut maxy = -1000000;
    let mut minz = 1000000;
    let mut maxz = -1000000;
    for line in stdin().lines() {
        let line = line.unwrap();
        let l = line.trim();
        let parts: Vec<_> = l.split(',').collect();
        let (x, y, z) = (parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap(), parts[2].parse::<i32>().unwrap());
        minx = minx.min(x);
        maxx = maxx.max(x);
        miny = miny.min(y);
        maxy = maxy.max(y);
        minz = minz.min(z);
        maxz = maxz.max(z);
        cubes.insert((x, y, z));
    }
    let mut grid = vec![
        vec![
            vec![
                false; (maxz - minz + 1) as usize
            ]; (maxy - miny + 1) as usize
        ]; (maxx - minx + 1) as usize
    ];
    for &(x, y, z) in &cubes {
        let x = (x - minx) as usize;
        let y = (y - miny) as usize;
        let z = (z - minz) as usize;
        grid[x][y][z] = true;
    }
    let mut ans = 0;
    dbg!(minx);
    dbg!(maxx);
    dbg!(miny);
    dbg!(maxy);
    dbg!(minz);
    dbg!(maxz);
    for x in minx..=maxx {
        dbg!(x);
        for y in miny..=maxy {
            for z in minz..=maxz {
                if cubes.contains(&(x, y, z)) {
                    continue;
                }
                if !bfs(x, y, z, &cubes) {
                    cubes.insert((x, y, z));
                }
            }
        }
    }
    for (x, y, z) in cubes.clone() {
        for adj in [
            (x + 1, y, z),
            (x - 1, y, z),
            (x, y + 1, z),
            (x, y - 1, z),
            (x, y, z + 1),
            (x, y, z - 1),
        ] {
            if !cubes.contains(&adj) {
                ans += 1;
            }
        }

    }
    dbg!(ans);

    println!("Hello, world!");
}
