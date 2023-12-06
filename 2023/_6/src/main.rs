use std::io::stdin;

fn main() {
    let lines: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    let times = &lines[0];
    let dis = &lines[1];
    let times: Vec<usize> = times.split(' ').flat_map(|t| t.parse::<usize>()).collect();
    let dis: Vec<usize> = dis.split(' ').flat_map(|t| t.parse::<usize>()).collect();
    let mut part1 = 1;
    for (&time, &dis) in times.iter().zip(dis.iter()) {
        let mut count = 0;
        for i in 0..time {
            if i * (time - i) > dis {
                count += 1;
            }
        }
        part1 *= count;
    }
    dbg!(part1);
    let time = format!("{}{}{}{}", times[0], times[1], times[2], times[3]);
    let time = time.parse::<usize>().unwrap();
    let dis = format!("{}{}{}{}", dis[0], dis[1], dis[2], dis[3]);
    let dis = dis.parse::<usize>().unwrap();
    let mut part2 = 0;
    for i in 0..time {
        if i * (time - i) > dis {
            part2 += 1;
        }
    }
    dbg!(part2);
}